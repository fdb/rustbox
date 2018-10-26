use super::{Connection, Node, Port};

pub struct Network {
    pub rendered_id: usize,
    pub nodes: Vec<Box<Node>>,
    pub connections: Vec<Connection>,
}

impl Network {
    pub fn new() -> Network {
        Network {
            rendered_id: 0,
            nodes: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn connect(&mut self, output_id: usize, output_port: &str, input_id: usize, input_port: &str) -> Result<(), &'static str> {
        let output_node = self.get_node(output_id);
        if output_node.is_none() { return Err("Output node could not be found."); }
        let input_node = self.get_node(input_id);
        if input_node.is_none() { return Err("Input node could not be found."); }
        if self.get_output_port(output_id, output_port).is_none() { return Err("Output port could not be found."); }
        if self.get_input_port(input_id, input_port).is_none() { return Err("Input port could not be found."); }
        let conn = Connection::new(output_id, output_port.to_string(), input_id, input_port.to_string());
        self.connections.push(conn);
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        let node = self.get_rendered_node();
        if node.is_none() {
            return Err("No rendered node.");
        }
        Ok(())
    }

    pub fn delete_node(&mut self, id: usize) {
        self.nodes.retain(|n| n.get_id() != id);
        if self.rendered_id == id {
            self.rendered_id = 0
        }
        // FIXME: also delete the connections
    }

    pub fn get_rendered_node(&self) -> Option<&Box<Node>> {
        self.get_node(self.rendered_id)
    }

    pub fn get_node(&self, id: usize) -> Option<&Box<Node>> {
        self.nodes.iter().find(|n| n.get_id() == id)
    }

    pub fn get_input_port(&self, id: usize, port_name: &str) -> Option<&Port> {
        let node = self.get_node(id)?;
        node.get_input(port_name)
    }

    pub fn get_output_port(&self, id: usize, port_name: &str) -> Option<&Port> {
        let node = self.get_node(id)?;
        node.get_output(port_name)
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn delete_node() {
        //let mut net = Network::new();
        //net.create_node("")
        // pub fn new_node(id: usize, type_name: &str, x: i32, y: i32) -> Option<Box<Node>> {
    }
}
