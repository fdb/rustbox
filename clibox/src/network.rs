use super::{Connection, Node};

pub struct Network {
    pub rendered_id: usize, 
    pub nodes: Vec<Box<Node>>,
    pub connections: Vec<Connection>,
}

impl Network {
    pub fn new() -> Network {
        Network {
            rendered_id: 0, 
            nodes: Vec::new(), 
            connections: Vec::new(),
        }
    }

    pub fn run(&mut self) {
    }

    pub fn get_rendered_node(&self) -> Option<&Box<Node>> {
        self.get_node(self.rendered_id)
    }

    pub fn get_node(&self, id: usize) -> Option<&Box<Node>> {
        self.nodes.iter().find(|n| n.get_id() == id)
    }
}
