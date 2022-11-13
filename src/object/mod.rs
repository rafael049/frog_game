pub mod transform;
pub mod material;
pub mod camera;

use std::cell::RefCell;
use std::rc::Rc;

use crate::{utils::*, behavior};
use crate::behavior::Behavior;
use crate::resources::{Vertex, Texture};
use crate::input::Input;
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
		pub camera: Option<camera::Camera>
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
						camera: None,
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
										camera: None,
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
						camera: None,
				}
		}

		pub fn update(&mut self, delta: f32, input: &Input) {

				// Execute behavior
				match self.behavior.as_mut() {
						Some(behavior) => {
								behavior.update(delta, input);
								println!("Obj <{}> has behavior", self.name);
						},
						None => {println!("Obj <{}> do not have behavior", self.name);},
				}
				
				//let speed = 0.1;
				//let mut pos = self.transform.as_mut().unwrap().get_position();

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

		pub fn model_transform(&self) -> Mat4 {
				let model_mat = self.transform.as_ref().unwrap().model_mat;
				return model_mat;
		}

		pub fn global_transform(&self) -> Mat4 {

				let model_mat = match &self.transform {
						Some(trsf) => trsf.model_mat,
						_ => mat4id(),
				};

				let body_mat = match &self.rigid_body {
						Some(rb) => rb.get_body().position().to_homogeneous(),
						None => mat4id(),
				};

				let global_mat = body_mat * model_mat;

				return global_mat;
		}
}
