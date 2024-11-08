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

#[derive(Debug)]  // Agrega Debug para permitir la impresión de Framebuffer
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

fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.max(v2.x).max(v3.x).ceil() as i32;
    let max_y = v1.y.max(v2.y).max(v3.y).ceil() as i32;

    (min_x, min_y, max_x, max_y)
}

fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Transformar posición
    let position = nalgebra_glm::vec4(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    let transformed = uniforms.model_matrix * position;

    // División de perspectiva
    let w = transformed.w;
    let transformed_position = Vec3::new(transformed.x / w, transformed.y / w, transformed.z / w);

    Vertex {
        position: vertex.position,
        color: vertex.color,
        transformed_position,
        ..*vertex
    }
}

fn inside(v1: &Vec3, v2: &Vec3, v3: &Vec3, p: &Vec3) -> bool {
    // Determina si el punto `p` está dentro del triángulo definido por `v1`, `v2` y `v3`
    let edge1 = (v2.x - v1.x) * (p.y - v1.y) - (v2.y - v1.y) * (p.x - v1.x);
    let edge2 = (v3.x - v2.x) * (p.y - v2.y) - (v3.y - v2.y) * (p.x - v2.x);
    let edge3 = (v1.x - v3.x) * (p.y - v3.y) - (v1.y - v3.y) * (p.x - v3.x);
    (edge1 >= 0.0 && edge2 >= 0.0 && edge3 >= 0.0) || (edge1 <= 0.0 && edge2 <= 0.0 && edge3 <= 0.0)
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
            let (v1, v2, v3) = (
                &triangle_vertices[0].transformed_position,
                &triangle_vertices[1].transformed_position,
                &triangle_vertices[2].transformed_position,
            );

            // Calcular la bounding box del triángulo
            let (min_x, min_y, max_x, max_y) = calculate_bounding_box(v1, v2, v3);

            // Procesar cada píxel dentro de la bounding box
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    let point = Vec3::new(x as f32, y as f32, 0.0);
                    if inside(v1, v2, v3, &point) {
                        let color = triangle_vertices[0].color.to_hex();
                        framebuffer.set_current_color(x as usize, y as usize, color);
                    }
                }
            }
        }
    }
}

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);
    framebuffer.clear(Color::black().to_hex());

    let obj = Obj::load("assets/file.obj").expect("Failed to load OBJ file");

    let mut model = Model3D::new();
    model.add_vertices_from_obj(&obj);

    let uniforms = Uniforms::new(Vec3::new(0.0, 0.0, 0.0), 1.0);

    render(&mut framebuffer, &uniforms, &model.vertices);

    println!("{:?}", framebuffer);
}
