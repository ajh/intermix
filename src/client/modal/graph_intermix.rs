use super::graph::*;
use super::modal_key_handler::*;

pub fn graph() -> Graph<NodeData, EdgeData> {
    let mut graph: Graph<NodeData, EdgeData> = Graph::new();

    let w = graph.add_node(NodeData { name: "welcome".to_string() });
    let c = graph.add_node(NodeData { name: "command".to_string() });
    let p = graph.add_node(NodeData { name: "program".to_string() });

    graph.add_edge(w, c, EdgeData { default: true, ..Default::default()});

    graph.add_edge(c, p, EdgeData { action: Some(ActionType::ProgramStart), codes: vec![99], ..Default::default()});
    graph.add_edge(c, c, EdgeData { action: Some(ActionType::Quit), codes: vec![113], ..Default::default()});

    graph.add_edge(p, p, EdgeData { action: Some(ActionType::ProgramInput), codes: vec![2,2], default: true, ..Default::default()});
    graph.add_edge(p, c, EdgeData { codes: vec![2,100], ..Default::default()});

    graph
}
