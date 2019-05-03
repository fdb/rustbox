use crate::network::{is_time_dependent, Network, Node};

const SVG_ROW_WIDTH: i32 = 80;
const SVG_COLUMN_HEIGHT: i32 = 20;
const SVG_PORT_WIDTH: i32 = 10;

fn node_to_svg(network: &Network, node: &Node) -> String {
    let mut s = String::new();
    s += &format!(
        r#"<g class="node" id="{}" transform="translate({}, {})">"#,
        node.name,
        node.x * SVG_ROW_WIDTH,
        node.y * SVG_COLUMN_HEIGHT
    );
    s += &format!(r#"<rect class="node__rect" x="2" y="2" width="{}" height="{}" fill="none" stroke="black"/>"#, SVG_ROW_WIDTH - 4, SVG_COLUMN_HEIGHT - 4);
    s += &format!(
        r#"<text class="node__name" x="5" y="14" font-size="12">{}</text>"#,
        node.name
    );
    let inputs = node.kind.inputs();
    for (index, _) in inputs.iter().enumerate() {
        s += &format!(
            r#"<rect class="node__rect" x="{}" y="0" width="{}" height="2" fill="black"/>"#,
            2 + index as i32 * SVG_PORT_WIDTH,
            SVG_PORT_WIDTH - 2
        );
    }
    s += &format!(
        r#"<rect class="node__rect" x="2" y="{}" width="{}" height="2" fill="black"/>"#,
        SVG_COLUMN_HEIGHT - 2,
        SVG_PORT_WIDTH - 2
    );
    if is_time_dependent(node.kind) {
        s += &format!(
            r#"<circle class="node__time" cx="{}" cy="{}" r="3" fill="red"/>"#,
            SVG_ROW_WIDTH - 10,
            SVG_COLUMN_HEIGHT / 2
        );
    } else if network.is_time_dependent(node) {
        s += &format!(
            r#"<circle class="node__time" cx="{}" cy="{}" r="3" fill="none" stroke="red"/>"#,
            SVG_ROW_WIDTH - 10,
            SVG_COLUMN_HEIGHT / 2
        );
    }

    s += "</g>";
    s
}

pub fn network_to_svg(network: &Network) -> String {
    let mut s = String::new();
    s += &format!(r#"<g class="network" id="{}">"#, network.name);
    for node in &network.nodes {
        s += &node_to_svg(network, node);
    }
    for conn in &network.connections {
        let output_node = network
            .nodes
            .iter()
            .find(|&node| node.name == conn.output)
            .unwrap();
        let input_node = network
            .nodes
            .iter()
            .find(|&node| node.name == conn.input)
            .unwrap();
        let port_index = input_node.kind.port_index(&conn.port);
        if port_index.is_none() {
            continue;
        }
        let port_index = port_index.unwrap();
        s += &format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black"/>"#,
            output_node.x * SVG_ROW_WIDTH + 7,
            (output_node.y + 1) * SVG_COLUMN_HEIGHT,
            input_node.x * SVG_ROW_WIDTH + 7 + (port_index as i32 * SVG_PORT_WIDTH),
            input_node.y * SVG_COLUMN_HEIGHT
        );
    }
    s += "</g>";
    s
}
