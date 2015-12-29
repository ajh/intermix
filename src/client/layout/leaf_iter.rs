use std::slice::Iter;
use vterm_sys;
use itertools::Itertools;
use super::{Node};

/// An iterator for leaf nodes.
#[derive(Debug)]
pub struct LeafIter<'a> {
    pub nodes: Vec<&'a Node>,
    pub index: usize,
}

impl<'a> Iterator for LeafIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<&'a Node> {
        if self.index < self.nodes.len() {
            let w = Some(self.nodes[self.index]);
            self.index += 1;
            w
        } else {
            None
        }
    }
}
