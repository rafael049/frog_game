use nphysics3d::algebra::Force3;
use nphysics3d::math::ForceType;
use nphysics3d::object::Body;

use crate::object;
use crate::input;
use crate::behavior::Functor;

use crate::utils::*;


pub struct PlayerBehavior {
		pub speed: f32,
}

impl PlayerBehavior {
		pub fn new() -> PlayerBehavior {
				PlayerBehavior { speed: 1.0 }
		}
}

impl Functor for PlayerBehavior {
		fn start(&mut self, target: *mut object::Object, input: &input::Input, delta: f32) {

		}

		fn update(&mut self, target: *mut object::Object, input: &input::Input, delta: f32) {
				// Convert target ptr to Object
				let mut obj:&mut  object::Object;
				unsafe {
						obj = &mut (*target);
				}

				// Get Rigid Body
				let mut rb = obj.rigid_body.as_mut().unwrap().get_body_mut();

				match input.get_actions(input::ActionID::MOVE) {
						input::ActionInput::DIRECTIONAL(dir) => {
								let speed = self.speed;
								let force = Force3::new(vec3(dir.x*speed, dir.y*speed, 0.0), vec3(0.0, 0.0, 0.0));
								rb.set_linear_damping(10.0);
								rb.set_angular_damping(f32::INFINITY);
								rb.apply_force(0, &force, ForceType::VelocityChange, false);
						}
						_ => {}
				}

		}

}
