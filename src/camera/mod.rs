use crate::utils::*;

pub struct Camera{
		pub position: Vec3,
		pub yaw: f32,
		pub pitch: f32,
		pub row: f32,
		pub front: Vec3,
		pub up: Vec3,
		pub view_mat: Mat4,
		pub proj_mat: Mat4,
		pub zoom: f32,
		pub aspect: f32,
		pub near: f32,
		pub far: f32,
}

impl Camera {
		pub fn new() -> Camera {
				Camera{
						position: vec3(0.0, 0.0, 0.0),
						yaw: 0.0,
						pitch: 0.0,
						row: 0.0,
						front: vec3(0.0, 0.0, -1.0),
						up: vec3(0.0, 1.0, 0.0),
						view_mat: mat4id(),
						proj_mat: mat4id(),
						zoom: 1.0,
						aspect: 1.0,
						near: -1.0,
						far: 1.0,
				}
		}

		pub fn update_view(&mut self) {
				//self.front = vec3(self.pitch.cos()*self.yaw.sin(),
				//												self.pitch.sin(),
				//												-self.pitch.cos()*self.yaw.cos());
				//let o = Point::new(self.position.x, self.position.y, self.position.z);
				//let t = (self.position + self.front);
				//let tp = Point::new(t.x, t.y, t.z);
				//self.view_mat = Mat4::look_at_lh(&o, &tp, &self.up);

				self.view_mat[3*4 + 0] = self.position.x;
				self.view_mat[3*4 + 1] = self.position.y;
		}

		pub fn update_proj(&mut self) {
				self.proj_mat = mat4orto(self.aspect, self.zoom, self.far, self.near);
		}




}
