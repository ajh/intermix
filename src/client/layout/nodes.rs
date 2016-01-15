use std::slice::Iter;
use vterm_sys;
use itertools::Itertools;
use super::{Node};

/// iteratates through nodes depth first
#[derive(Debug)]
pub struct Nodes<'a> {
    root: &'a Node,
    path: Vec<usize>,
    is_done: bool,
}

impl<'a> Nodes<'a> {
    pub fn new(root: &'a Node) -> Nodes<'a> {
        Nodes {
            root: root,
            path: vec![],
            is_done: false,
        }
    }
}

impl<'a> Nodes<'a> {
    fn find_node(&self) -> Option<&'a Node> {
        let mut node = self.root;

        for i in self.path.iter() {
            if let Some(n) = node.children.get(*i) {
                node = n;
            }
            else {
                return None;
            }
        }

        Some(node)
    }

    fn increment_path(&mut self) {
        self.path.push(0);

        while !self.path.is_empty() && self.find_node().is_none() {
            self.path.pop();
            if let Some(i) = self.path.pop() {
                self.path.push(i + 1);
            }
        }

        if self.path.is_empty() {
            self.is_done = true;
        }
    }
}

impl<'a> Iterator for Nodes<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<&'a Node> {
        if self.is_done {
            return None;
        }

        if let Some(n) = self.find_node() {
            self.increment_path();
            Some(n)
        }
        else {
            None
        }
    }
}

mod tests {
    use ::client::layout::*;

    #[test]
    fn it_iterates_with_root() {
        let root = Node::row(Default::default(), vec![]);
        let mut nodes = Nodes::new(&root);
        assert_eq!(nodes.next(), Some(&root));
        assert_eq!(nodes.next(), None);
    }

    #[test]
    fn it_iterates_root_and_child() {
        let root = Node::row(Default::default(), vec![
            Node::row(Default::default(), vec![]),
        ]);
        let mut nodes = Nodes::new(&root);
        assert_eq!(nodes.next(), Some(&root));
        assert_eq!(nodes.next(), root.children.first());
        assert_eq!(nodes.next(), None);
    }

    #[test]
    fn it_iterates_root_and_two_children() {
        let root = Node::row(Default::default(), vec![
            Node::row(Default::default(), vec![]),
            Node::row(Default::default(), vec![]),
        ]);
        let mut nodes = Nodes::new(&root);
        assert_eq!(nodes.next(), Some(&root));
        assert_eq!(nodes.next(), root.children.first());
        assert_eq!(nodes.next(), root.children.last());
        assert_eq!(nodes.next(), None);
    }

    #[test]
    fn it_iterates_root_with_two_children_the_first_of_which_has_a_child() {
        let root = Node::row(Default::default(), vec![
            Node::row(Default::default(), vec![
                Node::row(Default::default(), vec![]),
            ]),
            Node::row(Default::default(), vec![]),
        ]);
        let mut nodes = Nodes::new(&root);
        assert_eq!(nodes.next(), Some(&root));
        assert_eq!(nodes.next(), root.children.first());
        assert_eq!(nodes.next(), root.children.first().unwrap().children.first());
        assert_eq!(nodes.next(), root.children.last());
        assert_eq!(nodes.next(), None);
    }

    #[test]
    fn it_iterates_left_handed_tree() {
        let root = Node::row(Default::default(), vec![
            Node::row(Default::default(), vec![
                Node::row(Default::default(), vec![
                    Node::row(Default::default(), vec![
                        Node::row(Default::default(), vec![]),
                    ]),
                ]),
            ]),
        ]);
        let mut nodes = Nodes::new(&root);
        assert_eq!(nodes.next(), Some(&root));
        assert_eq!(nodes.next(), root.children.first());
        assert_eq!(nodes.next(), root.children.first().unwrap().children.first());
        assert_eq!(nodes.next(), root.children.first().unwrap().children.first().unwrap().children.first());
        assert_eq!(nodes.next(), root.children.first().unwrap().children.first().unwrap().children.first().unwrap().children.first());
        assert_eq!(nodes.next(), None);
    }
}
