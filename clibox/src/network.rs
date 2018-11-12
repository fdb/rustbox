use super::{Connection, Node, NodeId, Port, PortIndex, RenderContext};

pub struct Network {
    pub rendered_id: NodeId,
    pub nodes: Vec<Node>,
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

    pub fn connect(
        &mut self,
        output_id: NodeId,
        output_port: PortIndex,
        input_id: NodeId,
        input_port: PortIndex,
    ) -> Result<(), &'static str> {
        let output_node = self.get_node(output_id);
        if output_node.is_none() {
            return Err("Output node could not be found.");
        }
        let input_node = self.get_node(input_id);
        if input_node.is_none() {
            return Err("Input node could not be found.");
        }
        if self.get_output_port(output_id, output_port).is_none() {
            return Err("Output port could not be found.");
        }
        if self.get_input_port(input_id, input_port).is_none() {
            return Err("Input port could not be found.");
        }
        let conn = Connection::new(output_id, output_port, input_id, input_port);
        self.connections.push(conn);
        Ok(())
    }

    pub fn render(&self, context: &mut RenderContext) -> Result<(), &'static str> {
        let node = self.get_rendered_node();
        if node.is_none() {
            return Err("No rendered node.");
        }
        self.render_node(context, node.unwrap().id)?;
        Ok(())
    }

    fn render_node(&self, context: &mut RenderContext, id: NodeId) -> Result<(), &'static str> {
        let inputs = {
            let node = self.get_node(id);
            // FIXME: check if dirty
            if node.is_none() {
                return Err("Could not find node.");
            }
            &node.unwrap().inputs
        };
        for port_index in 0..inputs.len() {
            self.render_input_port(context, id, port_index)?;
        }
        self.get_node(id).unwrap().render(context);
        Ok(())
    }

    fn render_input_port(
        &self,
        context: &mut RenderContext,
        node_id: NodeId,
        input_port: PortIndex,
    ) -> Result<(), &'static str> {
        let conn = self.get_connection_with_input(node_id, input_port);
        if conn.is_some() {
            let conn = conn.unwrap();
            self.render_node(context, conn.output_id)?;
            context.clone_output_to_input(
                conn.output_id,
                conn.output_port,
                conn.input_id,
                conn.input_port,
            );
        // let values = {
        //     let output_port = self
        //         .get_output_port(conn.output_id, &conn.output_port)
        //         .unwrap();
        //     output_port.values.clone()
        // };

        // FIXME: Save this in RenderContext.
        //let mut input_port = self.get_input_port_mut(node_id, port_name).unwrap();
        //input_port.values = values;
        } else {
            // The port value doesn't need to be updated.
        }
        Ok(())
    }

    pub fn delete_node(&mut self, id: NodeId) {
        self.nodes.retain(|n| n.id != id);
        if self.rendered_id == id {
            self.rendered_id = 0
        }
        // FIXME: also delete the connections
    }

    pub fn get_rendered_node(&self) -> Option<&Node> {
        self.get_node(self.rendered_id)
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    pub fn get_input_port(&self, id: NodeId, input_port: PortIndex) -> Option<&Port> {
        let node = self.get_node(id)?;
        node.get_input(input_port)
    }

    // pub fn get_input_port_mut(&mut self, id: NodeId, port_name: &str) -> Option<&mut Port> {
    //     let node = self.get_node_mut(id)?;
    //     node.get_input_mut(port_name)
    // }

    pub fn get_output_port(&self, id: NodeId, output_port: PortIndex) -> Option<&Port> {
        let node = self.get_node(id)?;
        node.get_output(output_port)
    }

    pub fn get_connection_with_input(
        &self,
        input_id: NodeId,
        input_port: PortIndex,
    ) -> Option<&Connection> {
        self.connections
            .iter()
            .find(|c| c.input_id == input_id && c.input_port == input_port)
    }
}

#[cfg(test)]
mod test {
    // use super::*;

    #[test]
    fn delete_node() {
        //let mut net = Network::new();
        //net.create_node("")
        // pub fn new_node(id: NodeId, type_name: &str, x: i32, y: i32) -> Option<Box<Node>> {
    }
}
