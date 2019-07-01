mod project;
extern crate libc;

use std::boxed::Box;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::ptr;
use libc::c_void;
use project::{Project,Network, Node, NodeType};

#[repr(C)]
pub struct Editor {
    project: Project,
    current_network_index: i32,
    node_selection: Vec<u32>,
}

impl Editor {
    pub fn old_new(project: Project) -> Editor {
        Editor { project, current_network_index: 0, node_selection: vec![0] }
    }

    pub fn new() -> Editor {
        let clear_node = Node { x: 1, y: 1, name: "clear1".to_owned(), kind: NodeType::Clear };
        let quad_node = Node { x: 1, y: 3, name: "quad1".to_owned(), kind: NodeType::Quad };
        let network = Network { name: "main".to_owned(), nodes: vec![clear_node, quad_node], rendered_node: "quad1".to_owned() };
        let project = Project { name: "test".to_owned(), networks: vec![network] };
        Editor { project, current_network_index: 0, node_selection: vec![0] }
    }


    pub fn current_network(&self) -> Option<&Network> {
        self.project.networks.get(self.current_network_index as usize)
    }

    pub fn selected_nodes(&self) -> SelectedNodeIter {
        SelectedNodeIter { editor: self, index: 0 }
    }
}

pub struct SelectedNodeIter<'a> {
    editor: &'a Editor,
    index: usize,
}

impl<'a> Iterator for SelectedNodeIter<'a> {
    type Item = &'a Node;

    fn next (&mut self) -> Option<&'a Node> {
        if let Some(network) = self.editor.current_network() {
            if let Some(node_index) = self.editor.node_selection.get(self.index) {
                if let Some(node) = network.nodes.get(*node_index as usize) {
                    self.index += 1;
                    return Some(node);
                }
            }
        }
        return None;
    }
}



// pub struct Code {
//     name: String,
//     source: String,
// }

#[no_mangle]
pub extern "C" fn nbe_editor_create() -> Box<Editor> {
    let clear_node = Node { x: 1, y: 1, name: "clear1".to_owned(), kind: NodeType::Clear };
    let quad_node = Node { x: 1, y: 3, name: "quad1".to_owned(), kind: NodeType::Quad };
    let network = Network { name: "main".to_owned(), nodes: vec![clear_node, quad_node], rendered_node: "quad1".to_owned() };
    let project = Project { name: "test".to_owned(), networks: vec![network] };
    Box::new(Editor::old_new(project))
}

#[no_mangle]
pub extern "C" fn nbe_editor_project_get(editor: &Editor) -> &Project {
    &editor.project
}

#[no_mangle]
pub extern "C" fn nbe_editor_node_is_selected(editor: &Editor, node: &Node) -> bool {
    let network = editor.current_network();
    if network.is_none() {
        return false;
    }
    for n in editor.selected_nodes() {
        if ptr::eq(n, node) {
            return true;
        }
    }
    false
}

#[no_mangle]
pub extern "C" fn nbe_project_networks_count(project: &Project) -> u32 {
    project.networks.len() as u32
}

#[no_mangle]
pub extern "C" fn nbe_project_networks_create(project: &mut Project, c_name: *const c_char) -> &Network {
    let name = unsafe { CStr::from_ptr(c_name).to_string_lossy().into_owned() };
    println!("R: name: {}", name);
    project.networks.push(Network{name, nodes: vec![], rendered_node: "".to_owned()});
    &project.networks.last().unwrap()
}

#[no_mangle]
pub extern "C" fn nbe_project_networks_get(project: &Project, index: u32) -> &Network {
    &project.networks[index as usize]
}

#[no_mangle]
pub extern "C" fn nbe_network_nodes_count(network: &Network) -> u32 {
    network.nodes.len() as u32
}

#[no_mangle]
pub extern "C" fn nbe_network_nodes_get(network: &Network, index: u32) -> &Node {
    &network.nodes[index as usize]
}

#[no_mangle]
pub extern "C" fn nbe_network_name(network: &Network) -> *mut c_char {
    CString::new(network.name.clone()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn nbe_node_name(node: &Node) -> *mut c_char {
    CString::new(node.name.clone()).unwrap().into_raw()
}


#[no_mangle]
pub extern "C" fn editor_create() -> *mut c_void {
    Box::into_raw(Box::new(Editor::new())) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn editor_free(editor: *mut c_void) {
    Box::from_raw(editor as *mut Editor);
}

fn editor_deref<'a>(editor: *mut c_void) -> &'a mut Editor {
    unsafe { &mut *(editor as *mut Editor) }
}

#[no_mangle]
pub extern "C" fn editor_network_count(editor: *mut c_void) -> i32 {
    let editor = editor_deref(editor);
    editor.project.networks.len() as i32
}

#[no_mangle]
pub extern "C" fn editor_network_name(editor: *mut c_void, index: u32) -> *mut c_char {
    let editor = editor_deref(editor);
    let name = &editor.project.networks[index as usize].name;
    let s = CString::new(name.clone()).unwrap();
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn editor_add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
     use super::*;

    #[test]
    fn test_load() {
        let editor = nbe_editor_create();
        let project = &editor.project;
        assert_eq!(project.networks.len(), 1);
        let network = &project.networks[0];
        assert_eq!(network.nodes.len(), 2);
        assert_eq!(network.rendered_node, "quad1");
    }
}
