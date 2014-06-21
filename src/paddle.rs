use sdl2::;
use sdl2::render::Renderer;
use sdl2::video::Window;
use sdl2::rect::Rect;

static paddle_color: sdl2::pixels::Color = sdl2::pixels::RGB(255, 255, 255);

pub struct Paddle {
	position: ::vec2::Vec2,
	width: f32,
	height: f32,
}

impl Paddle {
	pub fn new(x: f32, y: f32) -> Paddle {
		Paddle { 
			position: ::vec2::Vec2::new(x, y),
			width: 20.0f32,
			height: 200.0f32,
		}
	}

	pub fn move(&mut self, vel_y: f32) {
		self.position.y += vel_y;

		if self.position.y < self.height / 2f32 {
			self.position.y = self.height / 2f32;
		}

		if self.position.y > 600f32 - (self.height / 2f32) {
			self.position.y = 600f32 - (self.height / 2f32);
		}
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