use super::{Network, NodeId, Port, PortIndex, PortValue};
use std::collections::HashMap;

pub struct RenderContext<'a> {
    pub network: &'a Network,
    pub inputs: HashMap<(NodeId, PortIndex), Vec<PortValue>>,
    pub outputs: HashMap<(NodeId, PortIndex), Vec<PortValue>>,
}

impl<'a> RenderContext<'a> {
    pub fn new(network: &'a Network) -> RenderContext<'a> {
        RenderContext {
            network,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    pub fn set_output_floats(&mut self, id: NodeId, output_port: PortIndex, values: Vec<f32>) {
        self.outputs.insert(
            (id, output_port),
            values.iter().map(|v| PortValue::Float(*v)).collect(),
        );
    }

    pub fn set_output_values(
        &mut self,
        id: NodeId,
        output_port: PortIndex,
        values: Vec<PortValue>,
    ) {
        self.outputs.insert((id, output_port), values);
    }

    pub fn get_output_values(
        &mut self,
        id: NodeId,
        output_port: PortIndex,
    ) -> Option<&Vec<PortValue>> {
        self.outputs.get(&(id, output_port))
    }

    pub fn get_input_values(&self, id: NodeId, input_port: PortIndex) -> &Vec<PortValue> {
        self.inputs
            .get(&(id, input_port))
            .unwrap_or_else(|| &self.network.get_input_port(id, input_port).unwrap().values)
    }

    pub fn clone_output_to_input(
        &mut self,
        output_id: NodeId,
        output_port: PortIndex,
        input_id: NodeId,
        input_port: PortIndex,
    ) {
        if let Some(values) = self.outputs.get(&(output_id, output_port)) {
            self.inputs.insert((input_id, input_port), values.clone());
        }
    }

    pub fn get_max_input_size(&self, id: NodeId) -> usize {
        let node = self.network.get_node(id);
        if node.is_none() {
            return 0;
        }
        let node = node.unwrap();

        node.get_inputs().iter().enumerate().fold(0, |acc, (i, p)| {
            let size = self.get_input_size(id, p, i);
            if acc > size {
                acc
            } else {
                size
            }
        })
    }

    pub fn get_input_size(&self, id: NodeId, port: &Port, port_index: PortIndex) -> usize {
        self.inputs
            .get(&(id, port_index))
            .unwrap_or_else(|| &port.values)
            .len()
    }
}
