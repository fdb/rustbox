mod connection;
mod network;
mod node;
mod port;
mod render_context;

pub use self::connection::Connection;
pub use self::network::Network;
pub use self::node::{Node, NodeData};
pub use self::port::{Port, PortDirection, PortKind, PortSlice};
pub use self::render_context::RenderContext;

pub type NodeId = usize;
pub type PortIndex = usize;

struct NullNode {
    data: NodeData,
}

impl NullNode {
    fn new(id: NodeId, x: i32, y: i32) -> NullNode {
        NullNode {
            data: NodeData::new(id, "Null", x, y),
        }
    }
}

impl Node for NullNode {
    fn get_node_data(&self) -> &NodeData {
        &self.data
    }
    fn get_node_data_mut(&mut self) -> &mut NodeData {
        &mut self.data
    }

    fn render(&self, _ctx: &mut RenderContext) {}
}

struct AddNode {
    data: NodeData,
    //in_a: &'a Port,
    //in_b: &'a Port,
    //out: &'a Port,
}

impl AddNode {
    pub fn new(id: NodeId, x: i32, y: i32) -> AddNode {
        // let in_a = Port::new_input("a", PortKind::Float);
        // let in_b = Port::new_input("b", PortKind::Float);
        // let out = Port::new_output("out", PortKind::Float);
        let mut data = NodeData::new(id, "Add", x, y);
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
    fn get_node_data(&self) -> &NodeData {
        &self.data
    }
    fn get_node_data_mut(&mut self) -> &mut NodeData {
        &mut self.data
    }

    fn render(&self, ctx: &mut RenderContext) {
        let max_size = ctx.get_max_input_size(self.get_id());
        let mut results = Vec::with_capacity(max_size);
        let in_a = ctx.get_input_slice(self.get_id(), 0);
        let in_b = ctx.get_input_slice(self.get_id(), 1);
        for i in 0..max_size {
            let a = in_a.get_float(i);
            let b = in_b.get_float(i);
            results.push(a + b);
        }
        ctx.set_output_floats(self.get_id(), 0, results);
    }
}

struct ParseFloatsNode {
    data: NodeData,
}

impl ParseFloatsNode {
    pub fn new(id: NodeId, x: i32, y: i32) -> ParseFloatsNode {
        let mut data = NodeData::new(id, "Parse Floats", x, y);
        data.add_string_input_port("s", vec!["1;2;3;4;5"]);
        data.add_float_output_port("out");
        ParseFloatsNode { data }
    }
}

impl Node for ParseFloatsNode {
    fn get_node_data(&self) -> &NodeData {
        &self.data
    }
    fn get_node_data_mut(&mut self) -> &mut NodeData {
        &mut self.data
    }

    fn render(&self, ctx: &mut RenderContext) {
        let max_size = self.get_max_input_size();
        assert_eq!(max_size, 1); // FIXME: support more than one string and combine them.
        let in_s = ctx.get_input_slice(self.get_id(), 0);
        let s = in_s.get_string(0);
        let mut results = Vec::new();
        for part in s.split(';') {
            let v = part.parse::<f32>().unwrap();
            results.push(v);
        }
        ctx.set_output_floats(self.get_id(), 0, results);
    }
}

pub fn new_node(id: NodeId, type_name: &str, x: i32, y: i32) -> Option<Box<Node>> {
    match type_name {
        "Null" => Some(Box::new(NullNode::new(id, x, y))),
        "Add" => Some(Box::new(AddNode::new(id, x, y))),
        "Parse Floats" => Some(Box::new(ParseFloatsNode::new(id, x, y))),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn render_single_node(
        node: Box<Node>,
        output_port_index: PortIndex,
    ) -> Result<PortSlice, &'static str> {
        let node_id = node.get_id();
        let mut network = Network::new();
        network.nodes.push(node);
        network.rendered_id = node_id;
        let mut ctx = RenderContext::new(&network);
        // FIXME: ctx.render()
        network.render(&mut ctx)?;
        Ok(ctx
            .get_output_slice(node_id, output_port_index)
            .unwrap()
            .clone())
    }

    #[test]
    fn create_node() {
        let mut node = new_node(1, "Add", 0, 0).unwrap();
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
        let results = render_single_node(node, 0).unwrap();
        assert_eq!(results.size(), 2);
        assert_eq!(results.get_float(0), 8.0);
        assert_eq!(results.get_float(1), 800.0);
    }

    #[test]
    fn test_parse_floats() {
        let node = new_node(1, "Parse Floats", 0, 0).unwrap();
        let results = render_single_node(node, 0).unwrap();
        assert_eq!(results.size(), 5);
        assert_eq!(results.get_float(0), 1.0);
        assert_eq!(results.get_float(1), 2.0);
        assert_eq!(results.get_float(2), 3.0);
        assert_eq!(results.get_float(3), 4.0);
        assert_eq!(results.get_float(4), 5.0);
        assert_eq!(results.get_float(5), 1.0);
    }

    #[test]

    fn test_network() {
        let mut network = Network::new();
        let parse_floats_node = new_node(1, "Parse Floats", 0, 0).unwrap();
        network.nodes.push(parse_floats_node);
        let mut add_node = new_node(2, "Add", 0, 0).unwrap();
        add_node.set_float("b", 0, 100.0);
        network.nodes.push(add_node);
        network.connections.push(Connection::new(1, 0, 2, 0));
        network.rendered_id = 2;
        let mut ctx = RenderContext::new(&network);
        network.render(&mut ctx).unwrap();
        let slice = ctx.get_output_slice(network.rendered_id, 0).unwrap();
        assert_eq!(slice.size(), 5);
        assert_eq!(slice.get_float(0), 101.0);
        assert_eq!(slice.get_float(4), 105.0);
    }
}
