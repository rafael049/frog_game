extern crate glium;

use crate::utils::*;

use glium::glutin::event::{VirtualKeyCode, ElementState, MouseButton};
use std::collections::HashMap;


#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum KeyCode {
		Key1,
		Key2,
		Key3,
		Key4,
		Key5,
		Key6,
		Key7,
		Key8,
		Key9,
		Key0,

		A,
		B,
		C,
		D,
		E,
		F,
		G,
		H,
		I,
		J,
		K,
		L,
		M,
		N,
		O,
		P,
		Q,
		R,
		S,
		T,
		U,
		V,
		W,
		X,
		Y,
		Z,

		Escape,

		F1,
		F2,
		F3,
		F4,
		F5,
		F6,
		F7,
		F8,
		F9,
		F10,
		F11,
		F12,
		F13,
		F14,
		F15,
		F16,
		F17,
		F18,
		F19,
		F20,
		F21,
		F22,
		F23,
		F24,

		Snapshot,
		Scroll,
		Pause,

		Insert,
		Home,
		Delete,
		End,
		PageDown,
		PageUp,

		Left,
		Up,
		Right,
		Down,

		// TODO: rename
		Back,
		Return,
		Space,

		Compose,

		Caret,

		Numlock,
		Numpad0,
		Numpad1,
		Numpad2,
		Numpad3,
		Numpad4,
		Numpad5,
		Numpad6,
		Numpad7,
		Numpad8,
		Numpad9,
		NumpadAdd,
		NumpadDivide,
		NumpadDecimal,
		NumpadComma,
		NumpadEnter,
		NumpadEquals,
		NumpadMultiply,
		NumpadSubtract,

		AbntC1,
		AbntC2,
		Apostrophe,
		Apps,
		Asterisk,
		At,
		Ax,
		Backslash,
		Calculator,
		Capital,
		Colon,
		Comma,
		Convert,
		Equals,
		Grave,
		Kana,
		Kanji,
		LAlt,
		LBracket,
		LControl,
		LShift,
		LWin,
		Mail,
		MediaSelect,
		MediaStop,
		Minus,
		Mute,
		MyComputer,
		// also called "Next"
		NavigateForward,
		// also called "Prior"
		NavigateBackward,
		NextTrack,
		NoConvert,
		OEM102,
		Period,
		PlayPause,
		Plus,
		Power,
		PrevTrack,
		RAlt,
		RBracket,
		RControl,
		RShift,
		RWin,
		Semicolon,
		Slash,
		Sleep,
		Stop,
		Sysrq,
		Tab,
		Underline,
		Unlabeled,
		VolumeDown,
		VolumeUp,
		Wake,
		WebBack,
		WebFavorites,
		WebForward,
		WebHome,
		WebRefresh,
		WebSearch,
		WebStop,
		Yen,
		Copy,
		Paste,
		Cut,
}

pub enum MouseButtonKeyCode {
		Right,
		Left,
		Middle
}

#[derive(Copy, Clone, PartialEq)]
pub enum KeyState {
		KeyDown,
		KeyUp,
}


pub enum ActionID {
		MOVE,
		LOOK,
		ZOOM,
		ACTION1,
		JUMP,
}

pub enum ActionInput {
		DIRECTIONAL(Vec2),
		ANALOGIC(f32),
		DIGITAL(bool),
		NONE,
}


pub struct Input {
		keys_state: Vec<KeyState>,
		mouse_buttons_state: Vec<KeyState>,
		mouse_position: (f32, f32),
		mouse_delta: (f32, f32),
		to_glutin: HashMap<KeyCode, VirtualKeyCode>,
		pub mouse_wheel: f32,
}

impl Input {
		pub fn new() ->Input {
				Input{ keys_state: vec![KeyState::KeyUp; 256],
							 mouse_buttons_state: vec![KeyState::KeyUp; 3],
							 mouse_position: (0.0, 0.0),
							 mouse_delta: (0.0, 0.0),
							 mouse_wheel: 0.0,
							 to_glutin: 
							 HashMap::from([
										(KeyCode::Key1, VirtualKeyCode::Key1),
										(KeyCode::Key2, VirtualKeyCode::Key2),
										(KeyCode::Key3, VirtualKeyCode::Key3),
										(KeyCode::Key4, VirtualKeyCode::Key4),
										(KeyCode::Key5, VirtualKeyCode::Key5),
										(KeyCode::Key6, VirtualKeyCode::Key6),
										(KeyCode::Key7, VirtualKeyCode::Key7),
										(KeyCode::Key8, VirtualKeyCode::Key8),
										(KeyCode::Key9, VirtualKeyCode::Key9),
										(KeyCode::Key0, VirtualKeyCode::Key0),

										(KeyCode::A, VirtualKeyCode::A),
										(KeyCode::B, VirtualKeyCode::B),
										(KeyCode::C, VirtualKeyCode::C),
										(KeyCode::D, VirtualKeyCode::D),
										(KeyCode::E, VirtualKeyCode::E),
										(KeyCode::F, VirtualKeyCode::F),
										(KeyCode::G, VirtualKeyCode::G),
										(KeyCode::H, VirtualKeyCode::H),
										(KeyCode::I, VirtualKeyCode::I),
										(KeyCode::J, VirtualKeyCode::J),
										(KeyCode::K, VirtualKeyCode::K),
										(KeyCode::L, VirtualKeyCode::L),
										(KeyCode::M, VirtualKeyCode::M),
										(KeyCode::N, VirtualKeyCode::N),
										(KeyCode::O, VirtualKeyCode::O),
										(KeyCode::P, VirtualKeyCode::P),
										(KeyCode::Q, VirtualKeyCode::Q),
										(KeyCode::R, VirtualKeyCode::R),
										(KeyCode::S, VirtualKeyCode::S),
										(KeyCode::T, VirtualKeyCode::T),
										(KeyCode::U, VirtualKeyCode::U),
										(KeyCode::V, VirtualKeyCode::V),
										(KeyCode::W, VirtualKeyCode::W),
										(KeyCode::X, VirtualKeyCode::X),
										(KeyCode::Y, VirtualKeyCode::Y),
										(KeyCode::Z, VirtualKeyCode::Z),

										(KeyCode::Escape, VirtualKeyCode::Escape),

										(KeyCode::F1, VirtualKeyCode::F1),
										(KeyCode::F2, VirtualKeyCode::F2),
										(KeyCode::F3, VirtualKeyCode::F3),
										(KeyCode::F4, VirtualKeyCode::F4),
										(KeyCode::F5, VirtualKeyCode::F5),
										(KeyCode::F6, VirtualKeyCode::F6),
										(KeyCode::F7, VirtualKeyCode::F7),
										(KeyCode::F8, VirtualKeyCode::F8),
										(KeyCode::F9, VirtualKeyCode::F9),
										(KeyCode::F10, VirtualKeyCode::F10),
										(KeyCode::F11, VirtualKeyCode::F11),
										(KeyCode::F12, VirtualKeyCode::F12),
										(KeyCode::F13, VirtualKeyCode::F13),
										(KeyCode::F14, VirtualKeyCode::F14),
										(KeyCode::F15, VirtualKeyCode::F15),
										(KeyCode::F16, VirtualKeyCode::F16),
										(KeyCode::F17, VirtualKeyCode::F17),
										(KeyCode::F18, VirtualKeyCode::F18),
										(KeyCode::F19, VirtualKeyCode::F19),
										(KeyCode::F20, VirtualKeyCode::F20),
										(KeyCode::F21, VirtualKeyCode::F21),
										(KeyCode::F22, VirtualKeyCode::F22),
										(KeyCode::F23, VirtualKeyCode::F23),
										(KeyCode::F24, VirtualKeyCode::F24),

										(KeyCode::Snapshot, VirtualKeyCode::Snapshot),
										(KeyCode::Scroll, VirtualKeyCode::Scroll),
										(KeyCode::Pause, VirtualKeyCode::Pause),

										(KeyCode::Insert, VirtualKeyCode::Insert),
										(KeyCode::Home, VirtualKeyCode::Home),
										(KeyCode::Delete, VirtualKeyCode::Delete),
										(KeyCode::End, VirtualKeyCode::End),
										(KeyCode::PageDown, VirtualKeyCode::PageDown),
										(KeyCode::PageUp, VirtualKeyCode::PageUp),

										(KeyCode::Left, VirtualKeyCode::Left),
										(KeyCode::Up, VirtualKeyCode::Up),
										(KeyCode::Right, VirtualKeyCode::Right),
										(KeyCode::Down, VirtualKeyCode::Down),

										// TODO: rename
										(KeyCode::Back, VirtualKeyCode::Back),
										(KeyCode::Return, VirtualKeyCode::Return),
										(KeyCode::Space, VirtualKeyCode::Space),

										(KeyCode::Compose, VirtualKeyCode::Compose),

										(KeyCode::Caret, VirtualKeyCode::Caret),

										(KeyCode::Numlock, VirtualKeyCode::Numlock),
										(KeyCode::Numpad0, VirtualKeyCode::Numpad0),
										(KeyCode::Numpad1, VirtualKeyCode::Numpad1),
										(KeyCode::Numpad2, VirtualKeyCode::Numpad2),
										(KeyCode::Numpad3, VirtualKeyCode::Numpad3),
										(KeyCode::Numpad4, VirtualKeyCode::Numpad4),
										(KeyCode::Numpad5, VirtualKeyCode::Numpad5),
										(KeyCode::Numpad6, VirtualKeyCode::Numpad6),
										(KeyCode::Numpad7, VirtualKeyCode::Numpad7),
										(KeyCode::Numpad8, VirtualKeyCode::Numpad8),
										(KeyCode::Numpad9, VirtualKeyCode::Numpad9),
										(KeyCode::NumpadAdd, VirtualKeyCode::NumpadAdd),
										(KeyCode::NumpadDivide, VirtualKeyCode::NumpadDivide),
										(KeyCode::NumpadDecimal, VirtualKeyCode::NumpadDecimal),
										(KeyCode::NumpadComma, VirtualKeyCode::NumpadComma),
										(KeyCode::NumpadEnter, VirtualKeyCode::NumpadEnter),
										(KeyCode::NumpadEquals, VirtualKeyCode::NumpadEquals),
										(KeyCode::NumpadMultiply, VirtualKeyCode::NumpadMultiply),
										(KeyCode::NumpadSubtract, VirtualKeyCode::NumpadSubtract),

										(KeyCode::AbntC1, VirtualKeyCode::AbntC1),
										(KeyCode::AbntC2, VirtualKeyCode::AbntC2),
										(KeyCode::Apostrophe, VirtualKeyCode::Apostrophe),
										(KeyCode::Apps, VirtualKeyCode::Apps),
										(KeyCode::Asterisk, VirtualKeyCode::Asterisk),
										(KeyCode::At, VirtualKeyCode::At),
										(KeyCode::Ax, VirtualKeyCode::Ax),
										(KeyCode::Backslash, VirtualKeyCode::Backslash),
										(KeyCode::Calculator, VirtualKeyCode::Calculator),
										(KeyCode::Capital, VirtualKeyCode::Capital),
										(KeyCode::Colon, VirtualKeyCode::Colon),
										(KeyCode::Comma, VirtualKeyCode::Comma),
										(KeyCode::Convert, VirtualKeyCode::Convert),
										(KeyCode::Equals, VirtualKeyCode::Equals),
										(KeyCode::Grave, VirtualKeyCode::Grave),
										(KeyCode::Kana, VirtualKeyCode::Kana),
										(KeyCode::Kanji, VirtualKeyCode::Kanji),
										(KeyCode::LAlt, VirtualKeyCode::LAlt),
										(KeyCode::LBracket, VirtualKeyCode::LBracket),
										(KeyCode::LControl, VirtualKeyCode::LControl),
										(KeyCode::LShift, VirtualKeyCode::LShift),
										(KeyCode::LWin, VirtualKeyCode::LWin),
										(KeyCode::Mail, VirtualKeyCode::Mail),
										(KeyCode::MediaSelect, VirtualKeyCode::MediaSelect),
										(KeyCode::MediaStop, VirtualKeyCode::MediaStop),
										(KeyCode::Minus, VirtualKeyCode::Minus),
										(KeyCode::Mute, VirtualKeyCode::Mute),
										(KeyCode::MyComputer, VirtualKeyCode::MyComputer),
										// also called "Next"
										(KeyCode::NavigateForward, VirtualKeyCode::NavigateForward),
										// also called "Prior"
										(KeyCode::NavigateBackward, VirtualKeyCode::NavigateBackward),
										(KeyCode::NextTrack, VirtualKeyCode::NextTrack),
										(KeyCode::NoConvert, VirtualKeyCode::NoConvert),
										(KeyCode::OEM102, VirtualKeyCode::OEM102),
										(KeyCode::Period, VirtualKeyCode::Period),
										(KeyCode::PlayPause, VirtualKeyCode::PlayPause),
										(KeyCode::Plus, VirtualKeyCode::Plus),
										(KeyCode::Power, VirtualKeyCode::Power),
										(KeyCode::PrevTrack, VirtualKeyCode::PrevTrack),
										(KeyCode::RAlt, VirtualKeyCode::RAlt),
										(KeyCode::RBracket, VirtualKeyCode::RBracket),
										(KeyCode::RControl, VirtualKeyCode::RControl),
										(KeyCode::RShift, VirtualKeyCode::RShift),
										(KeyCode::RWin, VirtualKeyCode::RWin),
										(KeyCode::Semicolon, VirtualKeyCode::Semicolon),
										(KeyCode::Slash, VirtualKeyCode::Slash),
										(KeyCode::Sleep, VirtualKeyCode::Sleep),
										(KeyCode::Stop, VirtualKeyCode::Stop),
										(KeyCode::Sysrq, VirtualKeyCode::Sysrq),
										(KeyCode::Tab, VirtualKeyCode::Tab),
										(KeyCode::Underline, VirtualKeyCode::Underline),
										(KeyCode::Unlabeled, VirtualKeyCode::Unlabeled),
										(KeyCode::VolumeDown, VirtualKeyCode::VolumeDown),
										(KeyCode::VolumeUp, VirtualKeyCode::VolumeUp),
										(KeyCode::Wake, VirtualKeyCode::Wake),
										(KeyCode::WebBack, VirtualKeyCode::WebBack),
										(KeyCode::WebFavorites, VirtualKeyCode::WebFavorites),
										(KeyCode::WebForward, VirtualKeyCode::WebForward),
										(KeyCode::WebHome, VirtualKeyCode::WebHome),
										(KeyCode::WebRefresh, VirtualKeyCode::WebRefresh),
										(KeyCode::WebSearch, VirtualKeyCode::WebSearch),
										(KeyCode::WebStop, VirtualKeyCode::WebStop),
										(KeyCode::Yen, VirtualKeyCode::Yen),
										(KeyCode::Copy, VirtualKeyCode::Copy),
										(KeyCode::Paste, VirtualKeyCode::Paste),
										(KeyCode::Cut, VirtualKeyCode::Cut),
								]),
				}
		}

		pub fn update(&mut self) {
				// reset mouse wheel
				self.mouse_wheel = 0.0;
		}

		pub fn process_event(&mut self, key_state: ElementState, code: VirtualKeyCode) {
				match key_state {
						ElementState::Pressed => {
								self.keys_state[code as usize] = KeyState::KeyDown;
						},
						ElementState::Released => {
								self.keys_state[code as usize] = KeyState::KeyUp;
						}
				}
		}

		pub fn process_mouse_move(&mut self, x: u32, y: u32, window_size: (u32, u32)) {
				let norm = (x as f32 / window_size.1 as f32,
										y as f32 / window_size.1 as f32);

				self.mouse_delta = (norm.0 - self.mouse_position.0,
				                    norm.1 - self.mouse_position.1);
				self.mouse_position = norm;
		}

		pub fn process_mouse_buttons(&mut self, state: ElementState, button: MouseButton) {

				if state == ElementState::Pressed {
						match button {
								MouseButton::Right => self.mouse_buttons_state[0] = KeyState::KeyDown,
								MouseButton::Left => self.mouse_buttons_state[1] = KeyState::KeyDown,
								MouseButton::Middle => self.mouse_buttons_state[2] = KeyState::KeyDown,
								_ => (),
						}
				}
				else if state == ElementState::Released {
						match button {
								MouseButton::Right => self.mouse_buttons_state[0] = KeyState::KeyUp,
								MouseButton::Left => self.mouse_buttons_state[1] = KeyState::KeyUp,
								MouseButton::Middle => self.mouse_buttons_state[2] = KeyState::KeyUp,
								_ => (),
						}
				}
		}

		pub fn get_key(&self, code: KeyCode) -> KeyState {
				self.keys_state[self.to_glutin[&code] as usize]
		}

		pub fn get_mouse_button(&self, code: MouseButtonKeyCode) -> KeyState {
				self.mouse_buttons_state[code as usize]
		}

		pub fn get_actions(&self, action: ActionID) -> ActionInput {
				match action {
						ActionID::MOVE => {
								let mut dir = vec2(0.0, 0.0);
								if self.get_key(KeyCode::W) == KeyState::KeyDown {
										dir.y = 1.0;
								}
								if self.get_key(KeyCode::S) == KeyState::KeyDown {
										dir.y = -1.0;
								}
								if self.get_key(KeyCode::D) == KeyState::KeyDown {
										dir.x = 1.0;
								}
								if self.get_key(KeyCode::A) == KeyState::KeyDown {
										dir.x = -1.0;
								}

								if dir.amax() > 0.0 {
										dir = dir.normalize();
										return ActionInput::DIRECTIONAL(dir);
								}
								return ActionInput::NONE;

						},
						ActionID::LOOK => {
								let dir = vec2(self.mouse_delta.0, self.mouse_delta.1);

								return ActionInput::DIRECTIONAL(dir);
						},
						ActionID::ZOOM => {

								return ActionInput::ANALOGIC(self.mouse_wheel);
						},
						ActionID::ACTION1 =>
								if self.get_key(KeyCode::Space) == KeyState::KeyDown {
										return ActionInput::DIGITAL(true);
								} else {
										return ActionInput::DIGITAL(false);
								},
						_ => return ActionInput::DIGITAL(false),
				}
		}

}
