mod bytecode;
mod compiler;
mod network;
mod svg;
mod value;
mod vm;

use std::env;
use std::fs;
use std::fs::File;

use crate::compiler::{compile_network, print_bytecode, print_constant_pool};
use crate::network::Network;
use crate::svg::network_to_svg;
use crate::vm::VM;

fn main() {
    // let mut values = HashMap::new();
    // values.insert("v".to_owned(), Value::Int(42));
    // let int1 = Node {
    //     name: "int1".to_owned(),
    //     x: 3,
    //     y: 5,
    //     kind: NodeKind::Int,
    //     values,
    // };
    // let network = Network { nodes: vec![int1] };

    // let serialized = serde_json::to_string(&network).unwrap();
    // println!("{:?}", serialized);

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run PROJECT_FILE");
        std::process::exit(1);
    }

    let path = &args[1];
    let result = File::open(path);
    if let Err(err) = result {
        println!("Error when opening {}: {}", path, err.to_string());
        return;
    }
    let file = result.unwrap();
    let network: Network = serde_json::from_reader(file).unwrap();

    let mut svg = String::new();
    svg += r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 600">"#;
    svg += &network_to_svg(&network);
    svg += r#"</svg>"#;
    // println!("{}", svg);
    fs::write("out.svg", svg).unwrap();
    //let file = File::create("out.svg").unwrap();
    //file.write_all(svg);

    // println!("{}", network_to_svg(&network));

    //serde_json::from_reader(rdr: R)

    let result = compile_network(&network).unwrap();
    println!("Bytecode: {:?}", result.bytecode);
    println!("Constants:");
    print_constant_pool(&result.constant_pool);
    println!("========================");
    println!("Instructions:");
    print_bytecode(&result.bytecode);
    println!("========================");

    let mut vm = VM::new(result.bytecode, result.constant_pool);
    let result = vm.run();
    if result.is_err() {
        println!("ERROR: {:?}", result.unwrap_err().message);
    }
    println!("STACK: {:?}", vm.stack);

    // let spread = Spread::Int(vec![1, 2, 3, 4]);
    // // spread.to_json
    // let serialized = serde_json::to_string(&spread).unwrap();
    // println!("{:?}", serialized);
}
