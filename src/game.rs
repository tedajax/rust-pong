use sdl2::;
use sdl2::render::Renderer;
use sdl2::video::Window;
use paddle::Paddle;
use input::Input;

pub struct Game {
	renderer: Renderer<Window>,
	left_paddle: Paddle,
	right_paddle: Paddle,
	input: Input,
	last_tick: uint,
}

impl Game {
	pub fn new(width: int, height: int) -> Game {
		sdl2::init(sdl2::InitVideo);

		let window = match Window::new("Rust Pong",
									   sdl2::video::PosCentered,
									   sdl2::video::PosCentered,
									   width,
									   height,
									   sdl2::video::OpenGL) {
			Ok(window) => window,
			Err(why) => fail!(format!("failed to create window: {}", why))
		};

		let renderer = match Renderer::from_window(window,
											  	   sdl2::render::DriverAuto,
												   sdl2::render::Accelerated) {
			Ok(renderer) => renderer,
			Err(why) => fail!(format!("failed to create renderer: {}", why))
		};

		Game { 
			renderer: renderer,
			left_paddle: Paddle::new(10.0, 300.0),
			right_paddle: Paddle::new(790.0, 300.0),
			input: Input::new(),
			last_tick: 0,
		}	
	}

	pub fn run(&mut self) {
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

		self.input.update();
	}

	fn render(&self) {
		let _ = self.renderer.set_draw_color(sdl2::pixels::RGB(0, 0, 0));
		let _ = self.renderer.clear();

		self.left_paddle.render(&self.renderer);
		self.right_paddle.render(&self.renderer);

		self.renderer.present();
	}
}
