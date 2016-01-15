use std::slice::Iter;
use vterm_sys;
use itertools::Itertools;
use super::{Node};

/// iteratates through nodes depth first
#[derive(Debug)]
pub struct NodesMut<'a> {
    root: &'a mut Node,
    path: Vec<usize>,
    is_done: bool,
}

impl<'a> NodesMut<'a> {
    pub fn new(root: &'a mut Node) -> NodesMut<'a> {
        NodesMut {
            root: root,
            path: vec![],
            is_done: false,
        }
    }
}

impl<'a> NodesMut<'a> {
    //fn find_node(&mut self) -> Option<&mut Node> {
        //let mut foo: Vec<Vec<usize>> = vec![vec![1,2], vec![6,9]];

        //{
            //let bar = &mut foo;
            //let baz = bar.get_mut(0).unwrap();
            //baz.push(3);
        //}

        //println!("{:?}", foo);

        //let mut path_iter = self.path.iter();
        //if let Some(i) = path_iter.next() {
            //let mut node = self.root.children.get_mut(*i).unwrap();
            //for i in path_iter {
                //if let Some(n) = node.children.get_mut(*i) {
                    //node = n;
                //}
                //else {
                    //return None;
                //}
            //}
            //return Some(node);
        //}
        //else {
            //return None;
        //};

        //return None;
    //}

    //fn increment_path(&mut self) {
        //self.path.push(0);

        //while !self.path.is_empty() && self.find_node().is_none() {
            //self.path.pop();
            //if let Some(i) = self.path.pop() {
                //self.path.push(i + 1);
            //}
        //}

        //if self.path.is_empty() {
            //self.is_done = true;
        //}
    //}
}

impl<'a> Iterator for NodesMut<'a> {
    type Item = &'a mut Node;

    fn next(&mut self) -> Option<&'a mut Node> {
        if self.is_done {
            return None;
        }

        return None;
        //if let Some(n) = self.find_node() {
            ////self.increment_path();
            //Some(n)
        //}
        //else {
            //None
        //}
    }
}

mod tests {
    use ::client::layout::*;

    //#[test]
    //fn it_iterates_with_root() {
        //let mut root = Node::row(NodeOptions { height: Some(1), ..Default::default()}, vec![]);
        //let mut nodes = NodesMut::new(&mut root);
        //assert_eq!(nodes.next().unwrap().height, Some(1));
        //assert_eq!(nodes.next(), None);
    //}

    //#[test]
    //fn it_iterates_root_and_child() {
        //let root = Node::row(Default::default(), vec![
            //Node::row(Default::default(), vec![]),
        //]);
        //let mut nodes = NodesMut::new(&root);
        //assert_eq!(nodes.next(), Some(&root));
        //assert_eq!(nodes.next(), root.children.first());
        //assert_eq!(nodes.next(), None);
    //}

    //#[test]
    //fn it_iterates_root_and_two_children() {
        //let root = Node::row(Default::default(), vec![
            //Node::row(Default::default(), vec![]),
            //Node::row(Default::default(), vec![]),
        //]);
        //let mut nodes = NodesMut::new(&root);
        //assert_eq!(nodes.next(), Some(&root));
        //assert_eq!(nodes.next(), root.children.first());
        //assert_eq!(nodes.next(), root.children.last());
        //assert_eq!(nodes.next(), None);
    //}

    //#[test]
    //fn it_iterates_root_with_two_children_the_first_of_which_has_a_child() {
        //let root = Node::row(Default::default(), vec![
            //Node::row(Default::default(), vec![
                //Node::row(Default::default(), vec![]),
            //]),
            //Node::row(Default::default(), vec![]),
        //]);
        //let mut nodes = NodesMut::new(&root);
        //assert_eq!(nodes.next(), Some(&root));
        //assert_eq!(nodes.next(), root.children.first());
        //assert_eq!(nodes.next(), root.children.first().unwrap().children.first());
        //assert_eq!(nodes.next(), root.children.last());
        //assert_eq!(nodes.next(), None);
    //}

    //#[test]
    //fn it_iterates_left_handed_tree() {
        //let root = Node::row(Default::default(), vec![
            //Node::row(Default::default(), vec![
                //Node::row(Default::default(), vec![
                    //Node::row(Default::default(), vec![
                        //Node::row(Default::default(), vec![]),
                    //]),
                //]),
            //]),
        //]);
        //let mut nodes = NodesMut::new(&root);
        //assert_eq!(nodes.next(), Some(&root));
        //assert_eq!(nodes.next(), root.children.first());
        //assert_eq!(nodes.next(), root.children.first().unwrap().children.first());
        //assert_eq!(nodes.next(), root.children.first().unwrap().children.first().unwrap().children.first());
        //assert_eq!(nodes.next(), root.children.first().unwrap().children.first().unwrap().children.first().unwrap().children.first());
        //assert_eq!(nodes.next(), None);
    //}
}
