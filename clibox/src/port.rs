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

    pub fn to_int(&self) -> i32 {
        match self.value {
            PortValue::Int(v) => v,
            PortValue::Float(v) => v as i32,
            PortValue::String(_) => 0,
        }
    }

    pub fn to_float(&self) -> f32 {
        match self.value {
            PortValue::Int(v) => v as f32,
            PortValue::Float(v) => v,
            PortValue::String(_) => 0.0,
        }
    }

    pub fn to_string(&self) -> String {
        match self.value {
            PortValue::Int(v) => format!("{}", v),
            PortValue::Float(v) => format!("{}", v),
            PortValue::String(v) => v.clone(),
        }
    }

    pub fn set_int(&mut self, v: i32) {
        match self.value {
            PortValue::Int(_) => self.value = PortValue::Int(v),
            PortValue::Float(_) => self.value = PortValue::Float(v as f32),
            PortValue::String(_) => self.value = PortValue::String(format!("{}", v)),
        }
    }

    pub fn set_float(&mut self, v: f32) {
        match self.value {
            PortValue::Int(_) => self.value = PortValue::Int(v as i32),
            PortValue::Float(_) => self.value = PortValue::Float(v),
            PortValue::String(_) => self.value = PortValue::String(format!("{}", v)),
        }
    }

    pub fn set_string(&mut self, s: &str) {
        match self.value {
            PortValue::Int(_) => self.value = PortValue::Int(0),
            PortValue::Float(_) => self.value = PortValue::Float(0.0),
            PortValue::String(_) => self.value = PortValue::String(s.to_owned()),
        }
    }
}
