mod network;
mod svg;

use crate::network::Network;
use crate::svg::network_to_svg;
use std::fs;
use std::fs::File;

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
    let path = "data/graph1.json";
    let file = File::open(path).unwrap();
    let network: Network = serde_json::from_reader(file).unwrap();
    // println!("{:?}", u);

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
}
