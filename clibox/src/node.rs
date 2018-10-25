use super::{Port, PortKind};

pub struct Node {
    pub name: String,
    pub ports: Vec<Port>,
}

impl Node {
    pub fn new(name: &str) -> Node {
        let mut ports = Vec::new();
        ports.push(Port::new_input("a", PortKind::Float));
        ports.push(Port::new_input("b", PortKind::Float));
        ports.push(Port::new_output("out", PortKind::Float));
        Node {
            name: name.to_owned(),
            ports
        }
    }
}
