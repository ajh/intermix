pub type NodeIndex = usize;
pub type EdgeIndex = usize;

pub struct Node<N> {
    pub data: N,
    pub edge_indexes: Vec<EdgeIndex>,
}

pub struct Edge<E> {
    pub data: E,
    pub target: NodeIndex,
}

pub struct Graph<N,E> {
    pub nodes: Vec<Node<N>>,
    pub edges: Vec<Edge<E>>,
}

impl<N,E> Graph<N,E> {
    pub fn new() -> Graph<N,E> {
        Graph { nodes: vec![], edges: vec![] }
    }

    pub fn add_node(&mut self, data: N) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(Node { data: data, edge_indexes: vec![]});
        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, data: E) {
        let index = self.edges.len();

        self.edges.push(Edge {
            target: target,
            data: data,
        });

        let mut s = &mut self.nodes[source];
        s.edge_indexes.push(index);
    }
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut graph: Graph<String, String> = Graph::new();

        let n0_index = graph.add_node("n0".to_string());
        let n1_index = graph.add_node("n1".to_string());

        graph.add_edge(n0_index, n1_index, "e1".to_string());
        graph.add_edge(n1_index, n0_index, "e2".to_string());

        assert_eq!(graph.nodes[n0_index].data, "n0".to_string());

        let next_node_index = graph.edges[
            graph.nodes[n0_index].edge_indexes.first().unwrap().clone()
        ].target;
        assert_eq!(graph.nodes[next_node_index].data, "n1".to_string());
    }
}
