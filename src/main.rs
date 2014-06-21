extern crate sdl2;
extern crate native;

mod game;
mod paddle;
mod vec2;
mod input;

fn main() {
	let mut game = game::Game::new(800, 600);
	game.run();
}