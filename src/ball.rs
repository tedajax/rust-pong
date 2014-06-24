use std::rand;
use std::rand::Rng;

use sdl2::;
use sdl2::render::Renderer;
use sdl2::video::Window;
use sdl2::rect::Rect;

use vec2::Vec2;
use config::Config;
use paddle::Paddle;

static PADDLE_RESPONSES: [f32, ..3] = [
	-1_f32,
	0_f32,
	1_f32
];

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
	left_paddle: *Paddle,
	right_paddle: *Paddle,
}

impl Ball {
	pub fn new(config: Config, left: &Paddle, right: &Paddle) -> Ball {
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
			min_speed: 200_f32,
			max_speed: 500_f32,
			size: size,
			top_boundary: top,
			bottom_boundary: bottom,
			left_paddle: left,
			right_paddle: right,
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

		self.angle = 0_f32;

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

	fn bounce_horizontal(&mut self) {
		let pi: f32 = Float::pi();
		self.angle = pi - self.angle;

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

	// returns paddle section hit or -1 if paddle not hit
	fn check_paddle(&self, paddle: &Paddle) -> i32 {
		let pw = paddle.width / 2_f32;
		let ph = paddle.height / 2_f32;

		// to the left
		if self.position.x + self.size < paddle.position.x - pw { return -1; }

		// to the right
		if self.position.x - self.size > paddle.position.x + pw { return -1; }

		// above
		if self.position.y + self.size < paddle.position.y - ph { return -1; }

		// below
		if self.position.y - self.size > paddle.position.y + ph { return -1; }

		let segments = PADDLE_RESPONSES.len() as f32;
		let intersect_height: f32 = self.size * 2_f32 + paddle.height;
		let segment_height: f32 = intersect_height / segments;

		for i in range(0, PADDLE_RESPONSES.len()) {
			println!("{}", i)
		}

		return -1;
	}

	fn check_paddles(&mut self) {
		unsafe {
			self.check_paddle(&*self.left_paddle);
		}
	}

	pub fn update(&mut self, dt: f32) {
		let pi: f32 = Float::pi();
		self.angle -= pi / 4_f32 * dt;

		let vel = ::vec2::Vec2 {
			x: self.angle.cos() * self.speed * dt,
			y: self.angle.sin() * self.speed * dt,
		};
		self.position = self.position + vel;

		self.check_sides();
		self.check_goal();
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