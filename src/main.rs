// main.rs

use crate::obj::Obj;
use crate::vertex::Vertex;
mod obj;
mod vertex;
mod color;
mod fragment;
mod line;
mod triangle;

// Estructura de una arista que conecta dos vértices
#[derive(Debug)]
struct Edge {
    start: Vertex,
    end: Vertex,
}

// Estructura de una cara que contiene tres vértices (triángulo)
#[derive(Debug)]
struct Face {
    vertices: [Vertex; 3],
}

// Estructura del modelo 3D que contiene una lista de vértices, aristas y caras
#[derive(Debug)]
struct Model3D {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    faces: Vec<Face>,
}

impl Model3D {
    // Constructor para crear un modelo 3D vacío
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
        }
    }

    // Método para agregar vértices desde un objeto OBJ
    fn add_vertices_from_obj(&mut self, obj: &Obj) {
        for vertex in obj.get_vertex_array() {
            self.vertices.push(vertex);
        }
    }
}

fn main() {
    // Cargar el archivo OBJ
    let obj = Obj::load("assets/tinker.obj").expect("Failed to load OBJ file");

    // Crear un nuevo modelo 3D y cargar vértices desde el archivo OBJ
    let mut model = Model3D::new();
    model.add_vertices_from_obj(&obj);

    // Imprimir el modelo 3D con los vértices cargados desde el archivo OBJ
    println!("{:#?}", model);
}
