mod connection;
mod function;
mod network;
mod node;
mod port;
mod render_context;

pub use self::connection::Connection;
pub use self::function::Function;
pub use self::network::Network;
pub use self::node::Node;
pub use self::port::{Port, PortDirection, PortKind, PortSlice};
pub use self::render_context::RenderContext;

pub type NodeId = usize;
pub type PortIndex = usize;

struct NullFunction {}
impl Function for NullFunction {
    fn setup(&self, _node: &mut Node) {}
    fn render(&self, _node: &Node, _ctx: &mut RenderContext) {}
}

struct AddFunction {}
impl Function for AddFunction {
    fn setup(&self, node: &mut Node) {
        node.add_float_input_port("a", vec![0.0]);
        node.add_float_input_port("b", vec![0.0]);
        node.add_float_output_port("out");
    }

    fn render(&self, node: &Node, ctx: &mut RenderContext) {
        let max_size = ctx.get_max_input_size(node.id);
        let mut results = Vec::with_capacity(max_size);
        let in_a = ctx.get_input_slice(node.id, 0);
        let in_b = ctx.get_input_slice(node.id, 1);
        for i in 0..max_size {
            let a = in_a.get_float(i);
            let b = in_b.get_float(i);
            results.push(a + b);
        }
        ctx.set_output_floats(node.id, 0, results);
    }
}

struct ParseFloatsFunction {}
impl Function for ParseFloatsFunction {
    fn setup(&self, node: &mut Node) {
        node.add_string_input_port("s", vec!["1;2;3;4;5"]);
        node.add_float_output_port("out");
    }

    fn render(&self, node: &Node, ctx: &mut RenderContext) {
        let max_size = ctx.get_max_input_size(node.id);
        assert_eq!(max_size, 1); // FIXME: support more than one string and combine them.
        let in_s = ctx.get_input_slice(node.id, 0);
        let s = in_s.get_string(0);
        let mut results = Vec::new();
        for part in s.split(';') {
            let v = part.parse::<f32>().unwrap();
            results.push(v);
        }
        ctx.set_output_floats(node.id, 0, results);
    }
}

pub fn new_function(type_name: &str) -> Option<Box<Function>> {
    match type_name {
        "Null" => Some(Box::new(NullFunction {})),
        "Add" => Some(Box::new(AddFunction {})),
        "Parse Floats" => Some(Box::new(ParseFloatsFunction {})),
        _ => None,
    }
}

pub fn new_node(id: NodeId, type_name: &str, x: i32, y: i32) -> Option<Node> {
    if let Some(mut function) = new_function(type_name) {
        let mut node = Node {
            id,
            name: type_name.to_owned(),
            function: Box::new(NullFunction {}),
            x,
            y,
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        function.setup(&mut node);
        std::mem::swap(&mut node.function, &mut function);
        return Some(node);
    }
    return None;
}

#[cfg(test)]
mod test {
    use super::*;

    fn render_single_node(
        node: Node,
        output_port_index: PortIndex,
    ) -> Result<PortSlice, &'static str> {
        let node_id = node.id;
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
        assert_eq!(node.name, "Add");
        assert_eq!(node.inputs.len(), 2);
        assert_eq!(node.outputs.len(), 1);
        assert_eq!(node.inputs[0].name, "a");
        assert_eq!(node.inputs[1].name, "b");
        assert_eq!(node.outputs[0].name, "out");
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
