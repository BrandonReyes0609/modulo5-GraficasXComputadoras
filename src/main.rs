// main.rs

use crate::obj::Obj;
use crate::vertex::Vertex;
use crate::color::Color;
use crate::fragment::Fragment;
use crate::triangle::triangle;

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
    // Agrega aquí cualquier variable uniforme necesaria para tu renderizado
}

fn render(framebuffer: &mut Framebuffer, _uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader Stage
    let transformed_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|v| {
            Vertex {
                position: v.position,
                color: v.color,
                ..*v
            }
        })
        .collect();

    // Primitive Assembly Stage
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

    // Verificación: Imprimir cuántos fragmentos se han generado
    println!("Fragments generated: {}", fragments.len());

    // Rasterization Stage & Fragment Processing Stage
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

    let uniforms = Uniforms {};

    render(&mut framebuffer, &uniforms, &model.vertices);

    println!("{:?}", framebuffer);
}
