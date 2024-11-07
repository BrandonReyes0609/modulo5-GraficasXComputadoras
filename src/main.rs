// Definición de un punto en 3D
#[derive(Debug, Clone, Copy)]
struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

// Estructura de un vértice que contiene un punto en 3D
#[derive(Debug, Clone, Copy)]
struct Vertex {
    position: Point3D,
}

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

    // Método para agregar un vértice al modelo
    fn add_vertex(&mut self, position: Point3D) -> Vertex {
        let vertex = Vertex { position };
        self.vertices.push(vertex);
        vertex
    }

    // Método para agregar una arista al modelo
    fn add_edge(&mut self, start: Vertex, end: Vertex) {
        let edge = Edge { start, end };
        self.edges.push(edge);
    }

    // Método para agregar una cara (triángulo) al modelo
    fn add_face(&mut self, v1: Vertex, v2: Vertex, v3: Vertex) {
        let face = Face { vertices: [v1, v2, v3] };
        self.faces.push(face);
    }
}

fn main() {
    // Crear un nuevo modelo 3D
    let mut model = Model3D::new();

    // Agregar vértices al modelo
    let v1 = model.add_vertex(Point3D { x: 0.0, y: 0.0, z: 0.0 });
    let v2 = model.add_vertex(Point3D { x: 1.0, y: 0.0, z: 0.0 });
    let v3 = model.add_vertex(Point3D { x: 0.0, y: 1.0, z: 0.0 });
    let v4 = model.add_vertex(Point3D { x: 1.0, y: 1.0, z: 0.0 });

    // Agregar aristas al modelo
    model.add_edge(v1, v2);
    model.add_edge(v2, v3);
    model.add_edge(v3, v1);

    // Agregar una cara al modelo (triángulo)
    model.add_face(v1, v2, v3);

    // Imprimir el modelo 3D
    println!("{:#?}", model);
}
