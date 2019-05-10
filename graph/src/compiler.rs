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
        print!("{:4} ", index);
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
            OP_POP => {
                println!("OP_POP");
            }
            OP_JMP => {
                let addr0 = bytecode[index + 0];
                let addr1 = bytecode[index + 1];
                index += 2;
                println!("OP_JMP {} {}", addr0, addr1);
            }
            OP_IF_EQ_I32 => {
                let addr0 = bytecode[index + 0];
                let addr1 = bytecode[index + 1];
                index += 2;
                println!("OP_IF_EQ_I32 {} {}", addr0, addr1);
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

    pub fn find_label(&self, label: &str) -> Option<usize> {
        self.labels.get(label).map(|pos| *pos)
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

    pub fn push_pop(&mut self) {
        self.bytecode.push(OP_POP);
    }    

    pub fn push_jmp(&mut self, addr: u16) {
        let addr: [u8; 2] = unsafe { std::mem::transmute(addr) };
        self.bytecode.push(OP_JMP);
        self.bytecode.extend(addr.iter());
    }

    pub fn push_if_eq_i32(&mut self, addr: u16) {
        let addr: [u8; 2] = unsafe { std::mem::transmute(addr) };
        self.bytecode.push(OP_IF_EQ_I32);
        self.bytecode.extend(addr.iter());
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

    fn visit_input_port(
        &mut self,
        node: &Node,
        input_port: &str,
        context: &mut CompilerContext,
    ) -> Result<(), CompileError> {
        match context.network.find_output_node(node, &input_port) {
            Some(output_node) => {
                self.visit(output_node, context)?;
            }
            None => {
                let value = node.values.get(input_port).unwrap();
                self.visit_value(value, context);
            }
        }
        Ok(())
    }
}

impl Visitor for CodeGenVisitor {
    fn visit(&mut self, node: &Node, context: &mut CompilerContext) -> Result<(), CompileError> {
        let label = self.mark_label(node.name.clone());
        match node.kind {
            NodeKind::Switch => {
                let mut node_fixups = HashMap::new();
                let mut jmp_fixups = Vec::new();
                let index_port = &node.kind.inputs()[0];
                self.visit_input_port(node, &index_port, context)?;
                // Now we have the index value of the input to select on the stack.
                let mut port_index = 0;
                for input_port in node.kind.inputs().iter().skip(1) {
                    if let Some(output_node) = context.network.find_output_node(node, &input_port) {
                        // FIXME: better error checking. What happens if we can't find the label here? Is this a compilation error?
                        // let label = self.find_label(&output_node.name).unwrap();
                        // let label: u16 = label as u16;
                        // let label: [u8; 2] = unsafe { std::mem::transmute(label) };
                        // Duplicate the index value.
                        self.push_dup();
                        self.push_const_i32(port_index);
                        self.push_if_eq_i32(0xcccc);
                        node_fixups.insert(output_node.name.clone(), self.bytecode.len() - 2);
                    }
                    port_index += 1;
                }
                // Discard (pop) the index value from the stack.
                self.push_pop();
                // Jump to the end of the node.
                self.push_jmp(0xcccc);
                jmp_fixups.push(self.bytecode.len() - 2);

                // First visit all available switch inputs.
                // FIXME: If the value for switch is constant, we only need to generate the bytecode for the port that is selected.
                let other_inputs = node.kind.inputs();
                for input_port in other_inputs.iter().skip(1) {
                    println!("Visiting {}", input_port);
                    // Visiting the input port also has the side effect that we write out the label (bytecode position) for that node.
                    // We need this position to jump to it from the switch node.
                    self.visit_input_port(node, input_port, context)?;
                    self.push_jmp(0xcccc);
                    jmp_fixups.push(self.bytecode.len() - 2);
                }

                let end_addr = self.bytecode.len();
                let end_addr = end_addr as u16;
                let end_addr: [u8; 2] = unsafe { std::mem::transmute(end_addr) };
                for fixup in &jmp_fixups {
                    self.bytecode[*fixup] = end_addr[0];
                    self.bytecode[*fixup + 1] = end_addr[1];
                }

                for (node_name, addr) in &node_fixups {
                    let node_addr = self.labels[node_name];
                    let node_addr = node_addr as u16;
                    let node_addr: [u8; 2] = unsafe { std::mem::transmute(node_addr) };
                    self.bytecode[*addr] = node_addr[0];
                    self.bytecode[*addr + 1] = node_addr[1];
                }

                // print_bytecode(&self.bytecode);
                // println!("LABELS: {:?}", self.labels);
                // println!("FIXUPS: {:?}", node_fixups);
                // println!("JMP FIXUPS: {:?}", jmp_fixups);

                // Afterwards the value of the input_port is still on the stack.

                //self.bytecode.push()

                // Evaluate the first op
                // Compare the output to see the result
                // Switch based on the label
            }
            _ => {
                // Prepare arguments
                let input_ports = node.kind.inputs();
                for input_port in input_ports {
                    self.visit_input_port(node, &input_port, context)?;
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
