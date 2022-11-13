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
mod renderer;

use glium::Display;
use glium::glutin::ContextBuilder;
use glium::glutin::window::WindowBuilder;
#[allow(unused_imports)]
use glium::{glutin, Surface};
use nphysics3d::math::Point;
use nphysics3d::algebra::Force3;
use nphysics3d::algebra::ForceType;
use nphysics3d::math::Velocity;
use nphysics3d::nalgebra::Isometry;
use nphysics3d::nalgebra::Translation3;
use nphysics3d::nalgebra::UnitQuaternion;
use nphysics3d::nalgebra::Vector2;
use nphysics3d::ncollide3d::shape::{Cuboid, ShapeHandle, self};
use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::object::BodyStatus;
use nphysics3d::object::{
    DefaultBodySet, DefaultColliderSet, Ground, ColliderDesc, BodyPartHandle, Body,
};
use nphysics3d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};


use crate::physics::physics_engine::PhysicsEngine;
use crate::physics::{RigidBody, Collider};
use crate::renderer::Renderer;
use crate::utils::*;
use crate::input::{ActionInput, KeyState};


fn main() {
		// Initializing
    let event_loop = glutin::event_loop::EventLoop::new();
		let window_builder = WindowBuilder::new();
		let context_builder = ContextBuilder::new().with_hardware_acceleration(Some(true)).with_vsync(true);
		let display = Display::new(window_builder, context_builder, &event_loop).unwrap();

		// Create renderer
		let mut renderer = Renderer::new(&display);
		
		// Physics Engine
		let mut physics_engine = PhysicsEngine::new();

		// Add ground
		let ground_shape = ShapeHandle::new(shape::Polyline::new(vec![Point::new(-5.0, -10.0, 0.0), Point::new(5.0, -10.0, 0.0)], None));
		let ground_handle = physics_engine.bodies.insert(Ground::new());
		let ground_co = ColliderDesc::new(ground_shape)
				.translation(Vec3::new(0.0, -3.0, 0.0))
				.build(BodyPartHandle(ground_handle, 0));
		physics_engine.colliders.insert(ground_co);

		// Mouse position
		let mut mouse_pos: (u32, u32) = (0, 0);

		// Resource manager
		let mut rsrc = resources::Resources::new();
		// Input manager
		let mut input = input::Input::new();

		// Main camera
		let camera_behavior = behavior::camera_behavior::CameraBehavior::new();
		let mut camera_obj = object::Object::new_empty("camera");
		let mut camera = object::camera::Camera::new();
		let aspect = renderer.window_size.0 as f32 / renderer.window_size.1 as f32;
		camera.aspect = aspect;
		camera.zoom = 2.0;
		camera_obj.camera.replace(camera);
		let camera_obj = Rc::new(RefCell::new(camera_obj));
		let behavior = behavior::Behavior::new(Box::new(camera_behavior), &mut camera_obj.borrow_mut());
		RefCell::borrow_mut(&camera_obj).behavior = Some(behavior);

		// Main shader
		let mut shader = shader::Shader::new(&display, "shaders/vertex_default.glsl", "shaders/fragment_default.glsl").unwrap();

		// Textures
    let sapo_texture = rsrc.get_texture("textures/sapo.png", &display);
    let texture2 = rsrc.get_texture("textures/teste2.png", &display);
    let mapa_tex = rsrc.get_texture("textures/mapa_teste.png", &display);
		// Script
		let script_teste = rsrc.get_script("scripts/teste.lua");

		// Plane vertex buffer
    let vertex_buffer = {
				let mesh = rsrc.get_mesh("plane");
				let mesh_b = (*mesh).borrow();
        glium::VertexBuffer::new(&display,
																 &mesh_b.vertex
        ).unwrap()
    };
		let rcell_vertex_buffer = Rc::new(RefCell::new(vertex_buffer));

		// Create objs
		let mut player_obj = object::Object::new_handler("player", rcell_vertex_buffer.clone(), sapo_texture.clone());
		let mut obj2 = object::Object::new_handler("obj2", rcell_vertex_buffer.clone(), texture2.clone());
		let mut mapa_obj = object::Object::new_handler("mapa_teste", rcell_vertex_buffer.clone(), mapa_tex);

		// Add Behavior
		{
				let player_behavior = Box::new(behavior::player_behavior::PlayerBehavior::new());
				let behavior = behavior::Behavior::new(player_behavior, &mut player_obj.borrow_mut());
				RefCell::borrow_mut(&player_obj).behavior = Some(behavior);
		}

		// Add Rigid Body to obj1
		let rigid_body = RigidBody::new(&mut physics_engine.bodies, BodyStatus::Dynamic, -1.0, 2.0);
    let cuboid = ShapeHandle::new(Cuboid::new(vec3(0.5, 0.5, 1.0)));
		// Build the collider.
		let collider = Collider::new(&mut physics_engine.colliders, rigid_body.handle, cuboid);

		player_obj.borrow_mut().rigid_body.replace(rigid_body);
		player_obj.borrow_mut().collider.replace(collider);

		// Add Rigid Body to obj2
		let rigid_body = RigidBody::new(&mut physics_engine.bodies, BodyStatus::Dynamic, -5.0, 0.0);
    let cuboid = ShapeHandle::new(shape::Cuboid::new(vec3(0.5, 0.5, 1.0)));
		// Build the collider.
		let collider = Collider::new(&mut physics_engine.colliders, rigid_body.handle, cuboid);

		obj2.borrow_mut().rigid_body.replace(rigid_body);
		obj2.borrow_mut().collider.replace(collider);

		// Add Body to mapa_teste
		let map_scale = 32.0;
		let rigid_body = RigidBody::new(&mut physics_engine.bodies, BodyStatus::Static, 0.0, 0.0);
    let map_shape = ShapeHandle::new(shape::Compound::new(vec![
				(Isometry::from_parts(Translation3::new(map_scale*0.0, map_scale*(-0.4375), 0.0), UnitQuaternion::identity()), ShapeHandle::new(shape::Cuboid::new(vec3(1.0, 0.125, 1.0) * map_scale/2.0))),
				(Isometry::from_parts(Translation3::new(map_scale*-0.375,map_scale* -0.125, 0.0), UnitQuaternion::identity()), ShapeHandle::new(shape::Cuboid::new(vec3(0.250, 0.5, 1.0)* map_scale/2.0))),
				(Isometry::from_parts(Translation3::new(map_scale*-0.125,map_scale* -0.250, 0.0), UnitQuaternion::identity()), ShapeHandle::new(shape::Cuboid::new(vec3(0.250, 0.250, 1.0)* map_scale/2.0))),
				(Isometry::from_parts(Translation3::new(map_scale*0.375, map_scale*-0.1875, 0.0), UnitQuaternion::identity()), ShapeHandle::new(shape::Cuboid::new(vec3(0.250, 0.375, 1.0) * map_scale/2.0))),
				]));
    //let cuboid = ShapeHandle::new(shape::Cuboid::new(vec3(1.0, 1.0, 1.0)));

		// Build the collider.
		let collider = Collider::new(&mut physics_engine.colliders, rigid_body.handle, map_shape);

		mapa_obj.borrow_mut().rigid_body.replace(rigid_body);
		mapa_obj.borrow_mut().collider.replace(collider);

		// Position and scale of the map
		mapa_obj.borrow_mut().transform.as_mut().unwrap().set_scale(&vec3(map_scale, map_scale, 1.0));

		let mut scene = scene::scene_tree::Scene::new();
		scene.add_child("root", &player_obj);
		scene.add_child("root", &obj2);
		scene.add_child("root", &mapa_obj);
		scene.add_camera("root", &camera_obj);

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

                // Update window size
                glutin::event::WindowEvent::Resized(physical_size) => {
										renderer.window_size.0 = physical_size.width;
										renderer.window_size.1 = physical_size.height;
										camera_obj.borrow_mut().camera.as_mut().unwrap().aspect = renderer.window_size.0 as f32 / renderer.window_size.1 as f32;
                },

								// Process keyboard input
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

								// Process mouse position
								glutin::event::WindowEvent::CursorMoved { position: pos, .. } => {
										mouse_pos = (pos.x as u32, pos.y as u32);
								},

								// Process mouse input
								glutin::event::WindowEvent::MouseInput { state: s, button: but, ..} =>
										input.process_mouse_buttons(s, but),


								// Process mouse wheel
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
										
                _ => {},
            },

						// Rendering
						glutin::event::Event::MainEventsCleared => {

								// Calculate time and delta
								let current_instant = Instant::now();
								let delta_time = current_instant.duration_since(last_instant).as_secs_f32();
								last_instant = current_instant;
								total_time += delta_time;

								renderer.render(&display, &scene, &shader, total_time);

								// Reload shader
								//let _ = shader.reload(&display);
								//rsrc.reload_scripts();

								// Process Input
								input.process_mouse_move(mouse_pos.0, mouse_pos.1, renderer.window_size);

								// update objects behavior
								scene.foreach_mut(|obj| {
										obj.update(delta_time, &input);
								});


								physics_engine.step(&mut scene);


								input.update();
						}
            _ => {}, // *control_flow = glutin::event_loop::ControlFlow::Poll,
        };
    });
}
