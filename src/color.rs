// color.rs
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn black() -> Self {
        Color { r: 0.0, g: 0.0, b: 0.0 }
    }

    pub fn red() -> Self {
        Color { r: 1.0, g: 0.0, b: 0.0 }
    }
}
