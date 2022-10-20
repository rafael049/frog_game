use crate::utils::*;

pub struct Material {
		pub color: Vec3,
		pub specular: f32,
}

impl Material {
		pub fn new(color: Vec3, specular: f32) -> Material {
				Material {
						color,
						specular,
				}
		}
}
