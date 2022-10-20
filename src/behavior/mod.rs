extern crate nphysics2d;

use nphysics2d::nalgebra as na;

use na::Vector3;
use na::Quaternion;

use mlua::prelude::*;
use std::rc::Weak;
use std::rc::Rc;
use std::cell::RefCell;

type Rcell<T> = Rc<RefCell<T>>;
type Wcell<T> = Weak<RefCell<T>>;

use crate::input::ActionID;
use crate::input::ActionInput;
use crate::input::Input;
use crate::object::Object;
use crate::resources::Script;

pub struct Behavior {
		state: Lua,

		pub script: Rcell<Script>,
		pub target: Wcell<Object>,
}

impl Behavior {
		pub fn new (script: &Rcell<Script>, target: &Wcell<Object>) -> Behavior {
				let state = Lua::new();

				{
						let src = &script.borrow().source;
						state.load(src).exec().unwrap();
						state.load("start()").exec().unwrap();
				}
				
				Behavior {
						state,
						script: script.clone(),
						target: target.clone(),
				}
		}

		pub fn start(&mut self, input: &Input) {
				#[cfg(debug_assertions)]
				self.reload_script();

				self.create_tables(0.0, input);

				{
						//let script = &self.script.borrow().source;
						self.state.load("start()").exec().unwrap();
				}

				self.set_tables();

		}

		pub fn update(&mut self, delta: f32, input: &Input) {

				#[cfg(debug_assertions)]
				self.reload_script();

				self.create_tables(delta, input);

				{
						let script = &self.script.borrow().source;
						let globals = self.state.globals();
						let update_f: mlua::Function = globals.get("update").unwrap();
						update_f.call::<_, ()>(delta).unwrap();
				}

				self.set_tables();

		}

		fn create_tables(&mut self, delta: f32, input: &Input) {
				
				// self table
				let mut pos = Vector3::new(0.0, 0.0, 0.0);
				let mut sca = Vector3::new(0.0, 0.0, 0.0);
				let mut rot = Quaternion::new(0.0, 0.0, 0.0, 0.0);
				let mut obj_ref: &mut Object;

				unsafe {
						obj_ref = &mut*self.target.upgrade().unwrap().as_ptr();
						pos = obj_ref.transform.as_ref().unwrap().get_position();
						//rot = obj_ref.transform.as_ref().unwrap().get_rotation();
						sca = obj_ref.transform.as_ref().unwrap().get_scale();
				}

				let self_table = self.state.create_table().unwrap();

				self_table.set("pos_x", pos.x);
				self_table.set("pos_y", pos.y);
				self_table.set("pos_z", pos.z);
				self_table.set("rot_x", rot.coords.x);
				self_table.set("rot_y", rot.coords.y);
				self_table.set("rot_z", rot.coords.z);
				self_table.set("rot_w", rot.coords.w);
				self_table.set("sca_x", sca.x);
				self_table.set("sca_y", sca.y);
				self_table.set("sca_z", sca.z);

				{
						let globals = self.state.globals();
						globals.set("self", self_table);
				}
				self.set_tables();

				// input table
				let input_table = self.state.create_table().unwrap();

				match input.get_actions(ActionID::MOVE) {
						ActionInput::DIRECTIONAL(dir) => {
								input_table.set("move_x", dir.x );
								input_table.set("move_y", dir.y );
						},
						_ => {
								input_table.set("move_x", 0.0 );
								input_table.set("move_y", 0.0 );
						},
				}
				match input.get_actions(ActionID::ACTION1) {
						ActionInput::DIGITAL(v) => {
								input_table.set("action1", v);
						},
						_ => {
								input_table.set("action1", false );
						},
				}

				{
						let globals = self.state.globals();
						globals.set("input", input_table);
				}

		}

		fn set_tables(&mut self) {
				let globals = self.state.globals();
				let mut obj_ref: &mut Object;
				unsafe {
						obj_ref = &mut*self.target.upgrade().unwrap().as_ptr();
				}

				// set self table
				let self_table = globals.get::<_,LuaTable >("self").unwrap();
				let new_pos = Vector3::new(
						self_table.get::<_, f32>("pos_x").unwrap(),
						self_table.get::<_, f32>("pos_y").unwrap(),
						self_table.get::<_, f32>("pos_z").unwrap(),
				);
				let new_rot = Quaternion::new(
						self_table.get::<_, f32>("rot_x").unwrap(),
						self_table.get::<_, f32>("rot_y").unwrap(),
						self_table.get::<_, f32>("rot_z").unwrap(),
						self_table.get::<_, f32>("rot_w").unwrap(),
				);
				let new_sca = Vector3::new(
						self_table.get::<_, f32>("sca_x").unwrap(),
						self_table.get::<_, f32>("sca_y").unwrap(),
						self_table.get::<_, f32>("sca_z").unwrap(),
				);
				obj_ref.transform.as_mut().unwrap().set_position(&new_pos);
				//obj_ref.transform.as_mut().unwrap().set_rotation(&new_rot);
				obj_ref.transform.as_mut().unwrap().set_scale(&new_sca);

		}

		fn reload_script(&mut self) {
				//TODO: reload somente quando houver alteração
				let script = &self.script.borrow().source;
				self.state.load(script).exec().unwrap();
		}
}
