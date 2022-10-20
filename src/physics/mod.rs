use crate::*;

use nphysics2d::{object::{DefaultBodyHandle,
												 DefaultColliderHandle,
												 RigidBodyDesc,
												 ColliderDesc,
												 DefaultBodySet, DefaultColliderSet, BodyPartHandle, BodyStatus}, ncollide2d::shape::ShapeHandle, math::Velocity};

pub struct RigidBody {
		pub handle: DefaultBodyHandle,
}

impl RigidBody {
		pub fn new(bodies: &mut DefaultBodySet<f32>, x: f32, y: f32) -> RigidBody {
				// Build the rigid body.
				let mut rb = RigidBodyDesc::new()
						.status(BodyStatus::Dynamic)
						.rotation(0.5)
						.translation(Vector2::new(x, y))
						.sleep_threshold(None)
						.mass(1.0)
						//.velocity(Velocity::angular(0.1))
						.build();
				//rb.set_deactivation_threshold(None);
				//rb.set_mass(2.0);
				let handle: DefaultBodyHandle = bodies.insert(rb);

				RigidBody { handle }
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
