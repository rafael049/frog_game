use nphysics3d::{world::{DefaultMechanicalWorld, DefaultGeometricalWorld}, object::{DefaultBodySet, DefaultColliderSet}, joint::DefaultJointConstraintSet, force_generator::DefaultForceGeneratorSet};

use crate::scene::scene_tree::Scene;
use crate::utils::*;


pub struct PhysicsEngine {
		pub mechanical_world: DefaultMechanicalWorld<f32>,
		pub geometrical_world: DefaultGeometricalWorld<f32>,
		pub bodies: DefaultBodySet<f32>,
		pub colliders: DefaultColliderSet<f32>,
		pub joint_constraints: DefaultJointConstraintSet<f32>,
		pub force_generators: DefaultForceGeneratorSet<f32>,
}


impl  PhysicsEngine {
		pub fn new() -> PhysicsEngine {
				let mechanical_world = DefaultMechanicalWorld::<f32>::new(vec3(0.0f32, -9.81f32, 0.0));
				let geometrical_world = DefaultGeometricalWorld::<f32>::new();

				let bodies = DefaultBodySet::<f32>::new();
				let colliders = DefaultColliderSet::<f32>::new();

				let joint_constraints = DefaultJointConstraintSet::<f32>::new();
				let force_generators = DefaultForceGeneratorSet::<f32>::new();
				PhysicsEngine { mechanical_world, geometrical_world, bodies, colliders, joint_constraints, force_generators  }
		}

		pub fn step(&mut self, scene: &mut Scene) {
				// physics
				//scene.foreach_mut(|obj| {
				//		match obj.rigid_body.as_ref() {
				//				Some(rb) => {
				//						let rb_handle = rb.handle;
				//						let rigid_body = self.bodies.rigid_body(rb_handle).unwrap();
				//						let v = rigid_body.position().translation;
				//						let pos = Vec3::new(v.x, v.y, 0.0);
				//						let angle = rigid_body.position().rotation.angle();
				//						obj.transform.as_mut().unwrap().set_position(&pos);
				//						obj.transform.as_mut().unwrap().set_2d_rotation(angle);
				//				},
				//				None => {},
				//		}
				//});

				// Run the physics simulation.
				self.mechanical_world.step(
						&mut self.geometrical_world,
						&mut self.bodies,
						&mut self.colliders,
						&mut self.joint_constraints,
						&mut self.force_generators
				);
		}
}
