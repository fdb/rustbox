//! Compiles the network to bytecode.

use crate::bytecode::*;
use crate::network::{Network, Node, NodeKind, Value};
use std::collections::HashMap;

trait ToByteCode {
    fn to_bytecode(&self, bytecode: &mut Vec<u8>);
}

impl ToByteCode for Value {
    fn to_bytecode(&self, bytecode: &mut Vec<u8>) {
        match self {
            Value::Int(v) => {
                let v: [u8; 4] = unsafe { std::mem::transmute(*v) };
                bytecode.push(OP_CONST_I32);
                bytecode.extend(v.iter());
            }
        }
    }
}

impl ToByteCode for NodeKind {
    fn to_bytecode(&self, bytecode: &mut Vec<u8>) {
        let op = (*self).into();
        bytecode.push(op);
    }
}

pub fn print_bytecode(bytecode: &Vec<u8>) {
    let mut index: usize = 0;
    loop {
        let op = bytecode[index];
        match op {
            OP_CONST_I32 => {
                let x1 = bytecode[index + 1];
                let x2 = bytecode[index + 2];
                let x3 = bytecode[index + 3];
                let x4 = bytecode[index + 4];
                index += 4;
                println!("OP_CONST_I32 {} {} {} {}", x1, x2, x3, x4);
            }
            OP_CALL_NODE => {
                let kind = bytecode[index + 1];
                let kind = match kind {
                    1 => NodeKind::Int,
                    2 => NodeKind::Add,
                    3 => NodeKind::Negate,
                    4 => NodeKind::Switch,
                    5 => NodeKind::Frame,
                    x => panic!("Error unknown kind {}", x),
                };
                println!("OP_CALL {:?}", kind);
                index += 1;
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
        index += 1;
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
}

impl CompiledNetwork {
    pub fn new() -> CompiledNetwork {
        CompiledNetwork {
            bytecode: Vec::new(),
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
}

impl CodeGenVisitor {
    pub fn new() -> CodeGenVisitor {
        CodeGenVisitor {
            bytecode: Vec::new(),
            labels: HashMap::new(),
        }
    }

    pub fn mark_label(&mut self, label: String) {
        self.labels.insert(label, self.bytecode.len());
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
                            value.to_bytecode(&mut self.bytecode);
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

    Ok(compiled_network)
}
