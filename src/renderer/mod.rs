
use glium::DrawParameters;

use crate::camera::Camera;
use crate::object::{Object, self};
use crate::scene::scene_tree::Scene;
use crate::shader::Shader;
use crate::glium::glutin::window::WindowBuilder;
use crate::glium::{Display, Surface};
use crate::glutin::ContextBuilder;

use crate::{utils::*, scene};

use std::cell::RefCell;


pub struct Renderer<'a> {
		pub window_size: (u32, u32),
		pub params: DrawParameters<'a>,
}



impl Renderer<'_> {
		pub fn new(display: &Display) -> Renderer<'static> {

				let window_size: (u32, u32) = (display.gl_window().window().inner_size().width,
																					 display.gl_window().window().inner_size().height);
				
				let params = DrawParameters {
						depth: glium::Depth {
								test: glium::draw_parameters::DepthTest::IfLess,
								write: true,
								.. Default::default()
						},
						.. Default::default()
				};
				
				Renderer{ window_size,
									params,
				}
		}

		pub fn render(&self, display: &Display, scene: &Scene, shader: &Shader, total_time: f32) {
				// Initiate Rendering
				let mut target = display.draw();
				target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

				// Get copy of camera trait
				let camera_prop: object::camera::Camera;
				{
						let camera_obj = scene.get_camera().unwrap_or_else(|| panic!("Camera object not found"));
						let camera_obj = camera_obj.borrow();
						let camera_obj = camera_obj.obj.borrow();

						camera_prop = camera_obj.camera.as_ref().unwrap_or_else(|| panic!("Camera Object has no Camera Property")).clone();
				}

				// get screen resulution
				let resolution = vec2(self.window_size.0 as f32, self.window_size.1 as f32);

				// Render objects
				scene.foreach( |obj| {
						// Get texture
						let texture_cell = match &obj.texture {
								Some(t) => t,
								None => return,
						};
						let texture = &*RefCell::borrow(&texture_cell);

						// Get transform
						let transform = obj.global_transform();
						// Get vertexBuffer
						let vertex_buffer = match &obj.vertex_buffer {
								Some(t) => t,
								None => return,
						};
						let vertex_buffer = &*RefCell::borrow(&vertex_buffer);

						// Get indexBuffer
						let index_buffer = match &obj.index_buffer {
								Some(t) => t,
								None => return,
						};
						let index_buffer = index_buffer;

						let uniforms = uniform!{
								view_mat: *camera_prop.view_mat.as_ref(),
								proj_mat: *camera_prop.proj_mat.as_ref(),
								iTime: total_time,
								iResolution: *resolution.as_ref(),
								trsf_mat: *transform.as_ref(),
								tex: texture 
						};

						target.draw(vertex_buffer, index_buffer, &shader.get_program(), &uniforms, &self.params).unwrap();
				});

				target.finish().unwrap();
		}
}
