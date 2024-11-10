use nalgebra_glm::{Vec2, Vec3};
use std::f32::consts::PI;
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

fn static_pattern_shader(fragment: &Fragment) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let pattern = ((x * 10.0).sin() * (y * 10.0).sin()).abs();

    let r = (pattern * 255.0) as u8;
    let g = ((1.0 - pattern) * 255.0) as u8;
    let b = 128;

    Color::new(r, g, b)
}

fn moving_circles_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let time = uniforms.time as f32 * 0.05;
    let circle1_x = (time.sin() * 0.4 + 0.5) % 1.0;
    let circle2_x = (time.cos() * 0.4 + 0.5) % 1.0;

    let dist1 = ((x - circle1_x).powi(2) + (y - 0.3).powi(2)).sqrt();
    let dist2 = ((x - circle2_x).powi(2) + (y - 0.7).powi(2)).sqrt();

    let circle_size = 0.1;
    let circle1 = if dist1 < circle_size { 1.0f32 } else { 0.0f32 };
    let circle2 = if dist2 < circle_size { 1.0f32 } else { 0.0f32 };

    let circle_intensity = (circle1 + circle2).min(1.0f32);

    Color::new(
        (circle_intensity * 255.0) as u8,
        (circle_intensity * 255.0) as u8,
        (circle_intensity * 255.0) as u8
    )
}

pub fn combined_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let base_color = static_pattern_shader(fragment);
    let circle_color = moving_circles_shader(fragment, uniforms);

    // Usa el color del círculo si no es negro, sino usa el color base
    if !circle_color.is_black() {
        circle_color * fragment.intensity
    } else {
        base_color * fragment.intensity
    }
}

fn purple_shader(_fragment: &Fragment) -> Color {
    Color::new(128, 0, 128) // Color morado
}

fn circle_shader(fragment: &Fragment) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let distance = (x * x + y * y).sqrt();

    if distance < 0.25 {
        Color::new(255, 255, 0) // Círculo amarillo
    } else {
        Color::new(0, 0, 0) // Fondo negro (transparente)
    }
}

pub fn combined_blend_shader(fragment: &Fragment, blend_mode: &str) -> Color {
    let base_color = purple_shader(fragment);
    let circle_color = circle_shader(fragment);

    let combined_color = match blend_mode {
        "normal" => base_color.blend_normal(&circle_color),
        "multiply" => base_color.blend_multiply(&circle_color),
        "add" => base_color.blend_add(&circle_color),
        "subtract" => base_color.blend_subtract(&circle_color),
        _ => base_color
    };

    combined_color * fragment.intensity
}

fn glow_shader(fragment: &Fragment) -> Color {
    let y = fragment.vertex_position.y;
    let stripe_width = 0.2;
    let glow_size = 0.05;

    let distance_to_center = (y % stripe_width - stripe_width / 2.0).abs();
    let glow_intensity = ((1.0 - (distance_to_center / glow_size).min(1.0)) * PI / 2.0).sin();

    Color::new(0, (0.6 * glow_intensity * 255.0) as u8, (glow_intensity * 255.0) as u8)
}

fn core_shader(fragment: &Fragment) -> Color {
    let y = fragment.vertex_position.y;
    let stripe_width = 0.2;
    let core_size = 0.02;

    let distance_to_center = (y % stripe_width - stripe_width / 2.0).abs();
    let core_intensity = if distance_to_center < core_size { 1.0 } else { 0.0 };

    Color::new((0.8 * core_intensity * 255.0) as u8, (0.9 * core_intensity * 255.0) as u8, (core_intensity * 255.0) as u8)
}

fn background_shader(_fragment: &Fragment) -> Color {
    Color::new(10, 10, 20) // Fondo azul oscuro
}

pub fn neon_light_shader(fragment: &Fragment) -> Color {
    let background = background_shader(fragment);
    let glow = glow_shader(fragment);
    let core = core_shader(fragment);

    let blended_glow = background.blend_screen(&glow);
    blended_glow.blend_add(&core)
}


pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, shader_type: &str, blend_mode: &str) -> Color {
    match shader_type {
        "static_pattern" => static_pattern_shader(fragment),
        "moving_circles" => moving_circles_shader(fragment, uniforms),
        "combined" => combined_shader(fragment, uniforms),
        "purple_circle_blend" => combined_blend_shader(fragment, blend_mode),
        "neon_light" => neon_light_shader(fragment),
        _ => Color::new(0, 0, 0), // Color negro en caso de que no se especifique un shader válido
    }
}

