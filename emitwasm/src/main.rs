use std::collections::HashMap;

use std::any::Any;
use walrus::ir::{Expr, ExprId, Value};
use walrus::{FunctionBuilder, LocalFunction, Module, ModuleConfig, ValType};

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
    let mut module = Module::with_config(config);
    let negate_func_type = module.types.add(&[ValType::F32], &[ValType::F32]);
    let negate_func = module.add_import_func("env", "negate", negate_func_type);
    let ty = module.types.add(&[ValType::F32], &[ValType::F32]);
    let mut builder = FunctionBuilder::new();
    //    let local_fn = LocalFunction::new()
    //    let mut block_builder = builder.block(Box::new([]), Box::new([]));
    //    let const_expr = block_builder.alloc Expr::Const { value: Value::F32(42.0) };
    //    let call_expr = ExprId::from(Expr::Call { func: negate_func, args: Box::new([const_expr]) });
    //    block_builder.f32_const()

    //    let expr =
    //block_builder.expr(ExprId::from(expr));

    //block_builder.expr(expr);
    // println!("{:?}", builder.expr);
    //builder.expr.call()
    //builder.block(params: Box<[ValType]>, results: Box<[ValType]>)
    let expr = builder.f32_const(42.0);
    //let unreachable = builder.unreachable();
    let function_id = builder.finish(ty, vec![], vec![expr], &mut module);
    module.exports.add("main", function_id);

    module.emit_wasm_file("out.wasm").unwrap();

    //block.expr(ExprId::)
    //builder.f32_const(42.0);
    //    let negate_fn = "negate1"; // GEt function id.

    // Create box of arguments.
    // let args = []
    // builder.call();

    // walrus::ir::Expr::Call()

    println!("{:?}", negate1);
}
