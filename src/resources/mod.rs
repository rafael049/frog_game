extern crate glium;
extern crate image;

use std::fs;
use std::io::{Read, Seek};
use std::collections::HashMap;
use std::time::SystemTime;

use glium::texture::SrgbTexture2d;

use std::cell::RefCell;
use std::rc::Rc;

type Rcell<T> = Rc<RefCell<T>>;

#[derive(Copy, Clone)]
pub struct Vertex {
		position: [f32; 3],
		normal:  [f32; 3],
		color: [f32; 3],
		uv: [f32; 2],
}

glium::implement_vertex!(Vertex, position, normal, color, uv);


pub struct Mesh{
		pub id: u64,
		pub vertex: Vec<Vertex>,
		pub index: Vec<u64>
}

pub type Texture = SrgbTexture2d;

pub struct Script {
		pub source: String,
		pub mod_date: SystemTime,
		pub file: fs::File,
}

pub struct Resources {
		meshes: HashMap<String, Rcell<Mesh> >,
		textures: HashMap<String, Rcell<Texture> >,
		scripts: HashMap<String, Rcell<Script>>,
}

fn load_mesh_from_file(_name: &str) -> Mesh {
		Mesh { id: 0,
						vertex:
						vec![
								Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [0.0, 0.0]},
								Vertex { position: [0.5, -0.5, -0.5],  normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [1.0, 0.0]},
								Vertex { position: [0.5,  0.5, -0.5],  normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [1.0, 1.0]},
								Vertex { position: [0.5,  0.5, -0.5],  normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [1.0, 1.0]},
								Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [0.0, 1.0]},
								Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [0.0, 0.0]},

								Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 0.0], uv: [0.0, 0.0]},
								Vertex { position: [0.5, -0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 0.0], uv: [1.0, 0.0]},
								Vertex { position: [0.5,  0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 0.0], uv: [1.0, 1.0]},
								Vertex { position: [0.5,  0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 0.0], uv: [1.0, 1.0]},
								Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 0.0], uv: [0.0, 1.0]},
								Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 0.0], uv: [0.0, 0.0]},

								Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 0.0], uv: [1.0, 0.0]},
								Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 0.0], uv: [1.0, 1.0]},
								Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 0.0], uv: [0.0, 1.0]},
								Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 0.0], uv: [0.0, 1.0]},
								Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 0.0], uv: [0.0, 0.0]},
								Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 0.0], uv: [1.0, 0.0]},

								Vertex { position: [0.5,  0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 1.0, 0.0], uv: [1.0, 0.0]},
								Vertex { position: [0.5,  0.5, -0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 1.0, 0.0], uv: [1.0, 1.0]},
								Vertex { position: [0.5, -0.5, -0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 1.0, 0.0], uv: [0.0, 1.0]},
								Vertex { position: [0.5, -0.5, -0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 1.0, 0.0], uv: [0.0, 1.0]},
								Vertex { position: [0.5, -0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 1.0, 0.0], uv: [0.0, 0.0]},
								Vertex { position: [0.5,  0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 1.0, 0.0], uv: [1.0, 0.0]},

								Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 1.0], uv: [0.0, 1.0]},
								Vertex { position: [0.5, -0.5, -0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 1.0], uv: [1.0, 1.0]},
								Vertex { position: [0.5, -0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 1.0], uv: [1.0, 0.0]},
								Vertex { position: [0.5, -0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 1.0], uv: [1.0, 0.0]},
								Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 1.0], uv: [0.0, 0.0]},
								Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[1.0, 0.0, 1.0], uv: [0.0, 1.0]},

								Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 1.0], uv: [0.0, 1.0]},
								Vertex { position: [0.5,  0.5, -0.5],  normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 1.0], uv: [1.0, 1.0]},
								Vertex { position: [0.5,  0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 1.0], uv: [1.0, 0.0]},
								Vertex { position: [0.5,  0.5,  0.5],  normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 1.0], uv: [1.0, 0.0]},
								Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 1.0], uv: [0.0, 0.0]},
								Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 0.0, 0.0], color:[0.0, 1.0, 1.0], uv: [0.0, 1.0]}
						],
					 index: vec![],
		}
}

impl Resources {
		pub fn new() -> Resources {
				let mut new_meshes = HashMap::new();

				let key = "plane".to_string();
				let mesh = 
						Mesh{
								id: 0,
								vertex: vec![
										Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [0.0, 0.0]},
										Vertex { position: [ 1.0, -1.0, 0.0],  normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [1.0, 0.0]},
										Vertex { position: [ 1.0,  1.0, 0.0],  normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [1.0, 1.0]},

										Vertex { position: [-1.0,  1.0, 0.0],  normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [0.0, 1.0]},
										Vertex { position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [0.0, 0.0]},
										Vertex { position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, 0.0], color:[0.0, 0.0, 1.0], uv: [1.0, 1.0]},
								],
								index: vec![],
						};
				let rcell_mesh = Rc::new(RefCell::new(mesh));
				new_meshes.insert(key, rcell_mesh);

				Resources { meshes: new_meshes, textures: HashMap::new(), scripts: HashMap::new() }
		}

		pub fn get_mesh(&mut self, name: &str) -> Rcell<Mesh>{
				if self.meshes.contains_key(name) {
						return Rc::clone(&self.meshes[name]);
				}
				else {
						let loaded_mesh = load_mesh_from_file(name);
						let rcell_mesh = Rc::new(RefCell::new(loaded_mesh));
						self.meshes.insert(name.to_string(), rcell_mesh);
						return Rc::clone(&self.meshes[name]);
				}
		}

		pub fn get_texture(&mut self, path: &str, display: &glium::Display) -> Rcell<Texture> {
				if self.textures.contains_key(path) {
						return Rc::clone(&self.textures[path]);
				}
				else {
						let image = image::open(path).unwrap().to_rgba8();
						let image_dimensions = image.dimensions();
						let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
						let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();

						let rcell_texture = Rc::new(RefCell::new(texture));

						self.textures.insert(path.to_string(), rcell_texture);
						return Rc::clone(&self.textures[path]);
				}
		}

		pub fn get_script(&mut self, path: &str) -> Rcell<Script> {
				if self.scripts.contains_key(path) {
						return Rc::clone(&self.scripts[path]);
				}
				else {
						let mut file = fs::File::open(&path).unwrap();
						let mod_date = file.metadata().unwrap().modified().unwrap();
						let mut source = String::new();
						file.read_to_string(&mut source).unwrap();

						let script = Script{ file, mod_date, source };
						let rcell_script = Rc::new(RefCell::new(script));

						self.scripts.insert(path.to_string(), rcell_script.clone());

						return rcell_script;
				}
		}

		pub fn reload_scripts(&mut self) {
				for mut rcell_script in self.scripts.values() {
						let mut script = RefCell::borrow_mut(&rcell_script);
						script.file.sync_all().unwrap();
						let mut modified_date = script.file.metadata().unwrap().modified().unwrap();

						if modified_date > script.mod_date {
								script.file.seek(std::io::SeekFrom::Start(0)).unwrap();
								let mut source = String::new();
								script.file.read_to_string(&mut source).unwrap();
								script.source = source;
								script.mod_date = modified_date;
						}
						
				}
		}
}

