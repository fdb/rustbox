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
        data.add_float_input_port("a", vec![0.0]);
        data.add_float_input_port("b", vec![0.0]);
        data.add_float_output_port("out");
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

struct ParseFloatsNode {
    data: NodeData
}

impl ParseFloatsNode {
    pub fn new(x: i32, y: i32) -> ParseFloatsNode {
        let mut data = NodeData::new("Parse Floats", x, y);
        data.add_string_input_port("s", vec!["1;2;3;4;5"]);
        data.add_float_output_port("out");
        ParseFloatsNode { data }
    }
}

impl Node for ParseFloatsNode {
    fn get_node_data(&self) -> &NodeData { &self.data }
    fn get_node_data_mut(&mut self) -> &mut NodeData { &mut self.data }

    fn run(&mut self) {
        let max_size = self.get_max_input_size();
        assert_eq!(max_size, 1); // FIXME: support more than one string and combine them.
        let in_s = self.get_input("s").unwrap();
        let s = in_s.get_string(0);
        let mut results = Vec::new();
        for part in s.split(';') {
            let v = part.parse::<f32>().unwrap();
            results.push(v);
        }
        self.set_output_floats("out", &results);
    }
}


pub fn new_node(type_name: &str, x: i32, y: i32) -> Option<Box<Node>> {
    match type_name {
        "Null" => Some(Box::new(NullNode::new(x, y))),
        "Add" => Some(Box::new(AddNode::new(x, y))),
        "Parse Floats" => Some(Box::new(ParseFloatsNode::new(x, y))),
        _ => None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_node() {
        let mut node = new_node("Add", 0, 0).unwrap();
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

    #[test]
    fn test_parse_floats() {
        let mut node = new_node("Parse Floats", 0, 0).unwrap();
        node.run();
        assert_eq!(node.get_output("out").unwrap().size(), 5);
        assert_eq!(node.get_float_output("out", 0).unwrap(), 1.0);
        assert_eq!(node.get_float_output("out", 1).unwrap(), 2.0);
        assert_eq!(node.get_float_output("out", 2).unwrap(), 3.0);
        assert_eq!(node.get_float_output("out", 3).unwrap(), 4.0);
        assert_eq!(node.get_float_output("out", 4).unwrap(), 5.0);
        assert_eq!(node.get_float_output("out", 5).unwrap(), 1.0); // Output wraps
    }
}
