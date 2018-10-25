pub enum PortPolarity {
    In,
    Out,
    InOut,
}

pub enum PortKind {
    Int,
    Float,
    String,
}

pub enum PortValue {
    Int(i32),
    Float(f32),
    String(String),
}

pub struct Port {
    pub name: String,
    pub value: PortValue,
    pub polarity: PortPolarity,
}

impl Port {
    pub fn default_value(kind: PortKind) -> PortValue {
        match kind {
            PortKind::Int => PortValue::Int(0),
            PortKind::Float => PortValue::Float(0.0),
            PortKind::String => PortValue::String("".to_owned())
        }
    }

    pub fn new_input(name: &str, kind: PortKind) -> Port {
        Port::new(name, kind, PortPolarity::In)
    }

    pub fn new_output(name: &str, kind: PortKind) -> Port {
        Port::new(name, kind, PortPolarity::InOut)
    }

    pub fn new(name: &str, kind: PortKind, polarity: PortPolarity) -> Port {
        Port {
            name: name.to_owned(),
            value: Port::default_value(kind),
            polarity
        }
    }
}
