use nalgebra_glm::{Vec2, Vec3};
use crate::color::Color;
use crate::Uniforms;

pub struct Fragment {
    pub position: Vec2,
    pub color: Color,
    pub depth: f32,
    pub normal: Vec3,
    pub intensity: f32,
    pub vertex_position: Vec3
}

impl Fragment {
    pub fn new(position: Vec2, color: Color, depth: f32, normal: Vec3, intensity: f32, vertex_position: Vec3) -> Self {
        Fragment {
            position,
            color,
            depth,
            normal,
            intensity,
            vertex_position
        }
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Opción 1: Cambio de color a través del tiempo
    let colors = [
        Color::new(255, 0, 0),   // Rojo
        Color::new(0, 255, 0),   // Verde
        Color::new(0, 0, 255),   // Azul
        Color::new(255, 255, 0), // Amarillo
        Color::new(255, 0, 255), // Magenta
        Color::new(0, 255, 255), // Cian
    ];
    let frames_per_color = 100;
    let color_index = (uniforms.time / frames_per_color) as usize % colors.len();
    let transition_progress = (uniforms.time % frames_per_color) as f32 / frames_per_color as f32;
    let current_color = colors[color_index];
    let next_color = colors[(color_index + 1) % colors.len()];
    let time_based_color = current_color.lerp(&next_color, transition_progress) * fragment.intensity;

    // Opción 2: Líneas horizontales en movimiento con el tiempo
    let color1 = Color::new(255, 0, 0); // Rojo
    let color2 = Color::new(0, 0, 255); // Azul
    let stripe_width = 0.2;
    let speed = 0.002;
    let moving_y = fragment.vertex_position.y + uniforms.time as f32 * speed;
    let stripe_factor = ((moving_y / stripe_width) * std::f32::consts::PI).sin() * 0.5 + 0.5;
    let moving_stripes_color = color1.lerp(&color2, stripe_factor) * fragment.intensity;

    // Opción 3: Patrón de puntos en movimiento
    let background_color = Color::new(250, 250, 250); // Gris claro
    let dot_color = Color::new(255, 0, 0);            // Rojo
    let dot_size = 0.1;
    let dot_spacing = 0.3;
    let speed = 0.01;
    let moving_x = fragment.vertex_position.x + uniforms.time as f32 * speed;
    let moving_y = fragment.vertex_position.y - uniforms.time as f32 * speed * 0.5;
    let pattern_x = ((moving_x / dot_spacing) * 2.0 * std::f32::consts::PI).cos();
    let pattern_y = ((moving_y / dot_spacing) * 2.0 * std::f32::consts::PI).cos();
    let dot_pattern = (pattern_x * pattern_y).max(0.0);
    let dot_factor = (dot_pattern - (1.0 - dot_size)).max(0.0) / dot_size;
    let moving_dots_color = background_color.lerp(&dot_color, dot_factor) * fragment.intensity;

    // Elige el return que deseas probar
    // return time_based_color;          // Opción 1: Cambio de color con el tiempo
    // return moving_stripes_color;      // Opción 2: Líneas en movimiento
    return moving_dots_color;            // Opción 3: Patrón de puntos en movimiento
}
