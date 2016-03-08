use super::graph::*;
use std::io::prelude::*;
use std::io;

#[derive(PartialEq, Clone, Debug)]
pub struct NodeData {
    pub name: String,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ActionType {
    ProgramFocus,
    ProgramInput,
    ProgramStart,
    ProgramSelectNext,
    ProgramSelectPrev,
    Quit,
}

#[derive(PartialEq, Clone, Debug)]
pub enum UserAction {
    UnknownInput {
        bytes: Vec<u8>,
    },
    ProgramInput {
        bytes: Vec<u8>,
    },
    ProgramStart,
    ProgramFocus,
    ProgramSelectNext,
    ProgramSelectPrev,
    ModeChange {
        name: String,
    },
    Quit,
}

#[derive(PartialEq, Clone, Debug)]
pub struct EdgeData {
    pub action: Option<ActionType>,
    pub codes: Vec<u8>,
    pub default: bool,
}

impl Default for EdgeData {
    fn default() -> EdgeData {
        EdgeData {
            action: None,
            codes: vec![],
            default: false,
        }
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
        let graph = super::graph_intermix::graph();
        ModalKeyHandler::new(0, graph)
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
            let mut next_node: Option<NodeIndex> = None;
            let mut action: Option<ActionType> = None;

            if let Some(i) = edge_indexes.iter()
                                         .find(|i| self.graph.edges[**i].data.codes == match_buf) {
                //trace!("exact match");
                self.match_buf.clear();
                action = self.graph.edges[*i].data.action;
                next_node = Some(self.graph.edges[*i].target);
            } else if let Some(_) = edge_indexes.iter().find(|i| {
                self.graph.edges[**i].data.codes.starts_with(&match_buf)
            }) {
                //trace!("partial match");
                self.match_buf = match_buf.clone();
            } else if let Some(i) = edge_indexes.iter().find(|i| self.graph.edges[**i].data.default) {
                //trace!("default edge");
                self.match_buf.clear();
                action = self.graph.edges[*i].data.action;
                next_node = Some(self.graph.edges[*i].target);
            } else {
                //trace!("unknown input {:?}", match_buf);
                self.match_buf.clear();
                self.actions_queue.push(UserAction::UnknownInput { bytes: match_buf.clone() });
            }

            if let Some(a) = action {
                let user_action = match a {
                    ActionType::ProgramInput => UserAction::ProgramInput { bytes: match_buf },
                    ActionType::ProgramStart => UserAction::ProgramStart,
                    ActionType::ProgramFocus => UserAction::ProgramFocus,
                    ActionType::ProgramSelectPrev => UserAction::ProgramSelectPrev,
                    ActionType::ProgramSelectNext => UserAction::ProgramSelectNext,
                    ActionType::Quit => UserAction::Quit,
                };
                self.actions_queue.push(user_action);
            }

            if let Some(i) = next_node {
                if i != self.current_node {
                    self.current_node = i;
                    self.actions_queue.push(UserAction::ModeChange {
                        name: self.graph.nodes[self.current_node].data.name.clone(),
                    });
                }
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::*;
    use super::super::graph::*;
    use std::io::prelude::*;

    #[test]
    fn when_edge_matches_it_follows_it() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        graph.add_edge(n0_index,
                       n1_index,
                       EdgeData { codes: "a".to_string().into_bytes(), ..Default::default() });
        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write("a".as_bytes()).unwrap();
        assert_eq!(h.current_node, n1_index);
    }

    #[test]
    fn when_edge_matches_it_follows_it_to_same_node() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        graph.add_edge(n0_index,
                       n0_index,
                       EdgeData { codes: "a".to_string().into_bytes(), ..Default::default() });

        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write("a".as_bytes()).unwrap();
        assert_eq!(h.current_node, n0_index);
    }

    #[test]
    fn when_no_edge_matches_it_sends_unknown_msg_to_channel() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });

        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write("a".as_bytes()).unwrap();
        assert_eq!(h.actions_queue.first(),
                   Some(&UserAction::UnknownInput { bytes: "a".to_string().into_bytes() }));
    }

    #[test]
    fn when_default_edge_exists_it_follow_it() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        graph.add_edge(n0_index,
                       n1_index,
                       EdgeData { default: true, ..Default::default() });

        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write("a".as_bytes()).unwrap();
        assert_eq!(h.current_node, n1_index);
    }

    #[test]
    fn when_edge_matches_it_follows_it_even_if_default_edge_exists() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        let n2_index = graph.add_node(NodeData { name: "n2".to_string() });
        graph.add_edge(n0_index,
                       n1_index,
                       EdgeData { codes: "a".to_string().into_bytes(), ..Default::default() });
        graph.add_edge(n0_index,
                       n2_index,
                       EdgeData { default: true, ..Default::default() });

        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write("a".as_bytes()).unwrap();
        assert_eq!(h.current_node, n1_index);
    }

    #[test]
    fn when_matching_edge_has_a_blah_action_it_blah_blahs() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        let n2_index = graph.add_node(NodeData { name: "n2".to_string() });
        graph.add_edge(n0_index,
                       n1_index,
                       EdgeData { codes: "a".to_string().into_bytes(), ..Default::default() });
        graph.add_edge(n0_index,
                       n2_index,
                       EdgeData { default: true, ..Default::default() });

        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write("a".as_bytes()).unwrap();
        assert_eq!(h.current_node, n1_index);
    }

    #[test]
    fn when_edge_with_multiple_bytes_matches_it_follows_it() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        graph.add_edge(n0_index,
                       n1_index,
                       EdgeData { codes: "ab".to_string().into_bytes(), ..Default::default() });
        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write("ab".as_bytes()).unwrap();
        assert_eq!(h.current_node, n1_index);
    }

    #[test]
    fn when_edge_with_multiple_bytes_matches_it_follows_it_across_writes() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        graph.add_edge(n0_index,
                       n1_index,
                       EdgeData { codes: "ab".to_string().into_bytes(), ..Default::default() });
        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write("a".as_bytes()).unwrap();
        h.write("b".as_bytes()).unwrap();
        assert_eq!(h.current_node, n1_index);
    }

    #[test]
    fn it_can_follow_edges_with_multibyte_codes_one_after_the_other() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        let n2_index = graph.add_node(NodeData { name: "n2".to_string() });
        graph.add_edge(n0_index,
                       n1_index,
                       EdgeData { codes: "ab".to_string().into_bytes(), ..Default::default() });
        graph.add_edge(n1_index,
                       n2_index,
                       EdgeData { codes: "cd".to_string().into_bytes(), ..Default::default() });
        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write("abcd".as_bytes()).unwrap();
        assert_eq!(h.current_node, n2_index);
    }

    #[test]
    fn it_can_follow_multiple_edges_in_a_single_write() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        let n2_index = graph.add_node(NodeData { name: "n2".to_string() });
        graph.add_edge(n0_index,
                       n1_index,
                       EdgeData { codes: "a".to_string().into_bytes(), ..Default::default() });
        graph.add_edge(n1_index,
                       n2_index,
                       EdgeData { codes: "b".to_string().into_bytes(), ..Default::default() });
        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write("ab".as_bytes()).unwrap();
        assert_eq!(h.current_node, n2_index);
    }

    #[test]
    fn when_matching_edge_has_a_program_input_action_it_adds_to_queue() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        graph.add_edge(n0_index,
                       n0_index,
                       EdgeData {
                           action: Some(ActionType::ProgramInput),
                           default: true,
                           ..Default::default()
                       });
        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write("a".as_bytes()).unwrap();
        assert_eq!(h.actions_queue.first(),
                   Some(&UserAction::ProgramInput { bytes: "a".to_string().into_bytes() }));
    }

    #[test]
    fn when_matching_edge_has_a_program_start_action_it_adds_to_queue() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        graph.add_edge(n0_index,
                       n0_index,
                       EdgeData {
                           action: Some(ActionType::ProgramStart),
                           default: true,
                           ..Default::default()
                       });
        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write("a".as_bytes()).unwrap();
        assert_eq!(h.actions_queue.first(), Some(&UserAction::ProgramStart));
    }

    #[test]
    fn when_matching_edge_has_a_quit_action_it_adds_to_queue() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        graph.add_edge(n0_index,
                       n0_index,
                       EdgeData {
                           action: Some(ActionType::Quit),
                           default: true,
                           ..Default::default()
                       });
        let mut h = ModalKeyHandler::new(n0_index, graph);

        h.write("a".as_bytes()).unwrap();
        assert_eq!(h.actions_queue.first(), Some(&UserAction::Quit));
    }
}
