use super::{NodeId, Port, PortDirection, PortIndex, RenderContext};

pub struct NodeData {
    pub id: NodeId,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub inputs: Vec<Port>,
    pub outputs: Vec<Port>,
}

impl NodeData {
    pub fn new(id: NodeId, name: &str, x: i32, y: i32) -> NodeData {
        NodeData {
            id,
            name: name.to_owned(),
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
}

pub trait Node {
    fn get_node_data(&self) -> &NodeData;
    fn get_node_data_mut(&mut self) -> &mut NodeData;
    fn render(&self, ctx: &mut RenderContext);

    fn get_id(&self) -> NodeId {
        self.get_node_data().id
    }
    fn get_name(&self) -> String {
        self.get_node_data().name.clone()
    }
    fn get_x(&self) -> i32 {
        self.get_node_data().x
    }
    fn get_y(&self) -> i32 {
        self.get_node_data().y
    }
    fn get_inputs(&self) -> &Vec<Port> {
        &self.get_node_data().inputs
    }
    fn get_outputs(&self) -> &Vec<Port> {
        &self.get_node_data().outputs
    }

    fn get_input(&self, index: PortIndex) -> Option<&Port> {
        self.get_node_data().inputs.get(index)
    }

    fn get_input_by_name(&self, name: &str) -> Option<&Port> {
        self.get_node_data().inputs.iter().find(|p| p.name == name)
    }

    fn get_input_by_name_mut(&mut self, name: &str) -> Option<&mut Port> {
        self.get_node_data_mut()
            .inputs
            .iter_mut()
            .find(|p| p.name == name)
    }

    fn get_output_by_name(&self, name: &str) -> Option<&Port> {
        self.get_node_data().outputs.iter().find(|p| p.name == name)
    }

    fn get_output(&self, index: PortIndex) -> Option<&Port> {
        self.get_node_data().outputs.get(index)
    }

    fn get_output_mut(&mut self, name: &str) -> Option<&mut Port> {
        self.get_node_data_mut()
            .outputs
            .iter_mut()
            .find(|p| p.name == name)
    }

    fn set_float(&mut self, name: &str, index: usize, v: f32) {
        match self.get_input_by_name_mut(name) {
            None => {}
            Some(input) => input.set_float(index, v),
        }
    }

    fn get_max_input_size(&self) -> NodeId {
        self.get_node_data()
            .inputs
            .iter()
            .fold(0, |acc, p| if acc > p.size() { acc } else { p.size() })
    }
}
