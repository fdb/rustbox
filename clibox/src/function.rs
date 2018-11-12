use super::{Node, RenderContext};
// use std::collections::HashMap;

pub trait Function {
    fn setup(&self, node: &mut Node);
    fn render(&self, node: &Node, ctx: &mut RenderContext);
}

// pub struct FunctionRepository {
//     functions: HashMap<String, Box<Function>>,
// }

// impl FunctionRepository {
//     pub fn new() -> FunctionRepository {
//         FunctionRepository {
//             functions: HashMap::new(),
//         }
//     }
// }
