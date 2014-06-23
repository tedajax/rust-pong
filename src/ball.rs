use std::rand;
use std::rand::Rng;

use sdl2::;
use sdl2::render::Renderer;
use sdl2::video::Window;
use sdl2::rect::Rect;

use config::Config;

pub struct Ball {
	position: ::vec2::Vec2,
	angle: f32,
	speed: f32,
	min_speed: f32,
	max_speed: f32,
	size: f32,
	top_boundary: f32,
	bottom_boundary: f32,
}

impl Ball {
	pub fn new(config: Config) -> Ball {
		let x = config.screen_width / 2_f32;
		let y = config.screen_height / 2_f32;
		let size = 10_f32;

		let top = config.top_boundary + size;
		let bottom = config.screen_height - config.bottom_boundary - size;

		Ball {
			position: ::vec2::Vec2::new(x, y),
			angle: 0_f32,
			speed: 0_f32,
			min_speed: 200_f32,
			max_speed: 2000_f32,
			size: size,
			top_boundary: top,
			bottom_boundary: bottom,
		}
	}

	pub fn begin(&mut self) {
		let mut rng = rand::task_rng();
		
		self.angle = rng.gen::<f32>() * Float::two_pi();
		self.speed = self.min_speed;
	}

	fn check_sides(&mut self) {
		let pi: f32 = Float::pi();
		let two_pi: f32 = Float::two_pi();

		if self.position.y < self.top_boundary {
			self.position.y = self.top_boundary;
			self.angle = two_pi - self.angle;
		}

		if self.position.y > self.bottom_boundary {
			self.position.y = self.bottom_boundary;
			self.angle = two_pi - self.angle;
		}

		if self.position.x < self.size {
			self.position.x = self.size;
			self.angle = pi - self.angle;
		}

		if self.position.x > 800_f32 - self.size {
			self.position.x = 800_f32 - self.size;
			self.angle = pi - self.angle;
		}
	}

	fn check_goal(&mut self) {

	}

	pub fn update(&mut self, dt: f32) {
		self.speed += 20_f32 * dt;

		let vel = ::vec2::Vec2 {
			x: self.angle.cos() * self.speed * dt,
			y: self.angle.sin() * self.speed * dt,
		};
		self.position = self.position + vel;

		self.check_sides();
	}

	pub fn render(&self, renderer: &Renderer<Window>) {
		let w = (self.size * 2f32) as i32;
		let h = (self.size * 2f32) as i32;
		let x = (self.position.x as i32) - (w / 2);
		let y = (self.position.y as i32) - (h / 2);

		let rect: Rect = Rect::new(x, y, w, h);

		let _ = renderer.set_draw_color(sdl2::pixels::RGB(255, 0, 0));
		let _ = renderer.fill_rect(&rect);
	}
}