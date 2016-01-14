use super::graph::*;
use std::io::prelude::*;
use std::io;
use std::sync::mpsc::*;

pub struct NodeData {
    name: String,
}

pub enum Action {
    Foo,
    Bar
}

#[derive(PartialEq, Debug)]
pub enum UserAction {
    UnknownInput { bytes: Vec<u8> }
}

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

pub struct InputHandler {
    current_node: NodeIndex,
    graph: Graph<NodeData, EdgeData>,
    tx: Sender<UserAction>,
}

impl InputHandler {
    pub fn new(first_node_index: NodeIndex, graph: Graph<NodeData, EdgeData>, tx: Sender<UserAction>) -> InputHandler {
        InputHandler {
            current_node: first_node_index,
            graph: graph,
            tx: tx,
        }
    }
}

impl Write for InputHandler {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for byte in buf.iter() {
            let edge_indexes = self.graph.nodes[self.current_node].edge_indexes.clone();

            if let Some(i) = edge_indexes.iter().find(|i| self.graph.edges[**i].data.codes == vec![*byte] ) {
                self.current_node = self.graph.edges[*i].target;
            }
            else if let Some(i) = edge_indexes.iter().find(|i| self.graph.edges[**i].data.default ) {
                self.current_node = self.graph.edges[*i].target;
            }
            else {
                self.tx.send(UserAction::UnknownInput { bytes: vec![*byte] });
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
        graph.add_edge(n0_index, n1_index, EdgeData { action: None, codes: vec![97], ..Default::default()});
        let (tx, _) = channel();
        let mut h = InputHandler::new(n0_index, graph, tx);

        h.write(&[97]);
        assert_eq!(h.current_node, n1_index);
    }

    #[test]
    fn when_edge_matches_it_follows_it_to_same_node() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        graph.add_edge(n0_index, n0_index, EdgeData { action: None, codes: vec![97], ..Default::default()});

        let (tx, _) = channel();
        let mut h = InputHandler::new(n0_index, graph, tx);

        h.write(&[97]);
        assert_eq!(h.current_node, n0_index);
    }

    #[test]
    fn when_no_edge_matches_it_sends_unknown_msg_to_channel() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });

        let (tx, rx) = channel();
        let mut h = InputHandler::new(n0_index, graph, tx);

        h.write(&[97]);
        assert_eq!(rx.try_recv(), Result::Ok(UserAction::UnknownInput { bytes: vec![97] } ));
    }

    #[test]
    fn when_default_edge_exists_it_follow_it() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        graph.add_edge(n0_index, n1_index, EdgeData { action: None, default: true, ..Default::default()});

        let (tx, _) = channel();
        let mut h = InputHandler::new(n0_index, graph, tx);

        h.write(&[97]);
        assert_eq!(h.current_node, n1_index);
    }

    #[test]
    fn when_edge_matches_it_follows_it_even_if_default_edge_exists() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        let n2_index = graph.add_node(NodeData { name: "n2".to_string() });
        graph.add_edge(n0_index, n1_index, EdgeData { action: None, codes: vec![97], ..Default::default()});
        graph.add_edge(n0_index, n2_index, EdgeData { action: None, default: true, ..Default::default()});

        let (tx, _) = channel();
        let mut h = InputHandler::new(n0_index, graph, tx);

        h.write(&[97]);
        assert_eq!(h.current_node, n1_index);
    }

    #[test]
    fn when_matching_edge_has_a_blah_action_it_blah_blahs() {
        let mut graph: Graph<NodeData, EdgeData> = Graph::new();
        let n0_index = graph.add_node(NodeData { name: "n0".to_string() });
        let n1_index = graph.add_node(NodeData { name: "n1".to_string() });
        let n2_index = graph.add_node(NodeData { name: "n2".to_string() });
        graph.add_edge(n0_index, n1_index, EdgeData { action: None, codes: vec![97], ..Default::default()});
        graph.add_edge(n0_index, n2_index, EdgeData { action: None, default: true, ..Default::default()});

        let (tx, _) = channel();
        let mut h = InputHandler::new(n0_index, graph, tx);

        h.write(&[97]);
        assert_eq!(h.current_node, n1_index);
    }
    // it_sends_edge_action_to_channel

    // it_follows_edge_when_code_matches
    // it_follows_edge_when_default
}
