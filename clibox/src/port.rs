pub enum PortDirection {
    In,
    Out,
}

#[derive(Debug, Copy, Clone)]
pub enum PortKind {
    Int,
    Float,
    String,
}

#[derive(Clone, Debug)]
pub enum PortValue {
    Int(i32),
    Float(f32),
    String(String),
}

impl PortValue {
    pub fn to_int(&self) -> i32 {
        match self {
            &PortValue::Int(v) => v,
            &PortValue::Float(v) => v as i32,
            &PortValue::String(_) => 0,
        }
    }

    pub fn to_float(&self) -> f32 {
        match self {
            &PortValue::Int(v) => v as f32,
            &PortValue::Float(v) => v,
            &PortValue::String(_) => 0.0,
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            &PortValue::Int(v) => format!("{}", v),
            &PortValue::Float(v) => format!("{}", v),
            &PortValue::String(v) => v.clone(),
        }
    }
}

pub struct Port {
    pub name: String,
    pub kind: PortKind,
    pub values: Vec<PortValue>,
    pub direction: PortDirection,
}

impl Port {
    pub fn default_value(kind: PortKind) -> PortValue {
        match kind {
            PortKind::Int => PortValue::Int(0),
            PortKind::Float => PortValue::Float(0.0),
            PortKind::String => PortValue::String("".to_owned()),
        }
    }

    pub fn new_input(name: &str, kind: PortKind) -> Port {
        Port::new(name, kind, PortDirection::In)
    }

    pub fn new_int_port(name: &str, values: Vec<i32>, direction: PortDirection) -> Port {
        Port {
            name: name.to_owned(),
            kind: PortKind::Int,
            values: values.iter().map(|v| PortValue::Int(*v)).collect(),
            direction,
        }
    }

    pub fn new_float_port(name: &str, values: Vec<f32>, direction: PortDirection) -> Port {
        Port {
            name: name.to_owned(),
            kind: PortKind::Float,
            values: values.iter().map(|v| PortValue::Float(*v)).collect(),
            direction,
        }
    }

    pub fn new_string_port(name: &str, values: Vec<&str>, direction: PortDirection) -> Port {
        Port {
            name: name.to_owned(),
            kind: PortKind::Float,
            values: values
                .iter()
                .map(|v| PortValue::String(v.to_owned().to_string()))
                .collect(),
            direction,
        }
    }

    pub fn new_output(name: &str, kind: PortKind) -> Port {
        Port::new(name, kind, PortDirection::Out)
    }

    pub fn new(name: &str, kind: PortKind, direction: PortDirection) -> Port {
        Port {
            name: name.to_owned(),
            kind,
            values: vec![Port::default_value(kind)],
            direction,
        }
    }

    pub fn size(&self) -> usize {
        self.values.len()
    }

    pub fn get_int(&self, index: usize) -> i32 {
        let v = &self.values[index % self.values.len()];
        v.to_int()
    }

    pub fn get_float(&self, index: usize) -> f32 {
        let v = &self.values[index % self.values.len()];
        v.to_float()
    }

    pub fn get_string(&self, index: usize) -> String {
        let v = &self.values[index % self.values.len()];
        v.to_string()
    }

    pub fn ensure_size(&mut self, new_size: usize) {
        if new_size > self.values.len() {
            self.values.resize(new_size, Port::default_value(self.kind))
        }
    }

    pub fn set_int(&mut self, index: usize, v: i32) {
        self.ensure_size(index + 1);
        self.values[index] = match self.kind {
            PortKind::Int => PortValue::Int(v),
            PortKind::Float => PortValue::Float(v as f32),
            PortKind::String => PortValue::String(format!("{}", v)),
        }
    }

    pub fn set_float(&mut self, index: usize, v: f32) {
        self.ensure_size(index + 1);
        self.values[index] = match self.kind {
            PortKind::Int => PortValue::Int(v as i32),
            PortKind::Float => PortValue::Float(v),
            PortKind::String => PortValue::String(format!("{}", v)),
        }
    }

    pub fn set_string(&mut self, index: usize, s: &str) {
        self.ensure_size(index + 1);
        self.values[index] = match self.kind {
            PortKind::Int => PortValue::Int(0),
            PortKind::Float => PortValue::Float(0.0),
            PortKind::String => PortValue::String(s.to_owned()),
        }
    }
}
