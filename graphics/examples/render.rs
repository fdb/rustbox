extern crate gl;
extern crate glutin;
extern crate graphics;
extern crate rand;

use std::ptr;
use std::mem;
use std::iter::repeat;
use std::str;
use std::ffi::CString;
use gl::types::*;
use glutin::VirtualKeyCode;
use graphics::{Mesh, Vector3, Matrix4, Quaternion};
use rand::distributions::{IndependentSample, Range};

fn check_gl_error(msg: &str) {
    unsafe {
        if gl::GetError() != 0 {
            panic!("OpenGL Error: {}", msg)
        }
    }
}

fn check_shader_error(shader: GLuint) {
    let mut result: i32 = 0;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut result);
        if result == 0 {
            println!("Shader compilation failed.");
        }
        let mut info_log_len = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut info_log_len);

        if info_log_len > 0 {
            let mut chars_written = 0;
            let info_log: String = repeat(' ').take(info_log_len as usize).collect();

            let c_str = CString::new(info_log.as_bytes()).unwrap();
            gl::GetShaderInfoLog(
                shader,
                info_log_len,
                &mut chars_written,
                c_str.as_ptr() as *mut _,
            );

            let bytes = c_str.as_bytes();
            let bytes = &bytes[..bytes.len() - 1];
            panic!(
                "Shader compilation failed: {}",
                str::from_utf8(bytes).unwrap()
            );
        }
    }

}

fn main() {
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Graphics Example".to_string())
        .with_dimensions(1280, 720)
        .with_vsync()
        .with_multisampling(16)
        .build(&events_loop)
        .unwrap();

    unsafe { window.make_current() }.unwrap();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);


    // Setup shader program
    let shader_handle: u32;
    let position_attrib: i32;
    let normal_attrib: i32;
    let model_matrix_uniform: i32;
    let view_matrix_uniform: i32;
    let projection_matrix_uniform: i32;
    unsafe {
        shader_handle = gl::CreateProgram();
        let vertex_handle = gl::CreateShader(gl::VERTEX_SHADER);
        let fragment_handle = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(vertex_handle, 1, [VERTEX_SHADER.as_ptr() as *const _].as_ptr(), ptr::null());
        gl::ShaderSource(fragment_handle, 1, [FRAGMENT_SHADER.as_ptr() as *const _].as_ptr(), ptr::null());
        gl::CompileShader(vertex_handle);
        gl::CompileShader(fragment_handle);
        check_shader_error(vertex_handle);
        check_shader_error(fragment_handle);
        gl::AttachShader(shader_handle, vertex_handle);
        gl::AttachShader(shader_handle, fragment_handle);
        gl::LinkProgram(shader_handle);

        // Check the compilation
        check_gl_error("Shader Compilation");

        model_matrix_uniform = gl::GetUniformLocation(shader_handle, b"in_model_matrix\0".as_ptr() as *const _);
        view_matrix_uniform = gl::GetUniformLocation(shader_handle, b"in_view_matrix\0".as_ptr() as *const _);
        projection_matrix_uniform = gl::GetUniformLocation(shader_handle, b"in_projection_matrix\0".as_ptr() as *const _);
        position_attrib = gl::GetAttribLocation(shader_handle, b"in_position\0".as_ptr() as *const _);
        normal_attrib = gl::GetAttribLocation(shader_handle, b"in_normal\0".as_ptr() as *const _);

        check_gl_error("Attrib Location");
    }

    // Buffers
    let mut vertex_buffer_handle: u32;
    let mut vertex_array_handle: u32;
    unsafe {
        vertex_buffer_handle = mem::uninitialized();
        gl::GenBuffers(1, &mut vertex_buffer_handle);

        vertex_array_handle = mem::uninitialized();
        gl::GenVertexArrays(1, &mut vertex_array_handle);
        gl::BindVertexArray(vertex_array_handle);

        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_handle);
        gl::EnableVertexAttribArray(position_attrib as u32);
        gl::VertexAttribPointer(position_attrib as u32, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<f32>() as i32, ptr::null());
        gl::EnableVertexAttribArray(normal_attrib as u32);
        gl::VertexAttribPointer(normal_attrib as u32, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<f32>() as i32, (3 * mem::size_of::<f32>()) as *const () as *const _);

        check_gl_error("Buffers");
    }

    // Mesh
    let mut mesh_list: Vec<Mesh> = Vec::new();
    let between = Range::new(-5., 5.);
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let x = between.ind_sample(&mut rng);
        let y = between.ind_sample(&mut rng);
        let z = between.ind_sample(&mut rng);
        let sz = 0.5;
        mesh_list.push(Mesh::new_box(&Vector3::new(x, y, z), sz, sz, sz));
    }
    let mut points: Vec<f32> = Vec::new();
    for mesh in &mesh_list {
        let mut vertex_points = mesh.to_vertex_data();
        points.append(&mut vertex_points);
    }
    unsafe {
        gl::BindVertexArray(vertex_array_handle);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_handle);
        gl::BufferData(gl::ARRAY_BUFFER, (points.len() * mem::size_of::<f32>()) as isize, points.as_ptr() as *const _, gl::STATIC_DRAW);
    }

    let mut mouse_x = 0;
    let mut mouse_y = 0;
    let mut camera_position = Vector3::new(0., 0., -10.);
    let mut time = 0.0;
    let mut running = true;

    events_loop.run_forever(|event| {
        time += 0.01;
        match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::KeyboardInput(glutin::ElementState::Pressed, _, code, _) => {
                        let key = code.unwrap();
                        match key {
                            VirtualKeyCode::W => { camera_position.z += 0.1; }
                            VirtualKeyCode::S => { camera_position.z -= 0.1; }
                            VirtualKeyCode::A => { camera_position.x -= 0.1; }
                            VirtualKeyCode::D => { camera_position.x += 0.1; }
                            VirtualKeyCode::Q => { camera_position.y += 0.1; }
                            VirtualKeyCode::E => { camera_position.y -= 0.1; }
                            _ => {}
                        }
                        println!("Camera: {:?}", camera_position);
                    }
                    glutin::WindowEvent::MouseMoved(x, y) => {
                        mouse_x = x;
                        mouse_y = y;
                    }
                    glutin::WindowEvent::Closed => { events_loop.interrupt(); running = false; },
                    _ => (),
                }
            }
        }
        if !running { return; }
        unsafe {
            gl::ClearColor(0.95, 0.95, 0.95, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            check_gl_error("Clear");

            gl::Enable(gl::DEPTH_TEST);
            //gl::Enable(gl::CULL_FACE);
            gl::UseProgram(shader_handle);
            let model_matrix = Matrix4::rotate_y_matrix(0.);
            gl::UniformMatrix4fv(model_matrix_uniform, 1, gl::FALSE, model_matrix.m.as_ptr());
            let (wx, wy) = window.get_inner_size_pixels().unwrap();
            let window_width = wx as f32;
            let window_height = wy as f32;
            let cx = ((mouse_x as f32 / window_width) - 0.5) * 2.;
            let cy = ((mouse_y as f32 / window_height) - 0.5) * 2.;

            // http://in2gpu.com/2016/02/26/opengl-fps-camera/
            let camera_pitch = -cy;
            let camera_yaw = cx;

            let q_pitch = Quaternion::with_angle_axis(camera_pitch, &Vector3::new(1., 0., 0.));
            let q_yaw = Quaternion::with_angle_axis(camera_yaw, &Vector3::new(0., 1., 0.));
            let orientation = q_pitch * q_yaw;
            //orientation = orientation.normalize();
            let camera_rotate = orientation.to_matrix();

            let eye = camera_position;

            let target = Vector3::new(cx.sin(), cy, cx.cos());
            //let target = Vector3::new(0.0, 0.0, 0.0);
            let up = Vector3::new(0.0, 1.0, 0.0);
            let camera1 = Matrix4::translate_matrix(camera_position.x, camera_position.y, camera_position.z);
            let camera2 = camera_rotate;
            let camera = camera1 * camera2;
            gl::UniformMatrix4fv(view_matrix_uniform, 1, gl::FALSE, camera.m.as_ptr());

            let factor = window_width / window_height;
            let scene_width = 1.0;
            let scene_height = scene_width * factor;

            let projection = Matrix4::perspective_matrix(-scene_width, scene_width, -scene_height, scene_height, 0.01, 2000.)
                .inverse()
                .unwrap();
            gl::UniformMatrix4fv(projection_matrix_uniform, 1, gl::FALSE, projection.m.as_ptr());

            gl::DrawArrays(gl::TRIANGLES, 0, points.len() as i32 / 3 as i32);
            //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            //gl::DrawArrays(gl::TRIANGLES, 0, points.len() as i32 / 3 as i32);
            //gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            check_gl_error("DrawArrays");
        }

        window.swap_buffers().unwrap();
    });

    //let poly = Mesh::new_polygon(8, 10.);
    //println!("Poly: {}", poly.vertices.len());
}

const VERTEX_SHADER: &'static [u8] = b"
#version 150
#define PI 3.14159265359
#define saturate(a) clamp( a, 0.0, 1.0 )
uniform mat4 in_model_matrix;
uniform mat4 in_view_matrix;
uniform mat4 in_projection_matrix;
in vec3 in_position;
in vec3 in_normal;
out vec3 frag_color;
void main() {
    vec3 light_direction = vec3(-30., -30., -100.);
    vec3 light_color = vec3(0.8, 0.8, 0.82);
    float light_intensity = 0.1;
    frag_color = vec3( 0.0 );
    float dir = dot(in_normal, light_direction);
    //light_color = PI * light_color;
    frag_color += saturate(dir) * light_color;
    gl_Position = in_projection_matrix * in_view_matrix * in_model_matrix *  vec4(in_position, 1.0);
}
\0";

const FRAGMENT_SHADER: &'static [u8] = b"
#version 150
in vec3 frag_color;
out vec4 out_color;
void main() {
    out_color = vec4(frag_color, 1.0);
}
\0";
