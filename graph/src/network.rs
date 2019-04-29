
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub enum Value {
    Int(i32),
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum NodeKind {
    Int,
    Add,
    Negate,
    Switch,
    Frame,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub kind: NodeKind,
    pub values: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Connection {
    pub output: String,
    pub input: String,
    pub port: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    pub name: String,
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
}

impl Network {
    pub fn find_node_by_name(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|&node| node.name == name)
    }

    // pub fn input_nodes<'a>(&'a self, node: &'a Node) -> impl Iterator<Item = &'a Node> {
    //     self.connections.iter()
    //         .filter(|&conn| &conn.input == &node.name)
    //         .map(|&conn| self.find_node_by_name(&conn.output).unwrap())
    // }

    pub fn input_nodes<'a>(&'a self, node: &'a Node) -> Vec<&Node> {
        self.connections.iter()
            .filter(|&conn| conn.input == node.name)
            .map(|conn| self.find_node_by_name(&conn.output).unwrap())
            .collect()
    }

    pub fn is_time_dependent(&self, node: &Node) -> bool {
        // If I am time-dependent myself, the result will always be true.
        if is_time_dependent(node.kind) { return true; }
        // If my input connections are time-dependent, I am as well.
        for input_node in self.input_nodes(node) {
            if self.is_time_dependent(input_node) {
                return true;
            }
        }
        false
    }
}

pub fn port_index_for_node_kind(kind: NodeKind, port: &str) -> usize {
    let inputs = inputs_for_node_kind(kind);
    inputs.iter().position(|s| s == port).unwrap_or(0)
}

pub fn inputs_for_node_kind(kind: NodeKind) -> Vec<String> {
    match kind {
        NodeKind::Int => vec!["v".to_owned()],
        NodeKind::Add => vec!["a".to_owned(), "b".to_owned()],
        NodeKind::Negate => vec!["v".to_owned()],
        NodeKind::Switch => vec![
            "index".to_owned(),
            "in0".to_owned(),
            "in1".to_owned(),
            "in2".to_owned(),
            "in3".to_owned(),
        ],
        NodeKind::Frame => vec![],
    }
}

pub fn is_time_dependent(kind: NodeKind) -> bool {
    match kind {
        NodeKind::Frame => true,
        _ => false
    }
}