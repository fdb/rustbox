
use super::NodeId;

pub struct Connection {
    pub output_id: NodeId, 
    pub output_port: String,
    pub input_id: NodeId,
    pub input_port: String,
}

impl Connection {
    pub fn new(output_id: NodeId, output_port: String, input_id: NodeId, input_port: String) -> Connection {
        Connection { output_id, output_port, input_id, input_port }
    }
}