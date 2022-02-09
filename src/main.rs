extern crate bitvec;
extern crate log;
extern crate rlua;
extern crate sdl2;

use log::{error, info};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use std::{borrow::BorrowMut, path::Path};

use crate::game::{draw, handle_event, TetrisState};

mod error;
mod file;
mod game;
mod lua;

macro_rules! init {
	($system:expr, $msg:expr) => {
		match $system {
			Ok(v) => v,
			Err(e) => {
				error!($msg, e);
				return;
			}
		}
	};

	($system:expr) => {
		match $system {
			Ok(v) => v,
			Err(_) => {
				return;
			}
		}
	};
}

fn main()
{
	// Init Logger

	env_logger::init();

	// Init SDL2 and its window system

	info!("Initializing SDL2 and its subsystems.");
	let sdl_context = init!(sdl2::init(), "Couldn't initialize SDL2: {}");
	let video_subsystem = init!(
		sdl_context.video(),
		"Couldn't initialize SDL2 videosubsystem: {}"
	);

	info!("Constructing window.");
	let window = init!(
		video_subsystem
			.window("Tetris", 800, 600)
			.position_centered()
			//.resizable() // Simpler to debug
			.build(),
		"Couldn't create window: {}"
	);

	info!("Constructing renderer for window.");
	let mut canvas = init!(
		window.into_canvas().accelerated().target_texture().build(),
		"Couldn't construct renderer: {}"
	);
	let texture_creator = canvas.texture_creator();

	canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
	canvas.set_draw_color(Color::RGB(0, 255, 255));
	canvas.clear();
	canvas.present();

	// Init Lua runtime

	let path = Path::new("test.lua");

	info!(
		"Initializing Lua plugin enviroment and loading {}.",
		path.display()
	);

	let lua = rlua::Lua::new();

	lua.context(|ctx| {
		// Load theme file

        game::load_default(&ctx);
		lua::exec_file(&ctx, path);

		// Init Game

		info!("Initializing tetris game.");
		let mut game = init!(TetrisState::init(
			&texture_creator,
			canvas.output_size().unwrap(),
			&ctx
		));

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
						handle_event(&event, &mut canvas, &mut game);
					}
				}
			}

			draw(&game, &mut canvas);

			canvas.present();
			::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}
	});
}
