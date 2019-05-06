use crate::bytecode::*;
use crate::network::{NodeKind, Spread, SpreadKind};

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    String(String),
    Spread(Spread),
}

impl Value {
    pub fn len(&self) -> usize {
        match self {
            Value::Int(_) | Value::String(_) => 1,
            Value::Spread(spread) => match spread {
                Spread::Int(v) => v.len(),
                Spread::Float(v) => v.len(),
                Spread::String(v) => v.len(),
            },
        }
    }

    pub fn get_int(&self, index: usize) -> i32 {
        match self {
            Value::Int(v) => *v,
            Value::String(_) => 0,
            Value::Spread(spread) => match spread {
                Spread::Int(v) => v[index % v.len()],
                Spread::Float(v) => v[index % v.len()] as i32,
                Spread::String(_) => 0,
            },
        }
    }
}

pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(message: String) -> RuntimeError {
        RuntimeError { message }
    }
}

pub struct VM {
    pub bytecode: Vec<u8>,
    pub constant_pool: Vec<Value>,
    pub ip: usize,
    pub stack: Vec<Value>,
}

impl VM {
    pub fn new(bytecode: Vec<u8>, constant_pool: Vec<Value>) -> VM {
        VM {
            bytecode,
            constant_pool,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn check_int_value(&self, value: Value) -> Result<i32, RuntimeError> {
        match value {
            Value::Int(v) => Ok(v),
            x => Err(RuntimeError::new(format!(
                "Value is not an int, but {:?}",
                x
            ))),
        }
    }

    pub fn pop_int_value(&mut self) -> Result<i32, RuntimeError> {
        let value = self.stack.pop();
        match value {
            Some(value) => self.check_int_value(value),
            None => Err(RuntimeError::new("Stack underflow".to_string())),
        }
    }

    pub fn pop_value(&mut self) -> Result<Value, RuntimeError> {
        let value = self.stack.pop();
        match value {
            Some(value) => Ok(value),
            None => Err(RuntimeError::new("Stack underflow".to_string())),
        }
    }

    pub fn check_spread_value(&self, value: Value) -> Result<Spread, RuntimeError> {
        match value {
            Value::Spread(v) => Ok(v),
            x => Err(RuntimeError::new(format!(
                "Value is not a spread, but {:?}",
                x
            ))),
        }
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        loop {
            let op = self.bytecode[self.ip];
            self.ip += 1;
            match op {
                OP_CONST_I32 => {
                    let mut value: [u8; 4] = [0; 4];
                    value.copy_from_slice(&self.bytecode[self.ip..self.ip + 4]);
                    self.ip += 4;
                    let value: i32 = unsafe { std::mem::transmute(value) };
                    self.stack.push(Value::Int(value));
                }
                OP_DUP => {
                    let v = self.stack[self.stack.len() - 1].clone();
                    self.stack.push(v);
                }
                OP_SPREAD_NEW => {
                    let spread_kind = self.bytecode[self.ip];
                    self.ip += 1;
                    let spread_kind = SpreadKind::from(spread_kind);
                    let spread = match spread_kind {
                        SpreadKind::Int => Spread::Int(Vec::new()),
                        SpreadKind::Float => Spread::Float(Vec::new()),
                        SpreadKind::String => Spread::String(Vec::new()),
                    };
                    self.stack.push(Value::Spread(spread));
                }
                OP_SPREAD_STORE => {
                    let value = self.stack.pop().unwrap();
                    let index = self.stack.pop().unwrap();
                    let spread = self.stack.pop().unwrap();
                    let value = self.check_int_value(value)?; // FIXME: we can't know this is an index value.
                    let index = self.check_int_value(index)?;
                    let spread = self.check_spread_value(spread)?;
                    //spread[index] = value;
                }
                OP_SPREAD_LOAD => {
                    let index = self.pop_int_value()?;
                    let spread = self.constant_pool.get(index as usize);
                    match spread {
                        Some(spread) => self.stack.push(spread.clone()),
                        None => {
                            return Err(RuntimeError::new(format!(
                                "Invalid constant pool index {}",
                                index
                            )))
                        }
                    };
                }
                OP_CALL_NODE => {
                    let kind = self.bytecode[self.ip];
                    self.ip += 1;
                    let kind = NodeKind::from(kind);
                    self.call_node(kind);
                }
                OP_END => return Ok(()),
                _ => unimplemented!("Invalid instruction"),
            }
        }
    }

    pub fn call_node(&mut self, kind: NodeKind)  -> Result<(), RuntimeError> {
        println!("call_node kind: {:?} stack: {:?}", kind, self.stack);
        match kind {
            NodeKind::Int => {}
            NodeKind::Add => {
                let a = self.pop_value()?;
                let b = self.pop_value()?;
                let max_size = a.len().max(b.len());
                        let mut results = Vec::with_capacity(max_size);
                        for i in 0..max_size {
                            let va = a.get_int(i);
                            let vb = b.get_int(i);
                            results.push(va + vb);
                        }
                        self.stack.push(Value::Spread(Spread::Int(results)));
                // if let (Value::Int(ia), Value::Int(ib)) = (&a, &b) {
                //     let result = ia + ib;
                //     self.stack.push(Value::Int(result));
                // }
                // if let (Value::Spread(ia), Value::Spread(ib)) = (&a, &b) {
                //     if let (Spread::Int(ax), Spread::Int(bx)) = (ia, ib) {
                //         let max_size = ax.len().max(bx.len());
                //         let mut results = Vec::with_capacity(max_size);
                //         for i in 0..max_size {
                //             let a = ax[i % ax.len()];
                //             let b = bx[i % bx.len()];
                //             results.push(a + b);
                //         }
                //         self.stack.push(Value::Spread(Spread::Int(results)));
                //     }
                // }
            }
            NodeKind::Negate => {
                let a = self.stack.pop().unwrap();
                if let Value::Int(ia) = a {
                    let result = -ia;
                    self.stack.push(Value::Int(result));
                }
            }
            NodeKind::Switch => {
                unimplemented!();
            }
            NodeKind::Frame => {
                self.stack.push(Value::Int(42));
            }
        }
        println!("  After: {:?}", self.stack);
        Ok(())
    }
}
