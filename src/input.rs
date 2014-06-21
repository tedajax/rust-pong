use std::collections::HashMap;
use sdl2::scancode::ScanCode;

pub struct Input {
	old_state: HashMap<ScanCode, bool>,
	new_state: HashMap<ScanCode, bool>,
}

impl Input {
	pub fn new() -> Input {
		Input {
			old_state: HashMap::new(),
			new_state: HashMap::new(),
		}
	}

	pub fn update(&mut self) {
		for (k, v) in self.new_state.iter() {
			self.old_state.insert(*k, *v);
		}
		self.new_state = ::sdl2::keyboard::get_keyboard_state();
	}

	fn old_key_state(&self, scancode: ScanCode) -> bool {
		match self.old_state.contains_key(&scancode) {
			false => false,
			true => *self.old_state.get(&scancode),
		}
	}

	fn new_key_state(&self, scancode: ScanCode) -> bool {
		match self.new_state.contains_key(&scancode) {
			false => false,
			true => *self.new_state.get(&scancode),
		}
	}

	#[allow(dead_code)]
	pub fn get_key(&self, scancode: ScanCode) -> bool {
		self.new_key_state(scancode)
	}

	#[allow(dead_code)]
	pub fn get_key_down(&self, scancode: ScanCode) -> bool {
		!self.old_key_state(scancode) && self.new_key_state(scancode)
	}

	#[allow(dead_code)]
	pub fn get_key_up(&self, scancode: ScanCode) -> bool {
		self.old_key_state(scancode) && !self.new_key_state(scancode)
	}
}