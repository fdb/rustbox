use super::{Function, NodeId, NullFunction, Port, PortDirection, PortIndex, RenderContext};

pub struct Node {
    pub id: NodeId,
    pub name: String,
    pub function: Box<Function>,
    pub x: i32,
    pub y: i32,
    pub inputs: Vec<Port>,
    pub outputs: Vec<Port>,
}

impl Node {
    pub fn new(id: NodeId, name: &str, x: i32, y: i32) -> Node {
        Node {
            id,
            name: name.to_owned(),
            function: Box::new(NullFunction {}),
            x,
            y,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn add_int_input_port(&mut self, name: &str, values: Vec<i32>) {
        self.inputs
            .push(Port::new_int_port(name, values, PortDirection::In));
    }

    pub fn add_float_input_port(&mut self, name: &str, values: Vec<f32>) {
        self.inputs
            .push(Port::new_float_port(name, values, PortDirection::In));
    }

    pub fn add_string_input_port(&mut self, name: &str, values: Vec<&str>) {
        self.inputs
            .push(Port::new_string_port(name, values, PortDirection::In));
    }

    pub fn add_int_output_port(&mut self, name: &str) {
        self.outputs
            .push(Port::new_int_port(name, vec![], PortDirection::Out));
    }

    pub fn add_float_output_port(&mut self, name: &str) {
        self.outputs
            .push(Port::new_float_port(name, vec![], PortDirection::Out));
    }

    pub fn add_string_output_port(&mut self, name: &str) {
        self.inputs
            .push(Port::new_string_port(name, vec![], PortDirection::Out));
    }

    pub fn render(&self, ctx: &mut RenderContext) {
        self.function.render(&self, ctx)
    }

    pub fn get_input(&self, index: PortIndex) -> Option<&Port> {
        self.inputs.get(index)
    }

    pub fn get_input_by_name(&self, name: &str) -> Option<&Port> {
        self.inputs.iter().find(|p| p.name == name)
    }

    pub fn get_input_by_name_mut(&mut self, name: &str) -> Option<&mut Port> {
        self.inputs.iter_mut().find(|p| p.name == name)
    }

    pub fn get_output_by_name(&self, name: &str) -> Option<&Port> {
        self.outputs.iter().find(|p| p.name == name)
    }

    pub fn get_output(&self, index: PortIndex) -> Option<&Port> {
        self.outputs.get(index)
    }

    pub fn get_output_mut(&mut self, name: &str) -> Option<&mut Port> {
        self.outputs.iter_mut().find(|p| p.name == name)
    }

    pub fn set_float(&mut self, name: &str, index: usize, v: f32) {
        match self.get_input_by_name_mut(name) {
            None => {}
            Some(input) => input.set_float(index, v),
        }
    }

    pub fn get_max_input_size(&self) -> usize {
        self.inputs
            .iter()
            .fold(0, |acc, p| if acc > p.size() { acc } else { p.size() })
    }
}
