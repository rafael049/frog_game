#[macro_use]
extern crate glium;
extern crate image;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

mod utils;
mod resources;
mod behavior;
mod camera;
mod input;
mod object;
mod shader;
mod scene;
mod physics;

#[allow(unused_imports)]
use glium::{glutin, Surface};
use nalgebra::ComplexField;
use nphysics2d::nalgebra::{Vector2, Unit};
use nphysics2d::ncollide2d::shape::{Cuboid, ShapeHandle, self};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::object::{
    DefaultBodySet, DefaultColliderSet, Ground, ColliderDesc, BodyPartHandle,
};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};


use crate::physics::{RigidBody, Collider};
use crate::utils::*;
use crate::behavior::Behavior;
use crate::input::{ActionInput, KeyState};


fn main() {

		let mut mechanical_world = DefaultMechanicalWorld::<f32>::new(vec2(0.0f32, -9.81f32));
    let mut geometrical_world = DefaultGeometricalWorld::<f32>::new();

    let mut bodies = DefaultBodySet::<f32>::new();
    let mut colliders = DefaultColliderSet::<f32>::new();

    let mut joint_constraints = DefaultJointConstraintSet::<f32>::new();
    let mut force_generators = DefaultForceGeneratorSet::<f32>::new();


		// Add ground
		let ground_shape = ShapeHandle::new(Cuboid::new(Vec2::new(100.0, 1.0)));
		let ground_handle = bodies.insert(Ground::new());
		let ground_co = ColliderDesc::new(ground_shape)
				.translation(Vec2::new(0.0, -3.0))
				.build(BodyPartHandle(ground_handle, 0));
		colliders.insert(ground_co);


    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_hardware_acceleration(Some(true)).with_vsync(true);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

		let mut window_size: (u32, u32) = (display.gl_window().window().inner_size().width,
																			 display.gl_window().window().inner_size().height);
		let mut mouse_pos: (u32, u32) = (0, 0);

		let mut rsrc = resources::Resources::new();
		let mut camera = camera::Camera::new();
		let mut input = input::Input::new();
		//let mut object = object::Object::new();
		let mut shader = shader::Shader::new(&display, "shaders/vertex_default.glsl", "shaders/fragment_default.glsl").unwrap();
    let texture = rsrc.get_texture("textures/teste.png", &display);
    let texture2 = rsrc.get_texture("textures/teste2.png", &display);
		let script_teste = rsrc.get_script("scripts/teste.lua");


		let aspect = window_size.0 as f32 / window_size.1 as f32;
		camera.aspect = aspect;
		camera.zoom = 2.0;

    let vertex_buffer = {
				let mesh = rsrc.get_mesh("plane");
				let mesh_b = (*mesh).borrow();
        glium::VertexBuffer::new(&display,
																 &mesh_b.vertex
        ).unwrap()
    };
		let rcell_vertex_buffer = Rc::new(RefCell::new(vertex_buffer));

		let mut obj1 = object::Object::new_handler("obj1", rcell_vertex_buffer.clone(), texture.clone());
		let mut obj2 = object::Object::new_handler("obj2", rcell_vertex_buffer.clone(), texture2.clone());
		// Add Behavior
		let down = Rc::downgrade(&obj2);
		let behavior = Behavior::new(&script_teste, &down);
		RefCell::borrow_mut(&obj2).behavior = Some(behavior);

		// Add Rigid Body
		let rigid_body = RigidBody::new(&mut bodies, 1.5, 5.0);
    let cuboid = ShapeHandle::new(Cuboid::new(Vector2::repeat(1.0)));
		// Build the collider.
		let collider = Collider::new(&mut colliders, rigid_body.handle, cuboid);

		obj1.borrow_mut().rigid_body.replace(rigid_body);
		obj1.borrow_mut().collider.replace(collider);

		// Add Rigid Body
		let rigid_body = RigidBody::new(&mut bodies, 3.3, 2.0);
    let cuboid = ShapeHandle::new(shape::Ball::new(1.0));
		// Build the collider.
		let collider = Collider::new(&mut colliders, rigid_body.handle, cuboid);

		obj2.borrow_mut().rigid_body.replace(rigid_body);
		obj2.borrow_mut().collider.replace(collider);

		let mut scene = scene::scene_tree::Scene::new();
		//obj1.transform.as_mut().unwrap().model_mat[4] = 1.5;
		scene.add_child("root", &obj1);
		scene.add_child("root", &obj2);

		let params = glium::DrawParameters {
				depth: glium::Depth {
						test: glium::draw_parameters::DepthTest::IfLess,
						write: true,
						.. Default::default()
				},
				.. Default::default()
		};

		use glutin::event::{KeyboardInput, VirtualKeyCode};
		use glutin::event_loop::ControlFlow;

		let mut last_instant = Instant::now();
		let mut total_time = 0.0;
		let mut delta = 0.0;

    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => {
										*control_flow = glutin::event_loop::ControlFlow::Exit;
										return;
								},
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(physical_size) => {
										window_size.0 = physical_size.width;
										window_size.1 = physical_size.height;
										camera.aspect = window_size.0 as f32 / window_size.1 as f32;
                },
								glutin::event::WindowEvent::KeyboardInput {
										input: KeyboardInput { virtual_keycode: Some(virtual_code), state, .. },
										..
								} => match (virtual_code, state) {
										(VirtualKeyCode::Escape, _) => {
												*control_flow = ControlFlow::Exit;
												return;
										},
										_ => input.process_event(state, virtual_code)
								}
								glutin::event::WindowEvent::CursorMoved { position: pos, .. } => {
										mouse_pos = (pos.x as u32, pos.y as u32);
										//display.gl_window().window() .set_cursor_position(LogicalPosition::new(window_size.0 as f32 /2.0, window_size.1 as f32 /2.0));
								},
								glutin::event::WindowEvent::MouseInput { state: s, button: but, ..} =>
										input.process_mouse_buttons(s, but),

								glutin::event::WindowEvent::MouseWheel { delta , ..} => {
										match delta {
												glutin::event::MouseScrollDelta::PixelDelta(v) => {
														input.mouse_wheel = v.y as f32;
												},
												glutin::event::MouseScrollDelta::LineDelta(a, b) => {
														input.mouse_wheel = b as f32;
												}
										}
								}
										
                _ => {}, //*control_flow = glutin::event_loop::ControlFlow::Poll,
            },


						// Rendering
						glutin::event::Event::MainEventsCleared => {

								// Calculate time and delta
								let current_instant = Instant::now();
								let delta_time = current_instant.duration_since(last_instant).as_secs_f32();
								last_instant = current_instant;
								total_time += delta_time;

								//println!("FPS:{}", 1.0/delta_time);

								//objects[0].transform.model_mat = rotate(&objects[0].transform.model_mat, 1.0*delta_time, &::vec3(0.0, 1.0, 1.0));

								// get screen resulution
								let resolution = vec2(window_size.0 as f32, window_size.1 as f32);

								// makes camera look to target
								let aspect = window_size.0 as f32 / window_size.1 as f32;
								camera.update_proj();
								camera.update_view();

								// Initiate Rendering
								let mut target = display.draw();
								target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);

								// Render objects
								scene.foreach( |obj| {
										// Get texture
										let texture_cell = match &obj.texture {
												Some(t) => t,
												None => return,
										};
										let texture = &*RefCell::borrow(&texture_cell);

										// Get transform
										let transform = match &obj.transform {
												Some(t) => t,
												None => return,
										};

										// Get vertexBuffer
										let vertex_buffer = match &obj.vertex_buffer {
												Some(t) => t,
												None => return,
										};
										let vertex_buffer = &*RefCell::borrow(&vertex_buffer);

										// Get indexBuffer
										let index_buffer = match &obj.index_buffer {
												Some(t) => t,
												None => return,
										};
										let index_buffer = index_buffer;

										let id = mat4id();

										let uniforms = uniform!{
												view_mat: *camera.view_mat.as_ref(),
												proj_mat: *camera.proj_mat.as_ref(),
												iTime: total_time,
												iResolution: *resolution.as_ref(),
												trsf_mat: *transform.model_mat.as_ref(),
												tex: texture 
										};

										target.draw(vertex_buffer, index_buffer, &shader.get_program(), &uniforms, &params).unwrap();
								});



								// Finish rendering
								target.finish().unwrap();

								// Reload shader
								let _ = shader.reload(&display);
								rsrc.reload_scripts();

								// Process Input
								input.process_mouse_move(mouse_pos.0, mouse_pos.1, window_size);

								// update objects
								(*scene.get_node("obj2").unwrap().borrow_mut()).obj.borrow_mut().update(delta_time, &input);

								let move_sentivity = 10.0;
								let look_sentivity = 200.0;

								if input.get_mouse_button(input::MouseButtonKeyCode::Right) == KeyState::KeyDown {
										match input.get_actions(input::ActionID::LOOK) {
												ActionInput::DIRECTIONAL(dir) => {
														camera.position.x += delta_time * look_sentivity * dir.x * camera.zoom;
														camera.position.y -= delta_time * look_sentivity * dir.y * camera.zoom;
												}
												_ => (),
										}
								}

								match input.get_actions(input::ActionID::ZOOM) {
										ActionInput::ANALOGIC(v) => if camera.zoom - v*1.05 > 0.0 {
												camera.zoom -= v*1.05;
										}
										else {
												camera.zoom = 0.05;
										}
										_ => {}
								}


								match input.get_actions(input::ActionID::MOVE) {
										ActionInput::DIRECTIONAL(dir) => {
												//let front = camera.front * dir.y;
												//let right = camera.front.cross(&camera.up) * dir.x;
												//camera.position += delta_time*move_sentivity * (front + right);
										},
										_ => (),
								}


								// physics
								scene.foreach_mut(|obj| {
										match obj.rigid_body.as_ref() {
												Some(rb) => {
														let rb_handle = rb.handle;
														let rigid_body = bodies.rigid_body(rb_handle).unwrap();
														let v = rigid_body.position().translation;
														let pos = Vec3::new(v.x, v.y, 0.0);
														let angle = rigid_body.position().rotation.angle();
														obj.transform.as_mut().unwrap().set_position(&pos);
														obj.transform.as_mut().unwrap().set_2d_rotation(angle);
												},
												None => {},
										}
								});

								// Run the physics simulation.
								mechanical_world.step(
										&mut geometrical_world,
										&mut bodies,
										&mut colliders,
										&mut joint_constraints,
										&mut force_generators
								);


								input.update();
						}
            _ => {}, // *control_flow = glutin::event_loop::ControlFlow::Poll,
        };
    });
}
