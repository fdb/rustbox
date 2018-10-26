mod port;
mod node;
//mod op;

pub use self::port::{PortPolarity, PortKind, PortValue, Port};
pub use self::node::{Node, NodeData};

struct NullNode {
    data: NodeData
}

impl NullNode {
    fn new() -> NullNode {
        NullNode {
            data: NodeData::new("Null", 0, 0)
        }
    }
}

impl Node for NullNode {
    fn get_node_data(&self) -> & NodeData { &self.data }
    fn get_node_data_mut(&mut self) -> &mut NodeData { &mut self.data }

    fn run(&mut self) {}
}

struct AddNode {
    data: NodeData,
    //in_a: &'a Port,
    //in_b: &'a Port,
    //out: &'a Port,
}

impl AddNode {
    pub fn new() -> AddNode {
        // let in_a = Port::new_input("a", PortKind::Float);
        // let in_b = Port::new_input("b", PortKind::Float);
        // let out = Port::new_output("out", PortKind::Float);
        let mut data = NodeData::new("Add", 0, 0);
        data.inputs.push(Port::new_input("a", PortKind::Float));
        data.inputs.push(Port::new_input("b", PortKind::Float));
        data.outputs.push(Port::new_output("out", PortKind::Float));

        AddNode { data }
        // node.get_node_data_mut().inputs.push(&node.in_a);
        // node.get_node_data_mut().inputs.push(&node.in_b);
        // node.get_node_data_mut().outputs.push(&node.out);
        //node
    }
}

impl Node for AddNode {
    fn get_node_data(&self) -> &NodeData { &self.data }
    fn get_node_data_mut(&mut self) -> &mut NodeData { &mut self.data }

    fn run(&mut self) {
        let a = self.get_input("a").unwrap().to_float();
        let b = self.get_input("b").unwrap().to_float();
        self.get_output_mut("out").unwrap().set_float(a + b);
        //self.out.set_float(a + b);
    }
}

pub fn new_node(type_name: &str) -> Option<Box<Node>> {
    match type_name {
        "null" => Some(Box::new(NullNode::new())),
        "add" => Some(Box::new(AddNode::new())),
        _ => None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_node() {
        let mut node = new_node("add").unwrap();
        assert_eq!(node.get_name(), "Add");
        assert_eq!(node.get_inputs().len(), 2);
        assert_eq!(node.get_outputs().len(), 1);
        assert_eq!(node.get_inputs()[0].name, "a");
        assert_eq!(node.get_inputs()[1].name, "b");
        assert_eq!(node.get_outputs()[0].name, "out");
        node.set_float("a", 3.0);
        node.set_float("b", 5.0);
        node.run();
        assert_eq!(node.get_float_output("out").unwrap(), 8.0);
    }
}
