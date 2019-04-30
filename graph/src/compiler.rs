//! Compiles the network to bytecode.

use crate::bytecode::*;
use crate::network::{Network, Node};

#[derive(Debug, PartialEq, Eq)]
pub enum CompileErrorKind {
}

#[derive(Debug, PartialEq, Eq)]
pub struct CompileError {
    location: (usize, usize),
    kind: CompileErrorKind,
}

pub struct CompilerContext {}

trait Visitor {
    fn visit(
        &mut self,
        network: &Network,
        node: &Node,
        context: &mut CompilerContext,
    ) -> Result<(), CompileError>;

    fn visit_inputs(
        &mut self,
        network: &Network,
        node: &Node,
        context: &mut CompilerContext,
    ) -> Result<(), CompileError> {
        for input_node in &network.input_nodes(node) {
            self.visit(network, input_node, context)?;
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
    fn visit(
        &mut self,
        network: &Network,
        node: &Node,
        context: &mut CompilerContext,
    ) -> Result<(), CompileError> {
        println!("Visiting: {}", node.name);
        self.visit_inputs(network, node, context)
    }
}

pub fn compile_network(network: &Network) -> Result<CompiledNetwork, CompileError> {
    let mut context = CompilerContext {};

    let rendered_node = network.rendered_node();
    if rendered_node.is_none() {
        let mut result = CompiledNetwork::new();
        result.bytecode.push(OP_END);
        return Ok(result);
    }
    let rendered_node = rendered_node.unwrap();

    let mut log_node_names_visitor = LogNodeNamesVisitor {};
    log_node_names_visitor.visit(network, rendered_node, &mut context)?;
    Ok(CompiledNetwork::new())
}
