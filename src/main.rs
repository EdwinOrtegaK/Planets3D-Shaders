use nalgebra_glm::{Vec2, Vec3, Mat4};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod experimental_shaders;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use triangle::triangle;
use shaders::vertex_shader;
use fastnoise_lite::{FastNoiseLite, NoiseType, CellularDistanceFunction};
use crate::fragment::{fragment_shader, Fragment, ring_shader};
use crate::color::Color;

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise_open_simplex: FastNoiseLite,
    noise_cellular: FastNoiseLite, 
}

pub struct Moon {
    pub position: Vec3,
    pub scale: f32,
    pub rotation: Vec3,
}

fn create_uniforms() -> Uniforms {
    let mut noise_open_simplex = FastNoiseLite::with_seed(1337);
    noise_open_simplex.set_noise_type(Some(NoiseType::OpenSimplex2));
    
    let mut noise_cellular = FastNoiseLite::with_seed(1337);
    noise_cellular.set_noise_type(Some(NoiseType::Cellular));
    noise_cellular.set_cellular_distance_function(Some(CellularDistanceFunction::Manhattan));

    Uniforms {
        model_matrix: Mat4::identity(),
        view_matrix: Mat4::identity(),
        projection_matrix: Mat4::identity(),
        viewport_matrix: Mat4::identity(),
        time: 0,
        noise_open_simplex,
        noise_cellular,
    }
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], shader_type: &str) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            // Llama a fragment_shader para calcular el color final del fragmento
            let shaded_color = fragment_shader(&fragment, uniforms, shader_type);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
    
}

fn create_open_simplex_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise
}

fn create_cellular_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::Cellular));
    noise
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Planetary System",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x333355);

    let mut translation = Vec3::new(window_width as f32 / 2.0, window_height as f32 / 2.0, 0.0);
    let mut rotation = Vec3::new(0.0, 0.0, 0.0);
    let mut scale = 100.0f32;

    let planet_obj = Obj::load("assets/sphere.obj").expect("Failed to load obj");
    let planet_vertex_array = planet_obj.get_vertex_array();

    let ring_obj = Obj::load("assets/rings.obj").expect("Failed to load rings.obj");
    let ring_vertex_array = ring_obj.get_vertex_array(); 

    let mut time = 0;

    // Añadimos las constantes para identificar los cuerpos celestes
    const STAR: u8 = 1;
    const ROCKY_PLANET: u8 = 2;
    const GAS_GIANT: u8 = 3;
    const GAS_GIANT_WITH_RINGS: u8 = 4;
    const PLANET_COLORFUL: u8 = 5;
    const PLANET_EXOTIC: u8 = 6;
    const DARK_RED: u8 = 7;
    const ROCKY_PLANET_WITH_MOON: u8 = 8;

    // Variable para guardar el cuerpo celeste seleccionado
    let mut selected_object: u8 = STAR;

    let mut moon = Moon {
        position: Vec3::new(0.0, 0.0, 0.0),
        scale: 30.0,
        rotation: Vec3::new(0.0, 0.0, 0.0),
    };

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
        time += 1;

        handle_input(&window, &mut translation, &mut rotation, &mut scale);

        // Cambiamos el objeto seleccionado con teclas
        if window.is_key_down(Key::Key1) {
            selected_object = STAR;
        } else if window.is_key_down(Key::Key2) {
            selected_object = ROCKY_PLANET;
        } else if window.is_key_down(Key::Key3) {
            selected_object = GAS_GIANT;
        } else if window.is_key_down(Key::Key4) {
            selected_object = GAS_GIANT_WITH_RINGS;
        } else if window.is_key_down(Key::Key5) {
            selected_object = PLANET_COLORFUL;
        } else if window.is_key_down(Key::Key6) {
            selected_object = PLANET_EXOTIC;
        } else if window.is_key_down(Key::Key7) {
            selected_object = DARK_RED;
        } else if window.is_key_down(Key::Key8) {
            selected_object = ROCKY_PLANET_WITH_MOON;
        }

        framebuffer.clear();

        let model_matrix = create_model_matrix(translation, scale, rotation);
        let mut uniforms = create_uniforms();
        uniforms.model_matrix = model_matrix;
        uniforms.time = time;

        // Renderizamos el objeto seleccionado con shaders específicos
        match selected_object {
            STAR => {
                framebuffer.set_current_color(0xFFDDDD);
                render(&mut framebuffer, &uniforms, &planet_vertex_array, "solar_surface");
            },
            ROCKY_PLANET => {
                framebuffer.set_current_color(0xAAAAAA);
                render(&mut framebuffer, &uniforms, &planet_vertex_array, "rocky_planet_shader");
            },
            GAS_GIANT => {
                framebuffer.set_current_color(0x00FFAA);
                render(&mut framebuffer, &uniforms, &planet_vertex_array, "gas_giant_shader");
            },
            GAS_GIANT_WITH_RINGS => {
                framebuffer.set_current_color(0x00FFAA);
                render(&mut framebuffer, &uniforms, &planet_vertex_array, "gas_giant_with_rings");

                let ring_model_matrix = create_model_matrix(translation, scale * 1.2, rotation);
                uniforms.model_matrix = ring_model_matrix;
                render(&mut framebuffer, &uniforms, &ring_vertex_array, "ring");
            },
            PLANET_COLORFUL => {
                framebuffer.set_current_color(0x00FFAA);
                render(&mut framebuffer, &uniforms, &planet_vertex_array, "colorful");
            },
            PLANET_EXOTIC => {
                framebuffer.set_current_color(0x00FFAA);
                render(&mut framebuffer, &uniforms, &planet_vertex_array, "exotic");
            },
            DARK_RED => {
                framebuffer.set_current_color(0x00FFAA);
                render(&mut framebuffer, &uniforms, &planet_vertex_array, "dark_red");
            },
            ROCKY_PLANET_WITH_MOON => {
                // Renderizar el nuevo planeta rocoso
                let planet_translation = translation;
                let planet_model_matrix = create_model_matrix(planet_translation, scale, rotation);
                uniforms.model_matrix = planet_model_matrix;

                framebuffer.set_current_color(0xAAAAAA);
                render(&mut framebuffer, &uniforms, &planet_vertex_array, "rocky_planet_with_moon_shader");

                // Calcular posición de la luna para la órbita
                let orbit_radius = 150.0;
                let orbit_speed = 0.02;
                let angle = time as f32 * orbit_speed;

                moon.position.x = planet_translation.x + orbit_radius * angle.cos();
                moon.position.y = planet_translation.y + orbit_radius * angle.sin();

                // Renderizar la luna
                let moon_model_matrix = create_model_matrix(moon.position, moon.scale, moon.rotation);
                uniforms.model_matrix = moon_model_matrix;
                framebuffer.set_current_color(0x888888);
                render(&mut framebuffer, &uniforms, &planet_vertex_array, "moon_shader");
            },
            _ => {},
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

/*
fn render_rings(framebuffer: &mut Framebuffer, uniforms: &Uniforms) {
    let ring_inner_radius = 1.2;
    let ring_outer_radius = 1.8;

    for x in -400..400 {
        for y in -400..400 {
            let xf = x as f32 / 100.0;
            let yf = y as f32 / 100.0;
            let distance = (xf.powi(2) + yf.powi(2)).sqrt();

            if distance > ring_inner_radius && distance < ring_outer_radius {
                let fragment = Fragment::new(
                    Vec2::new(xf, yf),
                    Color::new(0, 0, 0),
                    1.0,
                    Vec3::new(0.0, 0.0, 1.0),
                    1.0,
                    Vec3::new(xf, yf, 0.0),
                );

                // Aplicamos el shader de anillos
                let ring_color = ring_shader(&fragment, uniforms);
                
                // Calcular posiciones de pantalla sin `viewport_matrix`
                let x_screen = (xf * 100.0 + framebuffer.width as f32 / 2.0) as usize;
                let y_screen = (yf * 100.0 + framebuffer.height as f32 / 2.0) as usize;
                
                if x_screen < framebuffer.width && y_screen < framebuffer.height {
                    framebuffer.set_current_color(ring_color.to_hex());
                    framebuffer.point(x_screen, y_screen, 1.0);
                }
            }
        }
    }
}*/

fn handle_input(window: &Window, translation: &mut Vec3, rotation: &mut Vec3, scale: &mut f32) {
    let move_speed = 10.0; 
    let rotation_speed = 0.2; 
    let zoom_speed = 2.0; 

    // Movimiento de cámara
    if window.is_key_down(Key::Left) {
        translation.x -= move_speed; 
    }
    if window.is_key_down(Key::Right) {
        translation.x += move_speed; 
    }
    if window.is_key_down(Key::Up) {
        translation.y -= move_speed; 
    }
    if window.is_key_down(Key::Down) {
        translation.y += move_speed; 
    }

    // Control de rotación 
    if window.is_key_down(Key::A) {
        rotation.y += rotation_speed; 
    }
    if window.is_key_down(Key::D) {
        rotation.y -= rotation_speed; 
    }
    if window.is_key_down(Key::W) {
        rotation.x += rotation_speed; 
    }
    if window.is_key_down(Key::S) {
        rotation.x -= rotation_speed; 
    }

    // Zoom
    if window.is_key_down(Key::Q) {
        *scale += zoom_speed;  
    }
    if window.is_key_down(Key::E) {
        *scale -= zoom_speed;  
    }
}