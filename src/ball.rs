use std::rand;
use std::rand::Rng;

use sdl2::;
use sdl2::render::Renderer;
use sdl2::video::Window;
use sdl2::rect::Rect;

use std::rc::Rc;

use vec2::Vec2;
use config::Config;
use paddle::Paddle;

pub struct Ball {
	position: ::vec2::Vec2,
	start_position: ::vec2::Vec2,
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
			start_position: ::vec2::Vec2::new(x, y),
			angle: 0_f32,
			speed: 0_f32,
			min_speed: 300_f32,
			max_speed: 500_f32,
			size: size,
			top_boundary: top,
			bottom_boundary: bottom,
		}
	}

	pub fn begin(&mut self) {
		let mut rng = rand::task_rng();
		let pi = Float::pi();
		let pi_over_4: f32 = Float::frac_pi_4();
		let angle_range = pi_over_4 * 3_f32;
		let half_angle_range = angle_range / 2_f32;
		
		self.position = self.start_position;

		self.angle = rng.gen::<f32>() * angle_range - half_angle_range;

		if rng.gen() {
			self.angle += pi;
		}

		self.wrap_angle();

		self.speed = self.min_speed;
	}

	fn wrap_angle(&mut self) {
		self.angle = ::util::wrap_radians(self.angle);
	}

	fn get_direction_vec(&self) -> Vec2 {
		Vec2 { x: self.angle.cos(), y: self.angle.sin() }
	}

	fn bounce_vertical(&mut self) {
		let two_pi: f32 = Float::two_pi();
		self.angle = two_pi - self.angle;

		self.wrap_angle();
	}

	fn bounce_horizontal(&mut self, angle_scale: f32, dir: i32) {
		let pi: f32 = Float::pi();
		let max_angle: f32 = 75_f32 * pi / 180_f32;
		let angle = (pi * 2_f32) - (max_angle * angle_scale);

		self.angle = angle;

		if dir == 1 {
			self.angle += pi;
			self.angle = (pi * 2_f32) - self.angle;
		}

		self.wrap_angle();
	}

	fn check_sides(&mut self) {
		if self.position.y < self.top_boundary ||
		   self.position.y > self.bottom_boundary {
			self.position.y = ::util::clampf(self.position.y,
											 self.top_boundary,
											 self.bottom_boundary);
			self.bounce_vertical();
		}
	}

	fn check_goal(&mut self) {
		if self.position.x < -self.size * 2_f32 ||
		   self.position.x > 800_f32 + self.size * 2_f32 {
		   	self.begin();
		}
	}

	// if true paddle is hit and dist is set to normalized relative distance
	// from center of paddle.
	fn check_paddle(&self, paddle: &Paddle) -> (bool, f32) {
		let pw = paddle.width / 2_f32;
		let ph = paddle.height / 2_f32;
		let s = self.size;

		let no_hit = (false, 0_f32);

		// to the left
		if self.position.x + s < paddle.position.x - pw { return no_hit; }

		// to the right
		if self.position.x - s > paddle.position.x + pw { return no_hit; }

		// above
		if self.position.y + s < paddle.position.y - ph { return no_hit; }

		// below
		if self.position.y - s > paddle.position.y + ph { return no_hit; }

		let dist = (paddle.position.y - self.position.y) / (ph + s);

		return (true, dist);
	}

	fn check_paddles(&mut self, left: &Paddle, right: &Paddle) {
		let (left_hit, ld) = self.check_paddle(left);
		let (right_hit, rd) = self.check_paddle(right);
		
		let dir = Vec2::from_angle(self.angle).x.signum() as i32;

		if left_hit && dir == -1 {
			self.bounce_horizontal(ld, dir);
			self.speed += 250_f32;
		}

		if right_hit && dir == 1 {
			self.bounce_horizontal(rd, dir);
			self.speed += 250_f32;
		}
	}

	pub fn update(&mut self, dt: f32, left: &Paddle, right: &Paddle) {
		let vel = ::vec2::Vec2 {
			x: self.angle.cos() * self.speed * dt,
			y: self.angle.sin() * self.speed * dt,
		};
		self.position = self.position + vel;

		self.check_sides();
		self.check_goal();
		self.check_paddles(left, right);

		self.speed -= 50_f32 * dt;
		self.speed = ::util::clampf(self.speed, self.min_speed, self.max_speed);
	}

	pub fn render(&self, renderer: &Renderer<Window>) {
		let w = (self.size * 2_f32) as i32;
		let h = (self.size * 2_f32) as i32;
		let x = (self.position.x as i32) - (w / 2);
		let y = (self.position.y as i32) - (h / 2);

		let rect: Rect = Rect::new(x, y, w, h);

		let _ = renderer.set_draw_color(sdl2::pixels::RGB(255, 0, 0));
		let _ = renderer.fill_rect(&rect);
	}
}