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
    let mut negate1_values = HashMap::new();
    negate1_values.insert("v".to_string(), 42.0);
    let negate1 = Node {
        name: "negate1".to_string(),
        x: 0,
        y: 0,
        kind: NodeKind::Negate,
        values: negate1_values,
    };

    let config = ModuleConfig::new();
    //config.
    let mut module = Module::with_config(config);
    let ty = module.types.add(&[ValType::F32], &[ValType::F32]);
    let mut builder = FunctionBuilder::new();
    // let mut block_builder = builder.block(Box::new([]), Box::new([]));
    // block_builder.
    // println!("{:?}", builder.expr);
    //builder.expr.call()
    //builder.block(params: Box<[ValType]>, results: Box<[ValType]>)
    let expr = builder.f32_const(42.0);
    //let unreachable = builder.unreachable();
    let fid = builder.finish(ty, vec![], vec![expr], &mut module);
    module.exports.add("main", fid);

    module.emit_wasm_file("out.wasm").unwrap();

    //block.expr(ExprId::)
    //builder.f32_const(42.0);
    let negate_fn = "negate1"; // GEt function id.

    // Create box of arguments.
    // let args = []
    // builder.call();

    // walrus::ir::Expr::Call()

    println!("{:?}", negate1);
}
