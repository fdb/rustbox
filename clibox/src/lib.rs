mod port;
mod node;
//mod op;

pub use self::port::{PortDirection, PortKind, PortValue, Port};
pub use self::node::{Node, NodeData};

struct NullNode {
    data: NodeData
}

impl NullNode {
    fn new(x: i32, y: i32) -> NullNode {
        NullNode {
            data: NodeData::new("Null", x, y)
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
    pub fn new(x: i32, y: i32) -> AddNode {
        // let in_a = Port::new_input("a", PortKind::Float);
        // let in_b = Port::new_input("b", PortKind::Float);
        // let out = Port::new_output("out", PortKind::Float);
        let mut data = NodeData::new("Add", x, y);
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
        let max_size = self.get_max_input_size();
        let mut results = Vec::with_capacity(max_size);
        let in_a = self.get_input("a").unwrap();
        let in_b = self.get_input("b").unwrap();
        //let out = self.get_output_mut("out").unwrap();
        //out.ensure_size(max_size);
        for i in 0..max_size {
            let a = in_a.get_float(i);
            let b = in_b.get_float(i);
            results.push(a + b);
            //out.set_float(i, a + b);
        }
        self.set_output_floats("out", &results);
    }
}

pub fn new_node(type_name: &str, x: i32, y: i32) -> Option<Box<Node>> {
    match type_name {
        "null" => Some(Box::new(NullNode::new(x, y))),
        "add" => Some(Box::new(AddNode::new(x, y))),
        _ => None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_node() {
        let mut node = new_node("add", 0, 0).unwrap();
        assert_eq!(node.get_name(), "Add");
        assert_eq!(node.get_inputs().len(), 2);
        assert_eq!(node.get_outputs().len(), 1);
        assert_eq!(node.get_inputs()[0].name, "a");
        assert_eq!(node.get_inputs()[1].name, "b");
        assert_eq!(node.get_outputs()[0].name, "out");
        node.set_float("a", 0, 3.0);
        node.set_float("b", 0, 5.0);
        node.set_float("a", 1, 300.0);
        node.set_float("b", 1, 500.0);
        assert_eq!(node.get_max_input_size(), 2);
        node.run();
        assert_eq!(node.get_output("out").unwrap().size(), 2);
        assert_eq!(node.get_float_output("out", 0).unwrap(), 8.0);
        assert_eq!(node.get_float_output("out", 1).unwrap(), 800.0);
    }
}
