use crate::utils::*;

pub struct Transform {
		pub local_mat: Mat4,
		pub model_mat: Mat4,

		pub local_pos: Vec3,
		pub local_scale: Vec3,
		pub local_rot: Quat,

		pub model_pos: Vec3,
		pub model_scale: Vec3,
		pub model_rot: Quat,
}

impl Transform {
		pub fn new() -> Transform {
				Transform {
						local_mat: mat4id(),
						model_mat: mat4id(),

						local_pos: Vec3::zeros(),
						local_scale: Vec3::repeat(1.0),
						local_rot: Quat::identity(),

						model_pos: Vec3::zeros(),
						model_scale: Vec3::repeat(1.0),
						model_rot: Quat::identity(),
				}
		}

		pub fn set_position(&mut self, value: &Vec3) {
				self.model_mat[12] = value.x;
				self.model_mat[13] = value.y;
				self.model_mat[14] = value.z;
				self.model_pos = *value;
		}

		pub fn set_rotation(&mut self, value: &Quat) {
				self.model_rot = *value;
				self.model_mat = value.to_homogeneous();
				// apply position
				self.model_mat[12] = self.model_pos.x;
				self.model_mat[13] = self.model_pos.y;
				self.model_mat[14] = self.model_pos.z;
				// apply scale
				self.model_mat[0]  *= self.model_scale.x;
				self.model_mat[1]  *= self.model_scale.x;
				self.model_mat[2]  *= self.model_scale.x;
				self.model_mat[4]  *= self.model_scale.y;
				self.model_mat[5]  *= self.model_scale.y;
				self.model_mat[6]  *= self.model_scale.y;
				self.model_mat[8]  *= self.model_scale.z;
				self.model_mat[9]  *= self.model_scale.z;
				self.model_mat[10] *= self.model_scale.z;
		}

		pub fn set_2d_rotation(&mut self, value: f32) {
				let axis_angle = Vec3::z() * value;
				let quat = Quat::new(axis_angle);
				self.set_rotation(&quat);
		}

		pub fn set_scale(&mut self, value: &Vec3) {
				self.model_scale = *value;
				let model_mat = &self.model_mat;
				let scale = Vec3::new(
						value.x / (model_mat[0]*model_mat[0] + model_mat[1]*model_mat[1] + model_mat[2]*model_mat[2]).sqrt(),
				    value.y / (model_mat[4]*model_mat[4] + model_mat[5]*model_mat[5] + model_mat[6]*model_mat[6]).sqrt(),
				    value.z / (model_mat[8]*model_mat[8] + model_mat[9]*model_mat[9] + model_mat[10]*model_mat[10]).sqrt()
				);

				self.model_mat[0] *= scale.x;
				self.model_mat[1] *= scale.x;
				self.model_mat[2] *= scale.x;

				self.model_mat[4] *= scale.y;
				self.model_mat[5] *= scale.y;
				self.model_mat[6] *= scale.y;

				self.model_mat[8] *= scale.z;
				self.model_mat[9] *= scale.z;
				self.model_mat[10] *= scale.z;
		}

		pub fn get_position(&self) -> Vec3 {
				return self.model_pos;
		}

		pub fn get_rotation(&self) -> Quat {
				return self.model_rot;
		}

		pub fn get_2d_rotation(&self) -> f32 {
				return self.model_rot.angle();
		}

		pub fn get_scale(&self) -> Vec3 {
				let model_mat = &self.model_mat;
				return Vec3::new((model_mat[0]*model_mat[0] + model_mat[1]*model_mat[1] + model_mat[2]*model_mat[2]).sqrt(),
				                 (model_mat[4]*model_mat[4] + model_mat[5]*model_mat[5] + model_mat[6]*model_mat[6]).sqrt(),
				                 (model_mat[8]*model_mat[8] + model_mat[9]*model_mat[9] + model_mat[10]*model_mat[10]).sqrt()
						);
		}

}
