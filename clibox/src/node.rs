use super::{Port};

pub struct NodeData<'a> {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub inputs: Vec<&'a Port>,
    pub outputs: Vec<&'a Port>,
}

impl<'a> NodeData<'a> {
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

pub trait Node<'a> {
    fn get_node_data(&'a self) -> &'a NodeData;
    fn get_node_data_mut(&'a mut self) -> &'a mut NodeData;
    fn run(&mut self);

    fn get_name(&'a self) -> String { self.get_node_data().name.clone() }
    fn get_x(&'a self) -> i32 { self.get_node_data().x }
    fn get_y(&'a self) -> i32 { self.get_node_data().y }
    fn get_inputs(&'a self) -> &Vec<&Port> { &self.get_node_data().inputs }
    fn get_outputs(&'a self) -> &Vec<&Port> { &self.get_node_data().outputs }
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
