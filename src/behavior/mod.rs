extern crate nphysics3d;

use nphysics3d::nalgebra as na;

use na::Vector3;
use na::Quaternion;

use mlua::prelude::*;
use std::rc::Weak;
use std::rc::Rc;
use std::cell::RefCell;

use crate::utils::*;
use crate::input::ActionID;
use crate::input::ActionInput;
use crate::input::Input;
use crate::object::Object;
use crate::resources::Script;

use self::functor::Functor;

pub mod functor;
pub mod player_behavior;
pub mod camera_behavior;

type UpdateFn = Box<dyn Fn(*mut Object, &Input, f32) -> ()>;


pub struct Behavior {
		pub functor: Box<dyn Functor>,
		pub target: *mut Object,
}

impl Behavior {
		pub fn new (functor: Box<dyn Functor> , target: &mut Object) -> Behavior {
				Behavior {
						functor,
						target,
				}
		}

		pub fn start(&mut self, input: &Input) {

		}

		pub fn update(&mut self, delta: f32, input: &Input) {
				self.functor.update(self.target, input, delta);
		}
}
