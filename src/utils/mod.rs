pub extern crate nphysics3d;

use nphysics3d::nalgebra as na;


use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

pub type Rcell<T> = Rc<RefCell<T>>;
pub type Wcell<T> = Weak<RefCell<T>>;

//impl Rcell<T> {
//		pub fn new(v: T) -> Rcell<T> {
//				Rc::new(RefCell::new(v))
//		}
//}

pub type Vec2 = na::Vector2<f32>;
pub type Vec3 = na::Vector3<f32>;
pub type Mat4 = na::Matrix4<f32>;
pub type Quat = na::UnitQuaternion<f32>;
pub type Point = na::Point3<f32>;

pub fn vec2(x: f32, y:f32) -> Vec2 { Vec2::new(x, y) }
pub fn vec3(x: f32, y:f32, z:f32) -> Vec3 { Vec3::new(x, y, z) }
pub fn mat4id() -> Mat4 { Mat4::identity() }
pub fn mat4perspective(aspect:f32, fov:f32, near:f32, far:f32) -> Mat4 {
		*na::geometry::Perspective3::new(aspect, fov, near, far).as_matrix()
}
pub fn mat4orto(aspect:f32, zoom: f32, near:f32, far:f32) -> Mat4 {
		*na::geometry::Orthographic3::new(-aspect*zoom, aspect*zoom, -zoom, zoom, near, far).as_matrix()
}
