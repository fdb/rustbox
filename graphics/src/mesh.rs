use std::f32::consts::PI;
use super::Vector3;

pub const TWO_PI: f32 = PI * 2_f32;

#[repr(C)]
pub struct Mesh {
    pub vertices: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub triangles: Vec<u32>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            normals: Vec::new(),
            triangles: Vec::new(),
        }
    }

    pub fn new_polygon(num_sides: u32, radius: f32) -> Mesh {
        let mut vertices = Vec::with_capacity(num_sides as usize);
        let mut normals = Vec::with_capacity(num_sides as usize);
        let mut triangles = Vec::with_capacity((3 * (num_sides - 2))  as usize);
        let angle_step = TWO_PI / num_sides as f32;
        println!("angle_step: {}", angle_step);
        for i in 0..num_sides {
            let angle = angle_step * i as f32;
            vertices.push(Vector3::new(radius * angle.cos(), radius * angle.sin(), 1.));
            normals.push(Vector3::new(0., 0., -1.));
        }

        for i in 1..(num_sides - 1) {
            triangles.push(0);
            triangles.push(i + 1);
            triangles.push(i);
        }
        Mesh{vertices, normals, triangles}
    }

    pub fn new_box(position: &Vector3, width: f32, height: f32, depth: f32) -> Mesh {
        let num_vertices = 36;
        let num_triangles = 36;
        let mut vertices = Vec::with_capacity(num_vertices as usize);
        let mut normals = Vec::with_capacity(num_vertices as usize);
        let mut triangles = Vec::with_capacity(num_triangles as usize);
        let half_width = width / 2.;
        let half_height = height / 2.;
        let half_depth = depth / 2.;
        let x1 = position.x - half_width;
        let x2 = position.x + half_width;
        let y1 = position.y - half_height;
        let y2 = position.y + half_height;
        let z1 = position.z - half_depth;
        let z2 = position.z + half_depth;

        let points = vec![
            Vector3::new(x1, y1, z1),
            Vector3::new(x1, y2, z1),
            Vector3::new(x2, y2, z1),
            Vector3::new(x2, y1, z1),

            Vector3::new(x1, y1, z2),
            Vector3::new(x1, y2, z2),
            Vector3::new(x2, y2, z2),
            Vector3::new(x2, y1, z2),
        ];

        // Front
        vertices.push(points[3]);
        vertices.push(points[1]);
        vertices.push(points[2]);
        vertices.push(points[3]);
        vertices.push(points[0]);
        vertices.push(points[1]);

        // Bottom
        vertices.push(points[7]);
        vertices.push(points[0]);
        vertices.push(points[3]);
        vertices.push(points[7]);
        vertices.push(points[4]);
        vertices.push(points[0]);

        // Right
        vertices.push(points[6]);
        vertices.push(points[3]);
        vertices.push(points[2]);
        vertices.push(points[6]);
        vertices.push(points[7]);
        vertices.push(points[3]);

        // Left
        vertices.push(points[0]);
        vertices.push(points[5]);
        vertices.push(points[1]);
        vertices.push(points[0]);
        vertices.push(points[4]);
        vertices.push(points[5]);

        // Back
        vertices.push(points[7]);
        vertices.push(points[5]);
        vertices.push(points[4]);
        vertices.push(points[7]);
        vertices.push(points[6]);
        vertices.push(points[5]);

        // Top
        vertices.push(points[6]);
        vertices.push(points[1]);
        vertices.push(points[5]);
        vertices.push(points[6]);
        vertices.push(points[2]);
        vertices.push(points[1]);

        for _ in 0..18 {
            normals.push(Vector3::new(0., 0., 1.));
        }

        for _ in 0..18 {
            normals.push(Vector3::new(0., 0., -1.));
        }

        for i in 0..num_triangles {
            triangles.push(i);
        }

        Mesh {vertices, normals, triangles}
    }

    pub fn to_vertex_data(&self) -> Vec<f32> {
        // TODO: This should map nicely to glDrawElements, however we don't use that at the moment.
        let mut points = Vec::new();
        for i in &self.triangles {
            let v = &self.vertices[*i as usize];
            let n = &self.normals[*i as usize];
            points.push(v.x);
            points.push(v.y);
            points.push(v.z);
            points.push(n.x);
            points.push(n.y);
            points.push(n.z);
        }
        points
    }
}
