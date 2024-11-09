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

pub fn fragment_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    // Opción 1: Líneas horizontales de colores
    let y = fragment.position.y as usize;
    let colors = [
        Color::new(255, 0, 0),   // Rojo
        Color::new(0, 255, 0),   // Verde
        Color::new(0, 0, 255),   // Azul
        Color::new(255, 255, 0), // Amarillo
    ];
    let stripe_width = 20;
    let stripe_index = (y / stripe_width) % colors.len();

    // Opción 2: Interpolación con la posición del vértice
    let vertex_y = fragment.vertex_position.y as usize;
    let vertex_based_color = if vertex_y % 40 > 20 {
        colors[stripe_index] * 0.8 // Banda más oscura usando posición del vértice
    } else {
        colors[stripe_index]
    };

    // Opción 3: Interpolación lineal (lerp) entre franjas de color
    let stripe_coord = fragment.vertex_position.y;
    let stripe_float = (stripe_coord / 0.1).abs();
    let lerp_index = (stripe_float as usize) % colors.len();
    let next_index = (lerp_index + 1) % colors.len();
    let t = stripe_float.fract();
    let lerped_color = colors[lerp_index].lerp(&colors[next_index], t) * fragment.intensity;

    // Opción 4: Patrones trigonométricos
    let color1 = Color::new(255, 0, 0);   // Rojo
    let color2 = Color::new(0, 255, 0);   // Verde
    let color3 = Color::new(0, 0, 255);   // Azul
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let frequency = 10.0;
    let wave1 = (x * 7.0 * frequency + y * 5.0 * frequency).sin() * 0.5 + 0.5;
    let wave2 = (x * 5.0 * frequency - y * 8.0 * frequency + std::f32::consts::PI / 3.0).sin() * 0.5 + 0.5;
    let wave3 = (y * 6.0 * frequency + x * 4.0 * frequency + 2.0 * std::f32::consts::PI / 3.0).sin() * 0.5 + 0.5;
    let mut final_trig_color = color1.lerp(&color2, wave1);
    final_trig_color = final_trig_color.lerp(&color3, wave2);
    final_trig_color = final_trig_color.lerp(&color1, wave3);

    // Elige el return que deseas probar
    // return colors[stripe_index];          // Opción 1: Líneas horizontales
    // return vertex_based_color;            // Opción 2: Posición del vértice
    // return lerped_color;                  // Opción 3: Interpolación lerp
    return final_trig_color;                 // Opción 4: Patrones trigonométricos
}