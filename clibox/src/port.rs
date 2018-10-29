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
pub enum PortValues {
    Int(Vec<i32>),
    Float(Vec<f32>),
    String(Vec<String>),
}

#[derive(Clone, Debug)]
pub struct PortSlice {
    kind: PortKind,
    values: PortValues,
}

impl PortSlice {
    pub fn new_empty(kind: PortKind) -> PortSlice {
        let values = match kind {
            PortKind::Int => PortValues::Int(Vec::new()),
            PortKind::Float => PortValues::Float(Vec::new()),
            PortKind::String => PortValues::String(Vec::new()),
        };
        PortSlice { kind, values }
    }

    pub fn new_single(kind: PortKind) -> PortSlice {
        let values = match kind {
            PortKind::Int => PortValues::Int(vec![0]),
            PortKind::Float => PortValues::Float(vec![0.0]),
            PortKind::String => PortValues::String(vec!["".to_owned()]),
        };
        PortSlice { kind, values }
    }

    pub fn new_int_slice(values: Vec<i32>) -> PortSlice {
        PortSlice {
            kind: PortKind::Int,
            values: PortValues::Int(values),
        }
    }

    pub fn new_float_slice(values: Vec<f32>) -> PortSlice {
        PortSlice {
            kind: PortKind::Float,
            values: PortValues::Float(values),
        }
    }

    pub fn new_string_slice(values: Vec<&str>) -> PortSlice {
        let values = values.iter().map(|s| s.to_owned().to_string()).collect();
        PortSlice {
            kind: PortKind::String,
            values: PortValues::String(values),
        }
    }

    pub fn size(&self) -> usize {
        // If performance is a bottleneck we could transmute the enum to any kind of vector and ask the size.
        match &self.values {
            PortValues::Int(vals) => vals.len(),
            PortValues::Float(vals) => vals.len(),
            PortValues::String(vals) => vals.len(),
        }
    }

    pub fn get_int(&self, index: usize) -> i32 {
        match &self.values {
            PortValues::Int(vals) => vals[index % vals.len()],
            PortValues::Float(vals) => vals[index % vals.len()] as i32,
            PortValues::String(_) => 0,
        }
    }

    pub fn get_float(&self, index: usize) -> f32 {
        match &self.values {
            PortValues::Int(vals) => vals[index % vals.len()] as f32,
            PortValues::Float(vals) => vals[index % vals.len()],
            PortValues::String(_) => 0.0,
        }
    }

    pub fn get_string(&self, index: usize) -> String {
        match &self.values {
            PortValues::Int(vals) => format!("{}", vals[index % vals.len()]),
            PortValues::Float(vals) => format!("{}", vals[index % vals.len()]),
            PortValues::String(vals) => vals[index % vals.len()].to_owned(),
        }
    }

    pub fn ensure_size(&mut self, new_size: usize) {
        match &mut self.values {
            PortValues::Int(vals) => {
                if new_size > vals.len() {
                    vals.resize(new_size, 0)
                }
            }
            PortValues::Float(vals) => {
                if new_size > vals.len() {
                    vals.resize(new_size, 0.0)
                }
            }
            PortValues::String(vals) => {
                if new_size > vals.len() {
                    vals.resize(new_size, "".to_owned())
                }
            }
        }
    }

    pub fn set_int(&mut self, index: usize, v: i32) {
        self.ensure_size(index + 1);
        match &mut self.values {
            PortValues::Int(vals) => vals[index] = v,
            PortValues::Float(vals) => vals[index] = v as f32,
            PortValues::String(vals) => vals[index] = format!("{}", v),
        }
    }

    pub fn set_float(&mut self, index: usize, v: f32) {
        self.ensure_size(index + 1);
        match &mut self.values {
            PortValues::Int(vals) => vals[index] = v as i32,
            PortValues::Float(vals) => vals[index] = v,
            PortValues::String(vals) => vals[index] = format!("{}", v),
        }
    }

    pub fn set_string(&mut self, index: usize, v: &str) {
        // FIXME: Add parsing here?
        self.ensure_size(index + 1);
        match &mut self.values {
            PortValues::Int(vals) => vals[index] = 0,
            PortValues::Float(vals) => vals[index] = 0.0,
            PortValues::String(vals) => vals[index] = v.to_owned(),
        }
    }
}

pub struct Port {
    pub name: String,
    pub kind: PortKind,
    pub slice: PortSlice,
    pub direction: PortDirection,
}

impl Port {
    pub fn new_input(name: &str, kind: PortKind) -> Port {
        Port::new(name, kind, PortDirection::In)
    }

    pub fn new_int_port(name: &str, values: Vec<i32>, direction: PortDirection) -> Port {
        Port {
            name: name.to_owned(),
            kind: PortKind::Int,
            slice: PortSlice::new_int_slice(values),
            direction,
        }
    }

    pub fn new_float_port(name: &str, values: Vec<f32>, direction: PortDirection) -> Port {
        Port {
            name: name.to_owned(),
            kind: PortKind::Float,
            slice: PortSlice::new_float_slice(values),
            direction,
        }
    }

    pub fn new_string_port(name: &str, values: Vec<&str>, direction: PortDirection) -> Port {
        Port {
            name: name.to_owned(),
            kind: PortKind::Float,
            slice: PortSlice::new_string_slice(values),
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
            slice: PortSlice::new_single(kind),
            direction,
        }
    }

    pub fn size(&self) -> usize {
        self.slice.size()
    }

    pub fn get_int(&self, index: usize) -> i32 {
        self.slice.get_int(index)
    }

    pub fn get_float(&self, index: usize) -> f32 {
        self.slice.get_float(index)
    }

    pub fn get_string(&self, index: usize) -> String {
        self.slice.get_string(index)
    }

    pub fn ensure_size(&mut self, new_size: usize) {
        self.slice.ensure_size(new_size)
    }

    pub fn set_int(&mut self, index: usize, v: i32) {
        self.slice.set_int(index, v);
    }

    pub fn set_float(&mut self, index: usize, v: f32) {
        self.slice.set_float(index, v);
    }

    pub fn set_string(&mut self, index: usize, v: &str) {
        self.slice.set_string(index, v);
    }
}
