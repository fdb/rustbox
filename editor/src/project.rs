use std::collections::HashMap;

pub type ValueMap = HashMap<String, ParameterValue>;

pub enum NodeType {
    Clear,
    Quad,
}

pub enum ParameterKind {
    Int,
    Float,
    Vec2,
    Vec3,
    Color,
    String,
}

pub struct Parameter {
    pub name: String,
    pub kind: ParameterKind,
}

pub enum ParameterValue {
    Int(i32),
    Float(f32),
    Vec2(f32, f32),
    Vec3(f32, f32, f32),
    Color(u32),
    String(String),
}

pub enum FunctionKind {
    Code,
    Network,
}

pub trait Function {
    fn kind() -> FunctionKind;
    fn name(&self) -> &str;
    // fn parameters(&self) -> Vec<Parameter>;
    fn category(&self) -> &str;
    fn eval(&self, values: &ValueMap) -> ParameterValue;
}

#[repr(C)]
pub struct Node {
    pub id: i32,
    pub name: String,
    pub position: (f32, f32),
    pub function: String,
    pub values: HashMap<String, ParameterValue>
}

#[repr(C)]
pub struct Network {
    name: String,
    category: String,
    nodes: Vec<Node>,
    rendered_node: i32,
}

impl Function for Network {
    fn kind() -> FunctionKind { FunctionKind::Network }
    fn name(&self) -> &str { &self.name }
    fn category(&self) -> &str { &self.category }

    fn eval(&self, values: &ValueMap) -> ParameterValue {
        ParameterValue::Int(42)
    }
}

#[repr(C)]
pub struct Code {
}

impl Function for Code {
    fn kind() -> FunctionKind { FunctionKind::Code }
    fn name(&self) -> &str { "add" }
    fn category(&self) -> &str { "math" }

    fn eval(&self, values: &ValueMap) -> ParameterValue {
        ParameterValue::Int(33)
    }
}

#[repr(C)]
pub struct Project {
    name: String,
    networks: Vec<Network>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
