use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::color::Color;
use nalgebra_glm::Vec3;
use crate::calculate_bounding_box;
use crate::barycentric_coordinates;


pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    let (a, b, c) = (v1.transformed_position, v2.transformed_position, v3.transformed_position);

    let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);

    // Dirección de la fuente de luz estática
    let light_dir = Vec3::new(0.0, 0.0, -1.0);

    // Normal del triángulo (suponiendo que la normal de v1 sea representativa para flat shading)
    let normal = v1.transformed_normal.normalize();

    // Calcula la intensidad de la luz usando el producto punto
    let intensity = normal.dot(&light_dir).max(0.0);

    // Color base y aplicamos la intensidad de luz
    let base_color = Color { r: 100.0 / 255.0, g: 100.0 / 255.0, b: 100.0 / 255.0 }; // gris medio
    let lit_color = Color {
        r: base_color.r * intensity,
        g: base_color.g * intensity,
        b: base_color.b * intensity,
    };

    // Itera sobre cada píxel en el bounding box
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Vec3::new(x as f32, y as f32, 0.0);

            // Calcula las coordenadas baricéntricas
            let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c);

            // Verifica si el punto está dentro del triángulo
            if w1 >= 0.0 && w1 <= 1.0 &&
               w2 >= 0.0 && w2 <= 1.0 &&
               w3 >= 0.0 && w3 <= 1.0 {
                fragments.push(Fragment::new(x as f32, y as f32, lit_color, 0.0));
            }
        }
    }

    fragments
}
