use ego_tree;

use super::*;

pub trait LineContainer {
    // return vec of child elements organized into horizontal lines
    fn lines(&self) -> Vec<Vec<ego_tree::NodeId<Wrap>>>;
}

impl<'a> LineContainer for ego_tree::NodeRef<'a, Wrap> {
    fn lines(&self) -> Vec<Vec<ego_tree::NodeId<Wrap>>> {
        let mut output = vec![];
        let mut line = vec![];

        for child in self.children() {
            if child.value().is_new_line() && line.len() > 0 {
                output.push(line);
                line = vec![];
            }

            line.push(child.id());
        }

        if line.len() > 0 {
            output.push(line)
        }

        output
    }
}
