use crate::{Network, NodeId, Port, PortIndex, PortSlice};
use std::collections::HashMap;

pub struct RenderContext<'n> {
    pub network: &'n Network,
    pub inputs: HashMap<(NodeId, PortIndex), PortSlice>,
    pub outputs: HashMap<(NodeId, PortIndex), PortSlice>,
}

impl<'n, 'f> RenderContext<'n> {
    pub fn new(network: &'n Network) -> RenderContext<'n> {
        RenderContext {
            network,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    pub fn set_output_floats(&mut self, id: NodeId, output_port: PortIndex, values: Vec<f32>) {
        self.outputs
            .insert((id, output_port), PortSlice::new_float_slice(values));
    }

    pub fn get_output_slice(&mut self, id: NodeId, output_port: PortIndex) -> Option<&PortSlice> {
        self.outputs.get(&(id, output_port))
    }

    pub fn set_output_slice(&mut self, id: NodeId, output_port: PortIndex, slice: PortSlice) {
        self.outputs.insert((id, output_port), slice);
    }

    pub fn get_input_slice(&self, id: NodeId, input_port: PortIndex) -> &PortSlice {
        self.inputs
            .get(&(id, input_port))
            .unwrap_or_else(|| &self.network.get_input_port(id, input_port).unwrap().slice)
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

        node.inputs.iter().enumerate().fold(0, |acc, (i, p)| {
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
            .unwrap_or_else(|| &port.slice)
            .size()
    }
}
