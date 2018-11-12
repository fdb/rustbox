pub use crate::{Function, Node, RenderContext};

pub struct ValueFunction {}
impl Function for ValueFunction {
    fn setup(&self, node: &mut Node) {
        node.add_float_input_port("v", vec![0.0]);
        node.add_float_output_port("out");
    }

    fn render(&self, node: &Node, ctx: &mut RenderContext) {
        let in_v = ctx.get_input_slice(node.id, 0);
        ctx.set_output_slice(node.id, 0, in_v.clone());
    }
}

pub struct AddFunction {}
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

pub struct ParseFloatsFunction {}
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
