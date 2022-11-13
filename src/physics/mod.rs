pub mod physics_engine;

use crate::*;

use glium::buffer::Content;
use nphysics3d::{object::{DefaultBodyHandle,
												 DefaultColliderHandle,
												 RigidBodyDesc,
												 ColliderDesc,
												 DefaultBodySet, DefaultColliderSet, BodyPartHandle, BodyStatus}, ncollide3d::shape::ShapeHandle};
use nphysics3d::object;
use nphysics3d::nalgebra::Vector3;

pub struct RigidBody {
		pub handle: DefaultBodyHandle,
		body: *mut object::RigidBody<f32>,
}

impl RigidBody {
		pub fn new(bodies: &mut DefaultBodySet<f32>, status: BodyStatus, x: f32, y: f32) -> RigidBody {
				// Build the rigid body.
				let rb = RigidBodyDesc::new()
						.status(status)
						//.rotation(0.5)
						.translation(vec3(x, y, 0.0))
						.sleep_threshold(None)
						.mass(1.0)
						.kinematic_translations(Vector3::new(false, false, true))
						.kinematic_rotations(Vector3::new(true, true, false))
						//.velocity(Velocity::angular(0.1))
						.build();
				let handle: DefaultBodyHandle = bodies.insert(rb);
				let mut body: *mut object::RigidBody<f32>;
				unsafe {
						body = bodies.rigid_body_mut(handle).unwrap() as *mut object::RigidBody<f32>;
				}

				RigidBody { handle, body }
		}

		pub fn get_body_mut(&mut self) -> &mut object::RigidBody<f32> {
				unsafe {
						return self.body.as_mut().unwrap();
				}
		}

		pub fn get_body(&self) -> &object::RigidBody<f32> {
				unsafe {
						return self.body.as_ref().unwrap();
				}
		}
}


pub struct Collider {
		pub handle: DefaultColliderHandle,
}

impl Collider {
		pub fn new(colliders: &mut DefaultColliderSet<f32>, rb_handle: DefaultBodyHandle, shape: ShapeHandle<f32> ) -> Collider {
				let co = ColliderDesc::new(shape)
						.density(1.0)
						.build(BodyPartHandle(rb_handle, 0));
				let col_handle = colliders.insert(co);

				Collider { handle: col_handle }
		}
}
