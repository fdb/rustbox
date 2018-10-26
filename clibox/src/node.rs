use super::{Port};

pub struct NodeData {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub inputs: Vec<Port>,
    pub outputs: Vec<Port>,
}

impl NodeData {
    pub fn new(name: &str, x: i32, y: i32) -> NodeData {
        NodeData {
            name: name.to_owned(),
            x, 
            y,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
}

pub trait Node {
    fn get_node_data(& self) -> & NodeData;
    fn get_node_data_mut(& mut self) -> & mut NodeData;
    fn run(&mut self);

    fn get_name(& self) -> String { self.get_node_data().name.clone() }
    fn get_x(& self) -> i32 { self.get_node_data().x }
    fn get_y(& self) -> i32 { self.get_node_data().y }
    fn get_inputs(& self) -> &Vec<Port> { &self.get_node_data().inputs }
    fn get_outputs(& self) -> &Vec<Port> { &self.get_node_data().outputs }
    fn get_input(&self, name: &str) -> Option<&Port> { self.get_node_data().inputs.iter().find(|p| p.name == name) }
    fn get_input_mut(&mut self, name: &str) -> Option<&mut Port> { self.get_node_data_mut().inputs.iter_mut().find(|p| p.name == name) }
    fn get_output(&self, name: &str) -> Option<&Port> { self.get_node_data().outputs.iter().find(|p| p.name == name) }
    fn get_output_mut(&mut self, name: &str) -> Option<&mut Port> { self.get_node_data_mut().outputs.iter_mut().find(|p| p.name == name) }
    fn set_float(&mut self, name: &str, v: f32) { 
        match self.get_input_mut(name) {
            None => {},
            Some(input) => input.set_float(v)
        }
    }
    fn get_float_output(&self, name: &str) -> Option<f32> { 
        match self.get_output(name) {
            None => None,
            Some(port) => Some(port.to_float())
        }
    }
}

// pub struct Node {
//     pub name: String,
//     pub ports: Vec<Port>,
//     pub op: Box<Op>,
// }

// impl Node {
//     pub fn new(name: &str, op: Box<Op>) -> Node {
//         let mut ports = Vec::new();
//         ports.push(Port::new_input("a", PortKind::Float));
//         ports.push(Port::new_input("b", PortKind::Float));
//         ports.push(Port::new_output("out", PortKind::Float));
//         Node {
//             name: name.to_owned(),
//             ports,
//             op,
//         }
//     }
// }
