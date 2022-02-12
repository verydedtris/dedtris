extern crate bitvec;
extern crate log;
extern crate rlua;
extern crate sdl2;

use log::{error, info};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::path::Path;
use std::time::Duration;

use crate::error::Error;
use crate::game::{draw, handle_event, TetrisState};

mod error;
// mod file;
mod game;
mod lua;

fn main()
{
	// Init Logger

	env_logger::init();

	// Init SDL2 and its window system

	info!("Initializing SDL2 and its subsystems.");

	let sdl_context = end!(sdl2::init(), "Couldn't initialize SDL2");

	let video_subsystem = end!(
		sdl_context.video(),
		"Couldn't initialize SDL2 videosubsystem"
	);

	info!("Constructing window.");

	let window = end!(
		video_subsystem
			.window("Tetris", 800, 600)
			.position_centered()
			//.resizable() // Simpler to debug
			.build(),
		"Couldn't create window"
	);

	info!("Constructing renderer for window.");

	let mut canvas = end!(
		window.into_canvas().accelerated().target_texture().build(),
		"Couldn't construct renderer"
	);

	let texture_creator = canvas.texture_creator();

	canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
	canvas.set_draw_color(Color::RGB(0, 255, 255));
	canvas.clear();
	canvas.present();

	// Init Lua runtime

	let path = Path::new("Themes/test.lua");

	info!(
		"Initializing Lua plugin enviroment and loading {}.",
		path.display()
	);

	let lua = rlua::Lua::new();

	if let Err(_) = lua.context::<_, Result<(), Error>>(|ctx| {
		// Load theme file

		game::load_defaults(&ctx)?;
		lua::exec_file(&ctx, path)?;

		// Init Game

		info!("Initializing tetris game.");

		let mut game = TetrisState::init(&texture_creator, canvas.output_size().unwrap(), &ctx)?;

		// Event Loop

		info!("Beginning Game.");

		let mut event_pump = sdl_context.event_pump().unwrap();
		let mut i = 0;

		'running: loop {
			i = (i + 1) % 255;

			canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
			canvas.clear();

			for event in event_pump.poll_iter() {
				match event {
					Event::Quit { .. }
					| Event::KeyDown {
						keycode: Some(Keycode::Escape),
						..
					} => break 'running,
					_ => {
						if !handle_event(&event, &mut canvas, &mut game)? {
							break 'running;
						}
					}
				}
			}

			draw(&game, &mut canvas);

			canvas.present();
			::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}

		game.output_score();

		Ok(())
	}) {
		error!("Unrecoverable error.");
	}
}
