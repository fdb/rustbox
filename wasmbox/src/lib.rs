use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn appendNumberToBody(x: u32);
    fn appendStringToBody(s: &str);
    fn glClear(r: f32, g: f32, b: f32, a: f32);
    fn alert(msg: u32);
}

#[wasm_bindgen]
pub enum PlugPolarity {
    In,
    Out,
    InOut,
}

#[wasm_bindgen]
pub enum PlugKind {
    Int,
    Float,
    String,
}

pub enum PlugValue {
    Int(i32),
    Float(f32),
    String(String),
}

#[wasm_bindgen]
pub struct Plug {
    pub name: String,
    pub value: PlugValue,
    pub polarity: PlugPolarity,
}

impl Plug {

    fn default_value(kind: PlugKind) -> PlugValue {
        match kind {
            PlugKind::Int => PlugValue::Int(0),
            PlugKind::Float => PlugValue::Float(0.0),
            PlugKind::String => PlugValue::String("".to_owned())
        }
    }

    fn new_input(name: &str, kind: PlugKind) -> Plug {
        Plug::new(name, kind, PlugPolarity::In)
    }

    fn new_output(name: &str, kind: PlugKind) -> Plug {
        Plug::new(name, kind, PlugPolarity::InOut)
    }

    fn new(name: &str, kind: PlugKind, polarity: PlugPolarity) -> Plug {
        Plug {
            name: name.to_owned(),
            value: Plug::default_value(kind),
            polarity
        }
    }

}


#[wasm_bindgen]
pub struct Node {
    pub name: String,
    pub plugs: Vec<Plug>,
}

impl Node {
    fn new(name: &str) -> Node {
        let mut plugs = Vec::new();
        plugs.push(Plug::new_input("a", PlugKind::Float));
        plugs.push(Plug::new_input("b", PlugKind::Float));
        plugs.push(Plug::new_output("out", PlugKind::Float));
        Node {
            name: name.to_owned(),
            plugs
        }
    }
}


#[wasm_bindgen]
pub fn add_one(x: u32) -> u32 {
    x + 1
}


#[wasm_bindgen]
pub fn run() {
    for number in 1..10 {
        appendNumberToBody(number);
    }
    glClear(1.0, 0.0, 1.0, 1.0);
    // alert(666);
    appendStringToBody("Hello!");
}

#[wasm_bindgen]
pub fn create_node(name: &str) -> Node {
    Node::new(name)
}
