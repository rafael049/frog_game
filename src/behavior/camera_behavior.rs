use nphysics3d::algebra::Force3;
use nphysics3d::math::ForceType;
use nphysics3d::object::Body;

use crate::input::KeyState;
use crate::object;
use crate::input;
use crate::behavior::Functor;

use crate::utils::*;


pub struct CameraBehavior {
		pub zoom: f32,
		pub near: f32,
		pub far: f32,
}

impl CameraBehavior {
		pub fn new() -> CameraBehavior {
				CameraBehavior {
						zoom: 2.0,
						near: -0.1,
						far: 10.0,
				}
		}
}

impl Functor for CameraBehavior {
		fn start(&mut self, target: *mut object::Object, input: &input::Input, delta: f32) {

		}

		fn update(&mut self, target: *mut object::Object, input: &input::Input, delta: f32) {
				// Convert target ptr to Object
				let mut obj:&mut object::Object;
				unsafe {
						obj = &mut (*target);
				}

				// Get Rigid Body
				let mut camera = obj.camera.as_mut().unwrap_or_else(|| panic!("Camera prop not found"));

				let mut shift = false;

				match input.get_mouse_button(input::MouseButtonKeyCode::Right) {
						KeyState::KeyDown => {
								shift = true;
						}
						_ => {}
				}


				match input.get_actions(input::ActionID::LOOK) {
						input::ActionInput::DIRECTIONAL(dir) => {
								if shift {
										camera.view_mat[12] += dir.x*self.zoom;
										camera.view_mat[13] -= dir.y*self.zoom;
								}
						}
						_ => {}
				}
				match input.get_actions(input::ActionID::ZOOM) {
						input::ActionInput::ANALOGIC(v) => {
								self.zoom -= v*0.1;
						}
						_ => {}
				}

				let zoom = self.zoom * self.zoom;

				camera.proj_mat = mat4orto(camera.aspect, zoom, self.near, self.far);

		}

}
