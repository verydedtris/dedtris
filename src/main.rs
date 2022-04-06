extern crate ini;
extern crate log;
extern crate rlua;
extern crate sdl2;

mod error;
// mod file;
mod game;
mod lua;
mod menu;

fn main() -> Result<(), error::Error>
{
	// Init Logger
	env_logger::init();

	menu::print_banner();

	while !menu::start_menu() {}

	Ok(())
}
