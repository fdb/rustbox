//! Compiles the network to bytecode.

use crate::bytecode::*;
use crate::network::{Network, Node, NodeKind};
use crate::value::{Value, ValueKind};
use std::collections::HashMap;

trait ToByteCode {
    fn to_bytecode(&self, bytecode: &mut Vec<u8>);
}

// impl ToByteCode for Value {
//     fn to_bytecode(&self, bytecode: &mut Vec<u8>) {
//         match self {
//             Value::Int(v) => {
//                 let v: [u8; 4] = unsafe { std::mem::transmute(*v) };
//                 bytecode.push(OP_CONST_I32);
//                 bytecode.extend(v.iter());
//             }
//         }
//     }
// }

impl ToByteCode for i32 {
    fn to_bytecode(&self, bytecode: &mut Vec<u8>) {
        let v: [u8; 4] = unsafe { std::mem::transmute(*self) };
        // bytecode.push(OP_CONST_I32);
        bytecode.extend(v.iter());
    }
}

impl ToByteCode for f32 {
    fn to_bytecode(&self, bytecode: &mut Vec<u8>) {
        let v: [u8; 4] = unsafe { std::mem::transmute(*self) };
        // bytecode.push(OP_CONST_F32);
        bytecode.extend(v.iter());
    }
}

// impl ToByteCode for Spread {
//     fn to_bytecode(&self, bytecode: &mut Vec<u8>) {
//         match self {
//             Spread::Int(vals) => {
//                 if vals.len() == 1 {
//                     bytecode.push(OP_CONST_I32);
//                     vals[0].to_bytecode(bytecode);
//                 } else {
//                     //bytecode.push(OP_SPREAD_I32);
//                     //bytecode.push(1);
//                 }
//             }
//             Spread::Float(vals) => {
//                 if vals.len() == 1 {
//                     bytecode.push(OP_CONST_F32);
//                     vals[0].to_bytecode(bytecode);
//                 } else {
//                     //bytecode.push(OP_SPREAD_I32);
//                     //bytecode.push(1);
//                 }
//             }
//             Spread::String(vals) => {}
//         }
//     }
// }

impl ToByteCode for NodeKind {
    fn to_bytecode(&self, bytecode: &mut Vec<u8>) {
        let op = (*self).into();
        bytecode.push(op);
    }
}

pub fn print_constant_pool(constant_pool: &Vec<Value>) {
    for i in 0..constant_pool.len() {
        let value = &constant_pool[i];
        println!("{:2}: {:?}", i, value);
    }
}

pub fn print_bytecode(bytecode: &Vec<u8>) {
    let mut index: usize = 0;
    loop {
        let op = bytecode[index];
        index += 1;
        match op {
            OP_CONST_I32 => {
                let x1 = bytecode[index + 0];
                let x2 = bytecode[index + 1];
                let x3 = bytecode[index + 2];
                let x4 = bytecode[index + 3];
                index += 4;
                println!("OP_CONST_I32 {} {} {} {}", x1, x2, x3, x4);
            }
            OP_DUP => {
                println!("OP_DUP");
            }
            OP_CALL_NODE => {
                let kind = bytecode[index];
                index += 1;
                let kind = match kind {
                    1 => NodeKind::Int,
                    2 => NodeKind::Add,
                    3 => NodeKind::Negate,
                    4 => NodeKind::Switch,
                    5 => NodeKind::Frame,
                    x => panic!("Error unknown kind {}", x),
                };
                println!("OP_CALL {:?}", kind);
            }
            // OP_VALUE_NEW => {
            //     let kind = bytecode[index];
            //     index += 1;
            //     let kind = ValueKind::from(kind);
            //     println!("OP_SPREAD_NEW {:?}", kind);
            // }
            // OP_SPREAD_STORE => {
            //     println!("OP_SPREAD_STORE");
            // }
            OP_VALUE_LOAD => {
                println!("OP_VALUE_LOAD");
            }
            OP_END => {
                println!("OP_END");
                break;
            }
            x => {
                println!("ERROR UNKNOWN BYTECODE {}", x);
                break;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CompileErrorKind {}

#[derive(Debug, PartialEq, Eq)]
pub struct CompileError {
    location: (usize, usize),
    kind: CompileErrorKind,
}

pub struct CompilerContext<'a> {
    pub network: &'a Network,
}

trait Visitor {
    fn visit(&mut self, node: &Node, context: &mut CompilerContext) -> Result<(), CompileError>;

    fn visit_inputs(
        &mut self,
        node: &Node,
        context: &mut CompilerContext,
    ) -> Result<(), CompileError> {
        for input_node in &context.network.input_nodes(node) {
            self.visit(input_node, context)?;
        }
        Ok(())
    }
}

pub struct CompiledNetwork {
    pub bytecode: Vec<u8>,
    pub constant_pool: Vec<Value>,
}

impl CompiledNetwork {
    pub fn new() -> CompiledNetwork {
        CompiledNetwork {
            bytecode: Vec::new(),
            constant_pool: Vec::new(),
        }
    }
}

struct LogNodeNamesVisitor;

impl Visitor for LogNodeNamesVisitor {
    fn visit(&mut self, node: &Node, context: &mut CompilerContext) -> Result<(), CompileError> {
        println!("Visiting: {}", node.name);
        self.visit_inputs(node, context)
    }
}

struct CodeGenVisitor {
    pub bytecode: Vec<u8>,
    pub labels: HashMap<String, usize>,
    pub constant_pool: Vec<Value>,
}

impl CodeGenVisitor {
    pub fn new() -> CodeGenVisitor {
        CodeGenVisitor {
            bytecode: Vec::new(),
            labels: HashMap::new(),
            constant_pool: Vec::new(),
        }
    }

    pub fn mark_label(&mut self, label: String) {
        self.labels.insert(label, self.bytecode.len());
    }

    // We don't use usize since the constant pool can only take the size that we specify, which is bounded.
    pub fn intern_string(&mut self, s: &str) -> i32 {
        // Check if string is in the pool
        let pos = self.constant_pool.iter().position(|v| {
            if let Value::String(vs) = v {
                vs == s
            } else {
                false
            }
        });
        // If it is, return its position.
        if let Some(pos) = pos {
            return pos as i32;
        }
        // If not, place a copy in the constant pool.
        self.constant_pool.push(Value::String(s.to_string()));
        (self.constant_pool.len() - 1) as i32
    }

    pub fn push_const_i32(&mut self, v: i32) {
        self.bytecode.push(OP_CONST_I32);
        v.to_bytecode(&mut self.bytecode);
    }

    pub fn push_const_f32(&mut self, v: f32) {
        self.bytecode.push(OP_CONST_F32);
        v.to_bytecode(&mut self.bytecode);
    }

    pub fn push_dup(&mut self) {
        self.bytecode.push(OP_DUP);
    }

    /// Stack before:
    /// - count
    /// Stack after:
    /// - spread ref
    // pub fn push_spread_new(&mut self, size: usize, kind: SpreadKind) {
    //     // Push spread size
    //     self.bytecode.push(OP_CONST_I32);
    //     (size as i32).to_bytecode(&mut self.bytecode);
    //     // Push spread creation + type
    //     self.bytecode.push(OP_SPREAD_NEW);
    //     self.bytecode.push(SpreadKind::Int.into());
    // }

    // /// This instruction assumes the spread ref is already on the stack.
    // pub fn push_spread_store_i32(&mut self, index: usize, v: i32) {
    //     self.push_const_i32(index as i32);
    //     self.push_const_i32(v);
    //     self.bytecode.push(OP_SPREAD_STORE);
    // }

    fn visit_value(&mut self, value: &Value, _context: &mut CompilerContext) {
        match value {
            Value::Int(v) => self.push_const_i32(*v),
            Value::Float(v) => self.push_const_f32(*v),
            _ => {
                self.constant_pool.push(value.clone());
                let index = (self.constant_pool.len() - 1) as i32;
                self.push_const_i32(index);
                self.bytecode.push(OP_VALUE_LOAD);
            }
        }
    }
}

impl Visitor for CodeGenVisitor {
    fn visit(&mut self, node: &Node, context: &mut CompilerContext) -> Result<(), CompileError> {
        match node.kind {
            NodeKind::Switch => {
                // Evaluate the first op
                // Compare the output to see the result
                // Switch based on the label
            }
            _ => {
                // Prepare arguments
                let input_ports = node.kind.inputs();
                for input_port in input_ports {
                    match context.network.find_output_node(node, &input_port) {
                        Some(output_node) => {
                            self.visit(output_node, context)?;
                        }
                        None => {
                            let value = node.values.get(&input_port).unwrap();
                            self.visit_value(value, context);
                        }
                    }
                }
                self.bytecode.push(OP_CALL_NODE);
                node.kind.to_bytecode(&mut self.bytecode);
            }
        }
        Ok(())
    }
}

pub fn compile_network(network: &Network) -> Result<CompiledNetwork, CompileError> {
    let mut context = CompilerContext { network };

    let rendered_node = network.rendered_node();
    if rendered_node.is_none() {
        let mut result = CompiledNetwork::new();
        result.bytecode.push(OP_END);
        return Ok(result);
    }
    let rendered_node = rendered_node.unwrap();

    // let mut log_node_names_visitor = LogNodeNamesVisitor {};
    // log_node_names_visitor.visit(rendered_node, &mut context)?;

    let mut code_gen_visitor = CodeGenVisitor::new();
    code_gen_visitor.visit(rendered_node, &mut context)?;
    code_gen_visitor.bytecode.push(OP_END);

    let mut compiled_network = CompiledNetwork::new();
    compiled_network.bytecode.extend(code_gen_visitor.bytecode);
    compiled_network.constant_pool = code_gen_visitor.constant_pool;

    Ok(compiled_network)
}
