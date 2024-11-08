use crate::obj::Obj;
use crate::vertex::Vertex;
use crate::color::Color;
use crate::fragment::Fragment;
use crate::triangle::triangle;
use nalgebra_glm::{Vec3, Mat4};

mod obj;
mod vertex;
mod color;
mod fragment;
mod line;
mod triangle;

#[derive(Debug)]
struct Edge {
    start: Vertex,
    end: Vertex,
}

#[derive(Debug)]
struct Face {
    vertices: [Vertex; 3],
}

#[derive(Debug)]
struct Model3D {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    faces: Vec<Face>,
}

impl Model3D {
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
        }
    }

    fn add_vertices_from_obj(&mut self, obj: &Obj) {
        for vertex in obj.get_vertex_array() {
            self.vertices.push(vertex);
        }
    }
}

#[derive(Debug)]
struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Framebuffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    fn set_current_color(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    fn clear(&mut self, color: u32) {
        for pixel in &mut self.buffer {
            *pixel = color;
        }
    }
}

pub struct Uniforms {
    pub model_matrix: Mat4,
}

impl Uniforms {
    pub fn new(translation: Vec3, scale: f32) -> Self {
        let model_matrix = Mat4::new(
            scale, 0.0, 0.0, translation.x,
            0.0, scale, 0.0, translation.y,
            0.0, 0.0, scale, translation.z,
            0.0, 0.0, 0.0, 1.0,
        );
        Self { model_matrix }
    }
}

fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = nalgebra_glm::vec4(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    let transformed = uniforms.model_matrix * position;

    let w = transformed.w;
    let transformed_position = Vec3::new(transformed.x / w, transformed.y / w, transformed.z / w);

    Vertex {
        position: vertex.position,
        color: vertex.color,
        transformed_position,
        ..*vertex
    }
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader Stage
    let transformed_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|vertex| vertex_shader(vertex, uniforms))
        .collect();

    // Primitive Assembly & Rasterization Stage
    let mut fragments: Vec<Fragment> = Vec::new();
    for triangle_vertices in transformed_vertices.chunks(3) {
        if triangle_vertices.len() == 3 {
            let tri_fragments = triangle(
                &triangle_vertices[0],
                &triangle_vertices[1],
                &triangle_vertices[2],
            );
            fragments.extend(tri_fragments);
        }
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        let color = fragment.color.to_hex();
        framebuffer.set_current_color(x, y, color);
    }
}

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);
    framebuffer.clear(Color::black().to_hex());

    let obj = Obj::load("assets/tinker.obj").expect("Failed to load OBJ file");

    let mut model = Model3D::new();
    model.add_vertices_from_obj(&obj);

    let uniforms = Uniforms::new(Vec3::new(0.0, 0.0, 0.0), 1.0);

    render(&mut framebuffer, &uniforms, &model.vertices);

    println!("{:?}", framebuffer);
}
