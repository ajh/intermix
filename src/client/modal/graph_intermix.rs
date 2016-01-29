use super::graph::*;
use super::modal_key_handler::*;

const CTRL_B: u8 = 2u8;

pub fn graph() -> Graph<NodeData, EdgeData> {
    let mut graph: Graph<NodeData, EdgeData> = Graph::new();

    let w = graph.add_node(NodeData { name: "welcome".to_string() });
    let c = graph.add_node(NodeData { name: "command".to_string() });
    let p = graph.add_node(NodeData { name: "program".to_string() });

    graph.add_edge(w, c, EdgeData { default: true, ..Default::default()});

    graph.add_edge(c, c, EdgeData { action: Some(ActionType::ProgramStart), codes: "c".to_string().into_bytes(), ..Default::default()});
    graph.add_edge(c, c, EdgeData { action: Some(ActionType::Quit), codes: "q".to_string().into_bytes(), ..Default::default()});
    graph.add_edge(c, p, EdgeData { action: Some(ActionType::ProgramFocus), codes: "i".to_string().into_bytes(), ..Default::default()});

    // ('up arrow', 'program_select_prev');
    // ('down arrow', 'program_select_next');
    // ('c', 'program_create_and_focus');
    // ('x', 'program_kill');

    graph.add_edge(p, p, EdgeData { action: Some(ActionType::ProgramInput), codes: vec![CTRL_B, CTRL_B], default: true, ..Default::default()});
    graph.add_edge(p, c, EdgeData { codes: escape("c".to_string().into_bytes(), CTRL_B), ..Default::default()});

    graph
}

fn escape(mut codes: Vec<u8>, escape_char: u8) -> Vec<u8> {
    codes.insert(0, escape_char);
    codes
}
