use super::graph::*;
use std::io::prelude::*;
use std::io;

#[derive(PartialEq, Clone, Debug)]
pub struct NodeData {
    name: String,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Action {
    ProgramInput,
    ProgramStart,
    Quit,
}

#[derive(PartialEq, Clone, Debug)]
pub enum UserAction {
    UnknownInput { bytes: Vec<u8> },
    ProgramInput { bytes: Vec<u8> },
    ProgramStart,
    ModeChange { name: String },
    Quit,
}

#[derive(PartialEq, Clone, Debug)]
pub struct EdgeData {
    action: Option<Action>,
    codes: Vec<u8>,
    default: bool,
}
impl Default for EdgeData {
    fn default() -> EdgeData {
        EdgeData { action: None, codes: vec![], default: false }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct ModalKeyHandler {
    current_node: NodeIndex,
    graph: Graph<NodeData, EdgeData>,
    pub actions_queue: Vec<UserAction>,
    match_buf: Vec<u8>,
}

impl ModalKeyHandler {
    pub fn new(first_node_index: NodeIndex, graph: Graph<NodeData, EdgeData>) -> ModalKeyHandler {
        ModalKeyHandler {
            current_node: first_node_index,
            graph: graph,
            actions_queue: vec![],
            match_buf: vec![],
        }
    }

    pub fn new_with_graph() -> ModalKeyHandler {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let w = graph.add_node(NodeData { name: "welcome".to_string() });
        let c = graph.add_node(NodeData { name: "command".to_string() });
        let p = graph.add_node(NodeData { name: "program".to_string() });

        graph.add_edge(w, c, EdgeData { default: true, ..Default::default()});

        graph.add_edge(c, p, EdgeData { action: Some(Action::ProgramStart), codes: vec![99], ..Default::default()});
        graph.add_edge(c, c, EdgeData { action: Some(Action::Quit), codes: vec![113], ..Default::default()});

        graph.add_edge(p, p, EdgeData { action: Some(Action::ProgramInput), codes: vec![2,2], default: true, ..Default::default()});
        graph.add_edge(p, c, EdgeData { codes: vec![2,100], ..Default::default()});

        ModalKeyHandler::new(w, graph)
    }

    pub fn mode_name(&self) -> &String {
        &self.graph.nodes[self.current_node].data.name
    }
}

impl Write for ModalKeyHandler {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for byte in buf.iter() {
            let mut match_buf = self.match_buf.clone();
            match_buf.push(*byte);
            let match_buf = match_buf;

            let edge_indexes = self.graph.nodes[self.current_node].edge_indexes.clone();
            let mut next_node = self.current_node;
            let mut action: Option<Action> = None;

            if let Some(i) = edge_indexes.iter().find(|i| self.graph.edges[**i].data.codes == match_buf) {
                trace!("exact match");
                self.match_buf.clear();
                action = self.graph.edges[*i].data.action;
                next_node = self.graph.edges[*i].target;
            }
            else if let Some(i) = edge_indexes.iter().find(|i| self.graph.edges[**i].data.codes.starts_with(&match_buf)) {
                trace!("partial match");
                self.match_buf = match_buf.clone();
            }
            else if let Some(i) = edge_indexes.iter().find(|i| self.graph.edges[**i].data.default ) {
                trace!("default edge");
                self.match_buf.clear();
                action = self.graph.edges[*i].data.action;
                next_node = self.graph.edges[*i].target;
            }
            else {
                trace!("unknown input {:?}", match_buf);
                self.match_buf.clear();
                self.actions_queue.push(UserAction::UnknownInput { bytes: match_buf.clone() });
            }

            if let Some(a) = action {
                let user_action = match a {
                    Action::ProgramInput => UserAction::ProgramInput { bytes: match_buf },
                    Action::ProgramStart => UserAction::ProgramStart,
                    Action::Quit => UserAction::Quit,
                };
                self.actions_queue.push(user_action);
            }

            if self.current_node != next_node {
                self.current_node = next_node;
                self.actions_queue.push(UserAction::ModeChange {
                    name: self.graph.nodes[self.current_node].data.name.clone()
                });
            }
        }

        Ok(1)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

mod tests {
    use super::*;
    use super::super::graph::*;
    use std::io::prelude::*;
    use std::io;
    use std::sync::mpsc::*;

    #[test]
    fn when_edge_matches_it_follows_it() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        graph.add_edge(n0_index, n1_index, EdgeData { codes: vec![97], ..Default::default()});
        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write(&[97]);
        assert_eq!(h.current_node, n1_index);
    }

    #[test]
    fn when_edge_matches_it_follows_it_to_same_node() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        graph.add_edge(n0_index, n0_index, EdgeData { codes: vec![97], ..Default::default()});

        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write(&[97]);
        assert_eq!(h.current_node, n0_index);
    }

    #[test]
    fn when_no_edge_matches_it_sends_unknown_msg_to_channel() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });

        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write(&[97]);
        assert_eq!(h.actions_queue.first(), Some(&UserAction::UnknownInput { bytes: vec![97] } ));
    }

    #[test]
    fn when_default_edge_exists_it_follow_it() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        graph.add_edge(n0_index, n1_index, EdgeData { default: true, ..Default::default()});

        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write(&[97]);
        assert_eq!(h.current_node, n1_index);
    }

    #[test]
    fn when_edge_matches_it_follows_it_even_if_default_edge_exists() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        let n2_index = graph.add_node(NodeData { name: "n2".to_string() });
        graph.add_edge(n0_index, n1_index, EdgeData { codes: vec![97], ..Default::default()});
        graph.add_edge(n0_index, n2_index, EdgeData { default: true, ..Default::default()});

        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write(&[97]);
        assert_eq!(h.current_node, n1_index);
    }

    #[test]
    fn when_matching_edge_has_a_blah_action_it_blah_blahs() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        let n2_index = graph.add_node(NodeData { name: "n2".to_string() });
        graph.add_edge(n0_index, n1_index, EdgeData { codes: vec![97], ..Default::default()});
        graph.add_edge(n0_index, n2_index, EdgeData { default: true, ..Default::default()});

        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write(&[97]);
        assert_eq!(h.current_node, n1_index);
    }

    #[test]
    fn when_edge_with_multiple_bytes_matches_it_follows_it() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        graph.add_edge(n0_index, n1_index, EdgeData { codes: vec![97, 98], ..Default::default()});
        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write(&[97, 98]);
        assert_eq!(h.current_node, n1_index);
    }

    #[test]
    fn when_edge_with_multiple_bytes_matches_it_follows_it_across_writes() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        graph.add_edge(n0_index, n1_index, EdgeData { codes: vec![97, 98], ..Default::default()});
        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write(&[97]);
        h.write(&[98]);
        assert_eq!(h.current_node, n1_index);
    }

    #[test]
    fn it_can_follow_edges_with_multibyte_codes_one_after_the_other() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        let n2_index = graph.add_node(NodeData { name: "n2".to_string() });
        graph.add_edge(n0_index, n1_index, EdgeData { codes: vec![97, 98], ..Default::default()});
        graph.add_edge(n1_index, n2_index, EdgeData { codes: vec![99, 100], ..Default::default()});
        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write(&[97, 98, 99, 100]);
        assert_eq!(h.current_node, n2_index);
    }

    #[test]
    fn it_can_follow_multiple_edges_in_a_single_write() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        let n2_index = graph.add_node(NodeData { name: "n2".to_string() });
        graph.add_edge(n0_index, n1_index, EdgeData { codes: vec![97], ..Default::default()});
        graph.add_edge(n1_index, n2_index, EdgeData { codes: vec![98], ..Default::default()});
        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write(&[97, 98]);
        assert_eq!(h.current_node, n2_index);
    }

    #[test]
    fn when_matching_edge_has_a_program_input_action_it_adds_to_queue() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        graph.add_edge(n0_index, n0_index, EdgeData { action: Some(Action::ProgramInput), default: true, ..Default::default()});
        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write(&[97]);
        assert_eq!(h.actions_queue.first(), Some(&UserAction::ProgramInput { bytes: vec![97] } ));
    }

    #[test]
    fn when_matching_edge_has_a_program_start_action_it_adds_to_queue() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        graph.add_edge(n0_index, n0_index, EdgeData { action: Some(Action::ProgramStart), default: true, ..Default::default()});
        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write(&[97]);
        assert_eq!(h.actions_queue.first(), Some(&UserAction::ProgramStart));
    }

    #[test]
    fn when_matching_edge_has_a_quit_action_it_adds_to_queue() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        graph.add_edge(n0_index, n0_index, EdgeData { action: Some(Action::Quit), default: true, ..Default::default()});
        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write(&[97]);
        assert_eq!(h.actions_queue.first(), Some(&UserAction::Quit));
    }
}
