use crate::bytecode::*;
use crate::network::NodeKind;

pub struct VM {
    pub bytecode: Vec<u8>,
    pub ip: usize,
    pub stack: Vec<i32>,
}

impl VM {
    pub fn new(bytecode: Vec<u8>) -> VM {
        VM {
            bytecode,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let op = self.bytecode[self.ip];
            self.ip += 1;
            match op {
                OP_CONST_I32 => {
                    let mut value: [u8; 4] = [0;4];
                    value.copy_from_slice(&self.bytecode[self.ip..self.ip + 4]);
                    let value: i32 = unsafe { std::mem::transmute(value) };
                    self.stack.push(value);
                    self.ip += 4;
                }
                OP_CALL_NODE => {
                    let kind = self.bytecode[self.ip];
                    self.ip += 1;
                    let kind = NodeKind::from(kind);
                    self.call_node(kind);
                }
                OP_RET => unimplemented!("OP_RET not implemented"),
                OP_END => break,
                _ => unimplemented!("Invalid instruction")
            }
        }
    }

    pub fn call_node(&mut self, kind: NodeKind) {
        match kind {
            NodeKind::Int => {},
            NodeKind::Add => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                let result = a + b;
                self.stack.push(result);
            }
            NodeKind::Negate => {
                let a = self.stack.pop().unwrap();
                let result = -a;
                self.stack.push(result);
            }
            NodeKind::Switch => {
                unimplemented!();
            }
            NodeKind::Frame => {
                self.stack.push(42);
            }
        }
    }
}
