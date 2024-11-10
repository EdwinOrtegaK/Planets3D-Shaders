use nalgebra_glm::{Vec2, Vec3};
use std::f32::consts::PI;
use crate::color::Color;
use crate::Uniforms;
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;
use crate::experimental_shaders::{random_color_shader, panda_shader, cloud_shader, cellular_shader};

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

// Shaders para planetas
fn solar_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let time_factor = (uniforms.time as f32 * 0.05).sin() * 0.4 + 0.8; // Ajuste para evitar áreas muy oscuras

    // Variaciones en la superficie usando ruido para simular textura
    let zoom = 15.0;
    let noise_value = uniforms.noise.get_noise_2d(x * zoom, y * zoom) * 0.3 + 0.7; // Reducción de variación
    let surface_intensity = (0.9 + noise_value * 0.1) * time_factor; // Ajuste para eliminar la acumulación oscura

    // Color base con variaciones para simular la superficie solar
    let r = (255.0 * surface_intensity) as u8;
    let g = (200.0 * surface_intensity) as u8;
    let b = (50.0 * surface_intensity) as u8;

    let core_color = Color::new(r, g, b) * fragment.intensity;

    // Efecto de halo alrededor del Sol
    let distance_to_center = (x.powi(2) + y.powi(2)).sqrt();
    let halo_threshold = 0.0;
    let halo_intensity = if distance_to_center > halo_threshold {
        ((distance_to_center - halo_threshold) * 3.0).exp().min(1.0)
    } else {
        0.0
    };

    let halo_color = Color::new(255, 140, 0) * halo_intensity;

    // Iluminación ambiental para que toda la esfera tenga visibilidad mínima
    let ambient_intensity = 0.1;
    let ambient_color = Color::new(255, 100, 50) * ambient_intensity;

    // Mezcla del color de la superficie, halo, y luz ambiental
    core_color.blend_add(&halo_color).blend_add(&ambient_color)
}

fn rocky_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 8.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    
    // Ajuste para el patrón de color en la superficie
    let color_variation = (x * zoom + y * zoom).sin().abs();
    let main_color = Color::new(139, (69.0 * color_variation) as u8, (19.0 * color_variation) as u8);

    // Ajuste de iluminación ambiental para todo el planeta
    let ambient_intensity = 0.7;
    let ambient_color = Color::new(60, 30, 10);

    // Mezcla el color principal con la luz ambiental, eliminando sombras fuertes
    main_color * fragment.intensity + ambient_color * ambient_intensity
}

fn gas_giant_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 3.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let time = uniforms.time as f32 * 0.05;

    // Crear un patrón basado en ondas para un efecto gaseoso dinámico
    let pattern1 = ((x * zoom + time).sin() * (y * zoom + time).cos()).abs();
    let pattern2 = ((x * zoom * 0.5 - time).cos() * (y * zoom * 0.7 + time).sin()).abs();
    let combined_pattern = (pattern1 + pattern2 * 0.5).min(1.0);

    // Colores de base con tonos más azulados y menos fucsia
    let r = (combined_pattern * 100.0) as u8; // Reducción de intensidad en rojo para menos fucsia
    let g = ((1.0 - combined_pattern) * 170.0) as u8;
    let b = 240; // Aumento en el azul para tonos más fríos

    let base_color = Color::new(r, g, b);

    // Ajuste de iluminación ambiental para un aspecto de gas disperso
    let ambient_intensity = 0.5;
    let ambient_color = Color::new(80, 130, 200); // Tonos más azules en la iluminación ambiental

    // Mezcla del color base y el color ambiental para dar una apariencia gaseosa en toda la superficie
    base_color * fragment.intensity + ambient_color * ambient_intensity
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, shader_type: &str) -> Color {
    match shader_type {
        "solar_surface" => solar_shader(fragment, uniforms),
        "rocky_planet_shader" => rocky_planet_shader(fragment, uniforms),
        "gas_giant_shader" => gas_giant_shader(fragment, uniforms),
        _ => Color::new(0, 0, 0),
    }
}
