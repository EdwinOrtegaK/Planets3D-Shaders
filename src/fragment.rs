use nalgebra_glm::{Vec2, Vec3};
use std::f32::consts::PI;
use crate::color::Color;
use crate::Uniforms;
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;
use fastnoise_lite::FastNoiseLite;

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
    let r = (combined_pattern * 100.0) as u8;
    let g = ((1.0 - combined_pattern) * 170.0) as u8;
    let b = 240;

    let base_color = Color::new(r, g, b);

    // Ajuste de iluminación ambiental para un aspecto de gas disperso
    let ambient_intensity = 0.5;
    let ambient_color = Color::new(80, 130, 200);

    // Mezcla del color base y el color ambiental para dar una apariencia gaseosa en toda la superficie
    base_color * fragment.intensity + ambient_color * ambient_intensity
}

fn gas_giant_with_rings_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 4.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let time = uniforms.time as f32 * 0.03;

    // Crear un patrón gaseoso con bandas horizontales
    let pattern1 = ((x * zoom).sin() * (y * zoom * 0.5 + time).cos()).abs();
    let pattern2 = ((x * zoom * 0.7 - time).cos() * (y * zoom * 0.3 + time).sin()).abs();
    let combined_pattern = (pattern1 * 0.6 + pattern2 * 0.4).min(1.0);

    // Nuevos colores para distinguir este planeta
    let r = (combined_pattern * 220.0) as u8; // Tonos más cálidos en rojo
    let g = ((1.0 - combined_pattern) * 130.0 + 80.0) as u8;
    let b = 120;

    let base_color = Color::new(r, g, b);

    // Iluminación ambiental para un efecto gaseoso uniforme
    let ambient_intensity = 0.5;
    let ambient_color = Color::new(90, 60, 120);

    // Mezcla del color base y el color ambiental
    base_color * fragment.intensity + ambient_color * ambient_intensity
}

pub fn ring_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let color1 = Color::new(220, 200, 180);  
    let color2 = Color::new(150, 100, 70);   
    let color3 = Color::new(50, 30, 20);     

    let position = Vec3::new(
        fragment.vertex_position.x,
        fragment.vertex_position.y,
        fragment.depth,
    );

    let zoom = 10.0;
    let noise_zoom = 50.0; // Factor para ajustar el ruido

    // Generar el patrón de líneas horizontales
    let line_pattern = (position.y * zoom).sin().abs();

    // Agregar ruido para variar el patrón
    let ruido = uniforms.noise.get_noise_3d(
        position.x * noise_zoom,
        position.y * noise_zoom,
        position.z * noise_zoom,
    );

    // Mezclar el patrón de líneas con el ruido para crear variaciones
    let val_normalizado = (line_pattern * 0.7 + ruido * 0.3).clamp(0.0, 1.0);

    // Interpolación entre colores para crear el efecto de bandas
    let color_intermediate = color1.lerp(&color2, val_normalizado);
    let final_color = color_intermediate.lerp(&color3, val_normalizado);

    final_color * fragment.intensity
}

pub fn planet_colorful(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Definimos los colores basados en el diseño de la segunda imagen
    let color1 = Color::new(255, 223, 75);   // Amarillo claro
    let color2 = Color::new(255, 165, 0);    // Naranja
    let color3 = Color::new(238, 130, 238);  // Violeta
    let color4 = Color::new(173, 216, 230);  // Azul claro
    let color5 = Color::new(255, 105, 180);  // Rosa

    // Posiciones y tiempo para animaciones y efectos de ruido
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let tiempo = (uniforms.time as f32) * 0.01;

    // Frecuencia y ruido para patrones
    let frecuencia = 6.0;
    let distancia = (x * x + y * y).sqrt();

    // Generación de ruido suave en la superficie
    let ruido = (x * 0.3 + tiempo).sin() * (y * 0.3).cos() * 0.5;

    // Patrones de ondas en varias direcciones para simular las áreas de color
    let patron1 = ((distancia + ruido) * frecuencia + (y + ruido) * 3.0).sin() * 0.5 + 0.5;
    let patron2 = ((distancia + ruido) * frecuencia * 0.8 - (x + ruido) * 3.0).sin() * 0.5 + 0.5;
    let patron3 = ((distancia + ruido) * frecuencia * 1.2 + (x + ruido) * 4.0).sin() * 0.5 + 0.5;

    // Interpolación de colores según los patrones generados
    let mut color_final = color1.lerp(&color2, patron1);
    color_final = color_final.lerp(&color3, patron2);
    color_final = color_final.lerp(&color4, patron3);
    color_final = color_final.lerp(&color5, patron1 * patron2);

    let ambient_intensity = 0.5;
    let ambient_color = Color::new(90, 60, 120);

    // Multiplicamos por la intensidad del fragmento para asegurar brillo adecuado
    color_final * fragment.intensity + ambient_color * ambient_intensity
}

fn ruido_fractal(noise: &FastNoiseLite, x: f32, y: f32, octaves: u32, lacunarity: f32, gain: f32) -> f32 {
    let mut total = 0.0;
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    let mut max_value = 0.0;

    for _ in 0..octaves {
        total += noise.get_noise_2d(x * frequency, y * frequency) * amplitude;
        max_value += amplitude;

        amplitude *= gain;
        frequency *= lacunarity;
    }

    total / max_value
}

pub fn planet_exotic_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let color_amarillo = Color::new(255, 223, 75);    
    let color_naranja = Color::new(255, 165, 0);    
    let color_lila = Color::new(238, 130, 238);  
    let color_rosa = Color::new(255, 105, 180);  
    let color_purpura = Color::new(75, 0, 130);     

    let position = fragment.vertex_position;
    let t = uniforms.time as f32 * 0.6; 

    let zoom = 150.0;  
    let ruido = ruido_fractal(&uniforms.noise, position.x * zoom + t, position.y * zoom + t, 4, 2.0, 0.5);

    // Generar patrones de color con el ruido
    let patron1 = (ruido * 1.5 + (position.x * 0.5).sin() * 0.5).clamp(0.0, 1.0);
    let patron2 = ((position.y * 0.3 + ruido) * 2.0).sin().abs();

    let ambient_intensity = 0.3;
    let ambient_color = Color::new(30, 20, 60);

    // Interpolación de colores entre las diferentes zonas del planeta
    let mut color_final = color_amarillo.lerp(&color_naranja, patron1);
    color_final = color_final.lerp(&color_lila, patron2);
    color_final = color_final.lerp(&color_rosa, patron1 * patron2);
    color_final = color_final.lerp(&color_purpura, (1.0 - patron1) * 0.5);

    color_final * fragment.intensity + ambient_color * ambient_intensity
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, shader_type: &str) -> Color {
    match shader_type {
        "solar_surface" => solar_shader(fragment, uniforms),
        "rocky_planet_shader" => rocky_planet_shader(fragment, uniforms),
        "gas_giant_shader" => gas_giant_shader(fragment, uniforms),
        "gas_giant_with_rings" => gas_giant_with_rings_shader(fragment, uniforms),
        "ring" => ring_shader(fragment, uniforms),
        "colorful" => planet_colorful(fragment, uniforms),
        "exotic" => planet_exotic_shader(fragment, uniforms),
        _ => Color::new(0, 0, 0),
    }
}
