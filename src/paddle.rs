use sdl2::;
use sdl2::render::Renderer;
use sdl2::video::Window;
use sdl2::rect::Rect;
use config::Config;

static paddle_color: sdl2::pixels::Color = sdl2::pixels::RGB(255, 255, 255);

pub struct Paddle {
	pub position: ::vec2::Vec2,
	pub width: f32,
	pub height: f32,
	top_boundary: f32,
	bottom_boundary: f32,
}

impl Paddle {
	pub fn new(x: f32, y: f32, config: Config) -> Paddle {
		let half_height = config.paddle_height / 2f32;
		let top = half_height + config.top_boundary;
		let bottom = config.screen_height -
					 half_height -
					 config.bottom_boundary;

		Paddle { 
			position: ::vec2::Vec2::new(x, y),
			width: config.paddle_width,
			height: config.paddle_height,
			top_boundary: top,
			bottom_boundary: bottom,
		}
	}

	pub fn move(&mut self, vel_y: f32) {
		self.position.y += vel_y;

		if self.position.y < self.top_boundary {
			self.position.y = self.top_boundary;
		}

		if self.position.y > self.bottom_boundary {
			self.position.y = self.bottom_boundary;
		}

		println!("{}", self.position.y);
	}

	pub fn render(&self, renderer: &Renderer<Window>) {
		let w = self.width as i32;
		let h = self.height as i32;
		let x = (self.position.x as i32) - (w / 2);
		let y = (self.position.y as i32) - (h / 2);

		let rect: Rect = Rect::new(x, y, w, h);

		let _ = renderer.set_draw_color(paddle_color);
		let _ = renderer.fill_rect(&rect);
	}
}