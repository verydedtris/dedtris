extern crate bitvec;
extern crate log;
extern crate rlua;
extern crate sdl2;

use log::info;

mod error;
// mod file;
mod game;
mod lua;

fn main() -> Result<(), error::Error>
{
	// Init Logger

	env_logger::init();

	// Init SDL2 and its window system

	info!("Initializing SDL2 and its subsystems.");

	let sdl_context = sdl2::init()?;
	let video_subsystem = sdl_context.video()?;

	game::start_tetris_game(&sdl_context, &video_subsystem)
}
