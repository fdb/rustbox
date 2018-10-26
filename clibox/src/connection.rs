
pub struct Connection {
    pub output_id: usize, 
    pub output_port: String,
    pub input_id: usize,
    pub input_port: String,
}

impl Connection {
    pub fn new(output_id: usize, output_port: String, input_id: usize, input_port: String) -> Connection {
        Connection { output_id, output_port, input_id, input_port }
    }
}