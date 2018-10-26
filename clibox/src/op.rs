use super::{Node};

pub trait Op {
    fn run(&self, node: &mut Node);
}
