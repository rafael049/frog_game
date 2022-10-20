extern crate glium;

use std::fs;
use std::io::Read;
use std::io::Seek;
use std::time::SystemTime;

use glium::program::ProgramChooserCreationError;


#[derive(Debug)]
pub enum ShaderError {
		IoError(std::io::Error),
		ParseError(String),
}

impl From<std::io::Error> for ShaderError {
    fn from(error: std::io::Error) -> Self {
        ShaderError::IoError(error)
    }
}

impl From<ProgramChooserCreationError> for ShaderError {
    fn from(error: ProgramChooserCreationError) -> Self {
        ShaderError::ParseError(error.to_string())
    }
}



pub enum UniformField {
		ModelMatrix,
		LocalMatrix,
		ViewMatrix,
		ProjectionMatrix,
}
		
pub struct Shader {
		program: glium::Program,
		vs_file: fs::File,
		fs_file: fs::File,
		mod_date: SystemTime,
}


impl Shader {
		pub fn new(display: &glium::Display, vs_filename: &str, fs_filename: &str) -> Result<Shader, ShaderError> {

				let mut vs_file = fs::File::open(&vs_filename)?;
				let mut fs_file = fs::File::open(&fs_filename)?;

				let vs_mod = vs_file.metadata()?.modified()?;
				let fs_mod = fs_file.metadata()?.modified()?;

				let mod_date = vs_mod.max(fs_mod);

				let program = create_program(&display, &mut vs_file, &mut fs_file)?;

				Ok(Shader{ program, vs_file, fs_file, mod_date })
		}

		pub fn get_program(&self) -> &glium::Program {
				&self.program
		}

		pub fn reload(&mut self, display: &glium::Display) -> Result<(), ShaderError> {

				self.vs_file.sync_all().unwrap();
				self.fs_file.sync_all().unwrap();
				let vs_modified_date = self.vs_file.metadata()?.modified()?;
				let fs_modified_date = self.fs_file.metadata()?.modified()?;

				let new_mod_date = vs_modified_date.max(fs_modified_date);

				if new_mod_date > self.mod_date {
						self.vs_file.seek(std::io::SeekFrom::Start(0)).unwrap();
						self.fs_file.seek(std::io::SeekFrom::Start(0)).unwrap();
						self.program = create_program(&display, &mut self.vs_file, &mut self.fs_file)?
				}

				Ok(())
		}

}

fn create_program(display: &glium::Display, vs_file: &mut fs::File, fs_file: &mut fs::File) -> Result<glium::Program, ShaderError> {

		let mut vs_src = String::new();
		let mut fs_src = String::new();
		vs_file.read_to_string(&mut vs_src)?;
		fs_file.read_to_string(&mut fs_src)?;

		Ok(glium::program!(display,
				430 => {
						vertex: &vs_src,
						fragment: &fs_src,
				}
		).unwrap_or_else(| e | {
				println!("{:?}", e);
				default_program(display)
		} ))
}

fn default_program(display: &glium::Display) -> glium::Program {
    program!(display,
        140 => {
            vertex: "
                #version 140
                
                uniform mat4 trsf_mat;
                uniform mat4 view_mat;
                uniform mat4 proj_mat;
                
                in vec3 position;
                in vec3 color;
                
                out vec3 vColor;
                
                void main() {
                		gl_Position = (proj_mat*view_mat*trsf_mat) * vec4(position, 1.0) ;
                		vColor = color;
                }
            ",

            fragment: "
                #version 140
                in vec3 vColor;
                out vec4 f_color;
                void main() {
                    f_color = vec4(vColor, 1.0);
                }
            "
        },

    ).unwrap()
}
