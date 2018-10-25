mod port;
mod node;

pub use self::port::{PortPolarity, PortKind, PortValue, Port};
pub use self::node::{Node};

struct AddNode {
    pub name: String,
    pub ports: Vec<Port>,
}

//impl Node for AddNode {
//}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_node() {
        let node = Node::new("alpha");
        assert_eq!(node.name, "alpha");
        assert_eq!(node.ports.len(), 3);
        assert_eq!(node.ports[0].name, "a");
        assert_eq!(node.ports[1].name, "b");
        assert_eq!(node.ports[2].name, "out");
    }
}
