//! The main storage for values travelling through the system.
//! We have support for singular as well as composite values.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum ValueKind {
    Int,
    Float,
    String,
}

impl From<u8> for ValueKind {
    fn from(kind: u8) -> ValueKind {
        match kind {
            1 => ValueKind::Int,
            2 => ValueKind::Float,
            3 => ValueKind::String,
            _ => panic!("Invalid ValueKind {}", kind),
        }
    }
}

impl Into<u8> for ValueKind {
    fn into(self) -> u8 {
        match self {
            ValueKind::Int => 1,
            ValueKind::Float => 2,
            ValueKind::String => 3,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    Int(i32),
    Float(f32),
    String(String),
    IntList(Vec<i32>),
    FloatList(Vec<f32>),
    StringList(Vec<String>),
}

impl Value {
    pub fn len(&self) -> usize {
        match self {
            Value::Int(_) | Value::Float(_) | Value::String(_) => 1,
            Value::IntList(spread) => spread.len(),
            Value::FloatList(spread) => spread.len(),
            Value::StringList(spread) => spread.len(),
        }
    }

    pub fn get_int(&self, index: usize) -> i32 {
        match self {
            Value::Int(v) => *v,
            Value::Float(v) => *v as i32,
            Value::String(_) => 0,
            Value::IntList(v) => v[index % v.len()],
            Value::FloatList(v) => v[index % v.len()] as i32,
            Value::StringList(_) => 0,
        }
    }
}
