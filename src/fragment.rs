use nalgebra_glm::{Vec2, Vec3};
use crate::color::Color;
use crate::Uniforms;

pub struct Fragment {
    pub position: Vec2,
    pub color: Color,
    pub depth: f32,
    pub normal: Vec3,
    pub intensity: f32,
}

impl Fragment {
    pub fn new(position: Vec2, color: Color, depth: f32, normal: Vec3, intensity: f32) -> Self {
        Fragment {
            position,
            color,
            depth,
            normal,
            intensity,
        }
    }
}

pub fn fragment_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    fragment.color * fragment.intensity
}