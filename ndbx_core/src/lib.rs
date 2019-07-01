use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

pub trait Fn {
    fn eval(&self, Vec<&Value>) -> Value;
}

#[derive(Debug)]
struct AddFn {}
impl Fn for AddFn {
    fn eval(&self, values: Vec<&Value>) -> Value {
        let a = values[0].get_float();
        let b = values[1].get_float();
        println!("Add {} + {}", a, b);
        Value::Float(a + b)
    }
}

#[derive(Debug)]
struct MakeNumbersFn {}
impl Fn for MakeNumbersFn {
    fn eval(&self, values: Vec<&Value>) -> Value {
        let mut results = Vec::new();
        let s = values[0].get_str();
        let sep = values[1].get_str();
        for part in s.split(sep) {
            results.push(Value::String(String::from(part)));
        }
        Value::List(results)
    }
}

#[derive(Debug)]
pub enum ParameterType {
    Int,
    Float,
    String,
    Color,
}

#[derive(Debug)]
pub enum Value {
    Int(i32),
    Float(f32),
    String(String),
    Color(f32, f32, f32, f32),
    List(Vec<Value>),
}

impl Clone for Value {
    fn clone(&self) -> Value {
        match self {
            &Value::Int(v) => Value::Int(v),
            &Value::Float(v) => Value::Float(v),
            &Value::String(ref v) => Value::String(v.to_string()),
            &Value::Color(r, g, b, a) => Value::Color(r, g, b, a),
            &Value::List(ref v) => Value::List(v.clone()),
        }
    }
}

impl Value {
    pub fn get_int(&self) -> i32 {
        match self {
            &Value::Int(v) => v,
            &Value::Float(v) => v as i32,
            &Value::String(_) => 0,
            &Value::Color(_, _, _, _) => 0,
            &Value::List(_) => 0,
        }
    }

    pub fn get_float(&self) -> f32 {
        match self {
            &Value::Int(v) => v as f32,
            &Value::Float(v) => v,
            &Value::String(_) => 0.,
            &Value::Color(_, _, _, _) => 0.,
            &Value::List(_) => 0.,
        }
    }

    pub fn get_str(&self) -> &str {
        // We can't convert the values for strings because we're returning string references,
        // and there is no sensible way to set the lifetimes.
        // If you want to see the value as a string, use the display trait
        match self {
            &Value::Int(_) => "",
            &Value::Float(_) => "",
            &Value::String(ref v) => v.as_str(),
            &Value::Color(_, _, _, _) => "",
            &Value::List(_) => "[]",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Int(v) => write!(f, "{}", v),
            &Value::Float(v) => write!(f, "{}", v),
            &Value::String(ref v) => write!(f, "{}", v),
            &Value::Color(r, g, b, a) => write!(f, "({}, {}, {}, {})", r, g, b, a),
            &Value::List(_) => write!(f, "[]"),
        }
    }
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub kind: ParameterType,
    pub value: Value,
}

pub struct Function {
    pub name: String,
    pub op: Box<Fn>,
    pub parameters: Vec<Parameter>,
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Function {:?}", self.name)
    }
}

#[derive(Debug)]
pub struct FunctionRegistry {
    pub functions: HashMap<String, Function>,
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub function: String,
    pub values: HashMap<String, Value>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.name == other.name
    }
}

#[derive(Debug)]
pub struct Connection {
    pub output: String,
    pub input: String,
    pub port: String,
}

#[derive(Debug)]
pub struct Network {
    pub name: String,
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
}

#[derive(Debug)]
pub struct Context {}

impl Context {
    pub fn new() -> Context {
        Context {}
    }

    pub fn find_node<'a>(&self, network: &'a Network, name: &str) -> &'a Node {
        for node in &network.nodes {
            if node.name == name {
                return &node;
            }
        }
        panic!("No node found");
    }

    pub fn find_connections_by_output<'a>(
        &self,
        network: &'a Network,
        node: &'a Node,
    ) -> Vec<&'a Connection> {
        let mut results = Vec::new();
        for conn in &network.connections {
            if conn.output.as_str() == node.name {
                results.push(conn);
            }
        }
        results
    }

    pub fn find_connections_by_input<'a>(
        &self,
        network: &'a Network,
        node: &'a Node,
    ) -> Vec<&'a Connection> {
        let mut results = Vec::new();
        for conn in &network.connections {
            if conn.input.as_str() == node.name {
                results.push(conn);
            }
        }
        results
    }

    pub fn find_connection_by_input<'a>(
        &self,
        network: &'a Network,
        node: &'a Node,
        param: &str,
    ) -> Option<&'a Connection> {
        for conn in &network.connections {
            if conn.input.as_str() == node.name && conn.port.as_str() == param {
                return Some(conn);
            }
        }
        None
    }

    pub fn network_evaluation_order<'a>(
        &self,
        network: &'a Network,
        node: &'a Node,
        order: &mut Vec<&'a Node>,
    ) {
        let conns = self.find_connections_by_input(network, node);
        for conn in conns {
            let output_node = self.find_node(network, conn.output.as_str());
            println!("on {:?}", output_node);
            self.network_evaluation_order(network, output_node, order);
        }
        if !order.contains(&node) {
            order.push(node);
        }
    }

    pub fn render(
        &self,
        function_registry: &FunctionRegistry,
        network: &Network,
        node: &Node,
    ) -> Value {
        let mut result_map: HashMap<&str, Value> = HashMap::new();

        let mut order: Vec<&Node> = Vec::new();
        self.network_evaluation_order(network, node, &mut order);
        println!("ORDER: {:?}", order);

        for node in order {
            let result = {
                let function = &function_registry.functions[node.function.as_str()];
                let mut arg_lists: Vec<&Value> = Vec::new();
                arg_lists.reserve(function.parameters.len());
                for param in &function.parameters {
                    // Check if it's connected
                    let conn = self.find_connection_by_input(network, node, param.name.as_str());
                    if conn.is_some() {
                        let value = &result_map[conn.unwrap().output.as_str()];
                        arg_lists.push(value);
                    } else {
                        let value = match node.values.get(param.name.as_str()) {
                            Some(v) => &v,
                            None => &param.value,
                        };
                        arg_lists.push(value);
                    }
                }
                println!("fn {:?} args {:?}", function, arg_lists);
                function.op.eval(arg_lists)
            };

            println!("Node {:?} Result {:?}", node.name, result);

            result_map.insert(node.name.as_str(), result);
        }

        result_map.remove(node.name.as_str()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn make_network() {
        let a_param = Parameter {
            name: String::from("a"),
            kind: ParameterType::Float,
            value: Value::Float(0.),
        };
        let b_param = Parameter {
            name: String::from("b"),
            kind: ParameterType::Float,
            value: Value::Float(0.),
        };
        let add_parameters = vec![a_param, b_param];
        let add_fn = Function {
            name: String::from("add"),
            op: Box::new(AddFn {}),
            parameters: add_parameters,
        };

        let s_param = Parameter {
            name: String::from("s"),
            kind: ParameterType::String,
            value: Value::String(String::from("1;2;3")),
        };
        let sep_param = Parameter {
            name: String::from("sep"),
            kind: ParameterType::String,
            value: Value::String(String::from(";")),
        };
        let make_numbers_parameters = vec![s_param, sep_param];
        let make_numbers_fn = Function {
            name: String::from("make_numbers"),
            op: Box::new(AddFn {}),
            parameters: make_numbers_parameters,
        };

        let mut function_map = HashMap::new();
        function_map.insert(add_fn.name.clone(), add_fn);
        function_map.insert(make_numbers_fn.name.clone(), make_numbers_fn);
        let function_registry = FunctionRegistry {
            functions: function_map,
        };

        let mut add1_vals = HashMap::new();
        add1_vals.insert(String::from("a"), Value::Float(3.));
        add1_vals.insert(String::from("b"), Value::Float(5.));
        let add1 = Node {
            name: String::from("add1"),
            function: String::from("add"),
            values: add1_vals,
        };
        println!("add1 node: {:?}", add1);

        let mut make_numbers1_vals = HashMap::new();
        make_numbers1_vals.insert(String::from("s"), Value::String(String::from("11;22;33")));
        let make_numbers1 = Node {
            name: String::from("make_numbers1"),
            function: String::from("make_numbers"),
            values: make_numbers1_vals,
        };

        let c1 = Connection {
            output: String::from("make_numbers1"),
            input: String::from("add1"),
            port: String::from("a"),
        };
        let net = Network {
            name: String::from("main"),
            nodes: vec![make_numbers1, add1],
            connections: vec![c1],
        };

        let ctx = Context {};
        let result = ctx.render(&function_registry, &net, &net.nodes[1]);
        let result2 = result.clone();
        println!("Result: {:?}", result);
        println!("Result.to_string: {}", result.to_string());
    }
}
