pub mod transform;
pub mod material;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::*;
use crate::behavior::Behavior;
use crate::resources::Texture;
use crate::resources::{Vertex};
use crate::input::Input;
use crate::input::{ActionID, ActionInput};
use crate::physics::{RigidBody, Collider};

pub type Rcell<T> = Rc<RefCell<T>>;
pub type VertexBuffer = glium::VertexBuffer<Vertex>;
pub type IndexBuffer = glium::index::NoIndices;

pub struct Object {
		pub name: String,
		pub transform: Option<transform::Transform>,
		pub material: Option<material::Material>,
		pub texture: Option<Rcell<Texture>>,
		pub vertex_buffer: Option<Rcell<VertexBuffer>>,
		pub index_buffer: Option<IndexBuffer>,
		pub behavior: Option<Behavior>,
		pub rigid_body: Option<RigidBody>,
		pub collider: Option<Collider>,
}

impl Object {
		pub fn new(name: &str,vertex_buffer: Rcell<VertexBuffer>, texture: Rcell<Texture>) -> Object {
				Object {
						name: name.to_string(),
						transform: Some(transform::Transform::new()),
						material: Some(material::Material::new(vec3(0.0, 0.0, 0.0), 1.0)),
						texture: Some(texture),
						vertex_buffer: Some(vertex_buffer),
						index_buffer: Some(glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList)),
						behavior: None,
						rigid_body: None,
						collider: None,
				}
		}

		pub fn new_handler(name: &str,vertex_buffer: Rcell<VertexBuffer>, texture: Rcell<Texture>) -> Rcell<Object> {
				Rc::new(
						RefCell::new(
								Object {
										name: name.to_string(),
										transform: Some(transform::Transform::new()),
										material: Some(material::Material::new(vec3(0.0, 0.0, 0.0), 1.0)),
										texture: Some(texture),
										vertex_buffer: Some(vertex_buffer),
										index_buffer: Some(glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList)),
										behavior: None,
										rigid_body: None,
										collider: None,
								}))
		}

		pub fn new_empty(name: &str) -> Object {
				Object {
						name: name.to_string(),
						transform: None,
						material: None,
						texture: None,
						vertex_buffer: None,
						index_buffer: None,
						behavior: None,
						rigid_body: None,
						collider: None,
				}
		}

		pub fn update(&mut self, delta: f32, input: &Input) {

				let mut behavior = self.behavior.as_mut().unwrap();
				behavior.update(delta, input);
				let speed = 0.1;
				let mut pos = self.transform.as_mut().unwrap().get_position();

				/*
				match input.get_actions(ActionID::MOVE) {
						ActionInput::DIRECTIONAL(dir) => {
								pos.x += speed*dir.x;
								pos.y += speed*dir.y;
								self.transform.as_mut().unwrap().set_position(pos);
						},
						_ => (),
				}
				*/
		}
}
