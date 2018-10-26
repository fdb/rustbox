mod port;
mod node;
mod op;

pub use self::port::{PortPolarity, PortKind, PortValue, Port};
pub use self::node::{Node};
pub use self::op::{Op};

struct NullOp {}

impl Op for NullOp {
    fn run(&self, node: &mut Node) {}
}

struct AddOp<'a> {
    in_a: Option<&'a Port>,
    in_b: Option<&'a Port>,
    out: Option<&'a Port>,
}

impl<'a> AddOp<'a> {
    fn new() -> Node {
        let mut op = Box::new(AddOp { in_a: None, in_b: None, out: None });
        let in_a = Port::new_input("a", PortKind::Float);
        let in_b = Port::new_input("b", PortKind::Float);
        let out = Port::new_output("out", PortKind::Float);
        op.in_a = Some(&in_a);
        op.in_b = Some(&in_b);
        op.out = Some(&out);
        let mut node = Node::new("Add", op);
        node.ports.push(in_a);
        node.ports.push(in_b);
        node.ports.push(out);
        node
    }
}

impl<'a> Op for AddOp<'a> {

    fn run(&self, node: &mut Node) {
        let a = self.in_a.unwrap().to_float();
        let b = self.in_b.unwrap().to_float();
        self.out.unwrap().set_float(a + b);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_node() {
        let node = Node::new("Add", Box::new(NullOp {}));
        assert_eq!(node.name, "alpha");
        assert_eq!(node.ports.len(), 3);
        assert_eq!(node.ports[0].name, "a");
        assert_eq!(node.ports[1].name, "b");
        assert_eq!(node.ports[2].name, "out");
    }
}
