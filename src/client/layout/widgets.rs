use std::slice::Iter;
use vterm_sys;
use itertools::Itertools;
use super::{Widget};

/// An iterator for widgets within a Node.
#[derive(Debug)]
pub struct Widgets<'a> {
    pub widgets: Vec<&'a Widget>,
    pub index: usize,
}

impl<'a> Iterator for Widgets<'a> {
    type Item = &'a Widget;

    fn next(&mut self) -> Option<&'a Widget> {
        if self.index < self.widgets.len() {
            let w = Some(self.widgets[self.index]);
            self.index += 1;
            w
        } else {
            None
        }
    }
}
