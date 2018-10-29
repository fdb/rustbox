use super::{NodeId, PortIndex};

pub struct Connection {
    pub output_id: NodeId,
    pub output_port: PortIndex,
    pub input_id: NodeId,
    pub input_port: PortIndex,
}

impl Connection {
    pub fn new(
        output_id: NodeId,
        output_port: PortIndex,
        input_id: NodeId,
        input_port: PortIndex,
    ) -> Connection {
        Connection {
            output_id,
            output_port,
            input_id,
            input_port,
        }
    }
}
