use crate::input;
use crate::object;

pub trait Functor {
		fn start(&mut self, target: *mut object::Object, input: &input::Input, delta: f32);
		fn update(&mut self, target: *mut object::Object, input: &input::Input, delta: f32);
}
