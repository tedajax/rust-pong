use sdl2::;
use sdl2::render::Renderer;
use sdl2::video::Window;
use sdl2::rect::Rect;

use std::rand::Rng;
use std::rand;

use paddle::Paddle;
use ball::Ball;
use input::Input;
use config::Config;

pub struct Game {
	renderer: Renderer<Window>,
	left_paddle: Paddle,
	right_paddle: Paddle,
	ball: Ball,
	input: Input,
	last_tick: uint,
	config: Config,
}

impl Game {
	pub fn new(config: Config) -> Game {
		sdl2::init(sdl2::InitVideo);

		let window_flags = sdl2::video::Shown;

		let window = match Window::new("Rust Pong",
									   sdl2::video::PosCentered,
									   sdl2::video::PosCentered,
									   config.screen_width as int,
									   config.screen_height as int,
									   window_flags) {
			Ok(window) => window,
			Err(why) => fail!(format!("failed to create window: {}", why))
		};

		let renderer = match Renderer::from_window(window,
											  	   sdl2::render::DriverAuto,
												   sdl2::render::Accelerated) {
			Ok(renderer) => renderer,
			Err(why) => fail!(format!("failed to create renderer: {}", why))
		};

		let paddle_offset = config.paddle_side_boundary +
							config.paddle_width / 2f32;
		let left_paddle_pos = paddle_offset;
		let right_paddle_pos = (config.screen_width as f32) - paddle_offset;

		Game { 
			renderer: renderer,
			left_paddle: Paddle::new(left_paddle_pos, 300.0, config),
			right_paddle: Paddle::new(right_paddle_pos, 300.0, config),
			ball: Ball::new(config),
			input: Input::new(),
			last_tick: 0,
			config: config,
		}	
	}

	pub fn run(&mut self) {
		self.ball.begin();

		'game : loop {
			'event : loop {
				match sdl2::event::poll_event() {
					sdl2::event::QuitEvent(_) => break 'game,
					sdl2::event::KeyDownEvent(_, _, key, _, _) => {
						if key == sdl2::keycode::EscapeKey {
							break 'game
						}
					}
					_ => break 'event
				}
			}

			let new_tick = sdl2::timer::get_ticks();
			let tick_delta = new_tick - self.last_tick;
			let dt: f32 = (tick_delta as f32) / 1000.0f32;
			self.last_tick = new_tick;

			self.update(dt);
			self.render();
		}		
	}

	fn update(&mut self, dt: f32) {
		let paddle_speed = 500.0f32 * dt;

		if self.input.get_key(sdl2::scancode::WScanCode) {
			self.left_paddle.move(-paddle_speed);
		}
		if self.input.get_key(sdl2::scancode::SScanCode) {
			self.left_paddle.move(paddle_speed);
		}

		if self.input.get_key(sdl2::scancode::UpScanCode) {
			self.right_paddle.move(-paddle_speed);
		}
		if self.input.get_key(sdl2::scancode::DownScanCode) {
			self.right_paddle.move(paddle_speed);
		}

		self.ball.update(dt);

		self.input.update();
	}

	fn render(&self) {
		let _ = self.renderer.set_draw_color(sdl2::pixels::RGB(0, 0, 0));
		let _ = self.renderer.clear();

		self.render_boundaries();

		self.left_paddle.render(&self.renderer);
		self.right_paddle.render(&self.renderer);
		self.ball.render(&self.renderer);

		self.renderer.present();
	}

	fn render_boundaries(&self) {
		let config: Config = self.config;

		let top_rect = Rect{
			x: 0,
			y: 0,
			w: config.screen_width as i32,
			h: config.top_boundary as i32
		};

		let bottom_rect = Rect {
			x: 0,
			y: (config.screen_height - config.bottom_boundary) as i32,
			w: config.screen_width as i32,
			h: config.bottom_boundary as i32
		};

		let _ = self.renderer.set_draw_color(sdl2::pixels::RGB(255, 255, 255));
		let _ = self.renderer.fill_rect(&top_rect);
		let _ = self.renderer.fill_rect(&bottom_rect);
	}
}
