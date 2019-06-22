use std::collections::HashMap;
use walrus::{FunctionBuilder, Module, ModuleConfig, ValType};

#[derive(Debug, Copy, Clone)]
pub enum NodeKind {
    Int,
    Add,
    Negate,
    Switch,
    Frame,
}

impl NodeKind {
    pub fn inputs(&self) -> Vec<String> {
        match self {
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

    pub fn port_index(&self, port: &str) -> Option<usize> {
        let inputs = self.inputs();
        inputs.iter().position(|s| s == port)
    }
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub kind: NodeKind,
    pub values: HashMap<String, f32>,
}

#[derive(Debug)]
pub struct Connection {
    pub output: String,
    pub input: String,
    pub port: String,
}

#[derive(Debug)]
pub struct Network {
    pub name: String,
    pub rendered_node: String,
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
}

fn main() {
    // Construct a Walrus module
    let config = ModuleConfig::new();
    let mut module = Module::with_config(config);

    // Import the "negate" function.
    let negate_func_type = module.types.add(&[ValType::F32], &[ValType::F32]);
    let negate_func = module.add_import_func("env", "negate", negate_func_type);
    
    // Create the main function type.
    let main_func_type = module.types.add(&[], &[ValType::F32]);

    // Build the function.
    let mut builder = FunctionBuilder::new();
    let const_expr = builder.f32_const(42.0);
    let expr = builder.call(negate_func, Box::new([const_expr]));
    let main_func = builder.finish(main_func_type, vec![], vec![expr], &mut module);
    
    // Add the function to the exports.
    module.exports.add("main", main_func);

    // Emit the WASM file.
    module.emit_wasm_file("out.wasm").unwrap();
}
