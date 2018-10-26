mod port;
mod node;
//mod op;

pub use self::port::{PortPolarity, PortKind, PortValue, Port};
pub use self::node::{Node, NodeData};

struct NullNode<'a> {
    data: NodeData<'a>,
}

impl<'a> NullNode<'a> {
    fn new() -> NullNode<'a> {
        NullNode {
            data: NodeData::new("Null", 0, 0)
        }
    }
}

impl<'a> Node<'a> for NullNode<'a> {
    fn get_node_data(&'a self) -> &'a NodeData { &self.data }
    fn get_node_data_mut(&'a mut self) -> &'a mut NodeData { &mut self.data }

    fn run(&mut self) {}
}

struct AddNode<'a> {
    data: NodeData<'a>,
    in_a: Port,
    in_b: Port,
    out: Port,
}

impl<'a> AddNode<'a> {
    fn new() -> AddNode<'a> {
        let in_a = Port::new_input("a", PortKind::Float);
        let in_b = Port::new_input("b", PortKind::Float);
        let out = Port::new_output("out", PortKind::Float);
        let data = NodeData::new("Add", 0, 0);
        let mut node = AddNode { data, in_a, in_b, out };
        // node.get_node_data_mut().inputs.push(&node.in_a);
        // node.get_node_data_mut().inputs.push(&node.in_b);
        // node.get_node_data_mut().outputs.push(&node.out);
        node
    }
}

impl<'a> Node<'a> for AddNode<'a> {
    fn get_node_data(&'a self) -> &'a NodeData { &self.data }
    fn get_node_data_mut(&'a mut self) -> &'a mut NodeData { &mut self.data }

    fn run(&mut self) {
        let a = self.in_a.to_float();
        let b = self.in_b.to_float();
        self.out.set_float(a + b);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_node() {
        let node = AddNode::new();
        assert_eq!(node.get_name(), "Add");
        assert_eq!(node.get_inputs().len(), 2);
        assert_eq!(node.get_outputs().len(), 1);
        assert_eq!(node.get_inputs()[0].name, "a");
        assert_eq!(node.get_inputs()[1].name, "b");
        assert_eq!(node.get_outputs()[0].name, "out");
    }
}
