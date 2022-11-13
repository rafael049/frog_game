use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

use crate::object;
use crate::object::Object;


type Rcell<T> = Rc<RefCell<T>>;
type Wcell<T> = Weak<RefCell<T>>;

type TreeObj = Tree<Object>;

pub struct Tree<T> {
		pub parent: Option< Wcell<TreeObj>>,
		pub children: Vec<Rcell<TreeObj>>,
		pub obj: Rcell<T>,
}

pub struct Scene {
		pub root: Rcell<Tree<Object>>,
		pub table: HashMap<String, Rcell<Tree<Object>> >,

		camera_key: String,
}


impl Scene {
		pub fn new() -> Scene {
				let empty_obj = Rc::new(RefCell::new(Object::new_empty("root")));
				let node = Tree {
						parent: None,
						children: vec![],
						obj: empty_obj,
				};
				let root = Rc::new(RefCell::new(node));
				let mut table = HashMap::new();
				table.insert("root".to_string(), root.clone());
				let camera_key = String::from("camera");

				Scene {
						root,
						table,
						camera_key,
				}
		}

		pub fn add_child(&mut self, parent_key: &str, obj: &Rcell<Object>) {
				let name = RefCell::borrow(&obj).name.clone();
				let parent_node = match self.table.get(parent_key) {
						Some(x) => x.clone(),
						None => panic!("Parent node '{}' does not exist", parent_key),
				};
				let parent_rc = Rc::downgrade(&parent_node);
				let node = Rc::new(RefCell::new(
						Tree {
								parent: Some(parent_rc),
								children: vec![],
								obj: obj.clone()
						}
				));

				self.table.insert(name, node.clone());

				parent_node.borrow_mut().children.push(node);
		}

		pub fn add_camera(&mut self, parent_key: &str, obj: &Rcell<Object>) {
				self.add_child(parent_key, obj);
				let name = RefCell::borrow(&obj).name.clone();
				self.camera_key = name;
		}

		pub fn set_camera(&mut self, key: &str) {
				self.camera_key = String::from(key);
		}

		pub fn get_camera(&self) -> Option<Rcell<TreeObj>> {
				return self.get_node(&self.camera_key);
		}

		pub fn foreach<F>(&self, mut function: F ) where F: FnMut(&object::Object){

				let mut stack = vec![self.root.clone()];

				while !stack.is_empty() {

						let cell_node = stack.pop().unwrap();
						let node = RefCell::borrow(&cell_node);
						let children = &node.children;
						let obj = &node.obj;

						function(&RefCell::borrow_mut(&obj));

						for c in children {
								stack.push(c.clone());
						}
				}
		}
		pub fn foreach_mut<F>(&mut self, mut function: F ) where F: FnMut(&mut object::Object){

				let mut stack = vec![self.root.clone()];

				while !stack.is_empty() {

						let cell_node = stack.pop().unwrap();
						let node = RefCell::borrow(&cell_node);
						let children = &node.children;
						let obj = &node.obj;

						function(&mut*RefCell::borrow_mut(&obj));

						for c in children {
								stack.push(c.clone());
						}
				}
		}

		pub fn get_node(&self, node_key: &str) -> Option<Rcell<TreeObj>> {
				match self.table.get(node_key) {
						Some(x) => Some(x.clone()),
						None => None,
				}
		}
}
