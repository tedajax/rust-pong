extern crate sdl2;
extern crate native;

extern crate core;

mod game;
mod paddle;
mod ball;
mod vec2;
mod input;
mod config;
mod util;

static game_config: config::Config = config::Config {
	screen_width: 800.0f32,
	screen_height: 600.0f32,
	paddle_width: 20.0f32,
	paddle_height: 150.0f32,
	paddle_side_boundary: 5f32,
	top_boundary: 20f32,
	bottom_boundary: 20f32,
};

fn main() {
	let mut game = game::Game::new(game_config);
	game.run();
}