extern crate bitvec;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::path::Path;
use std::time::Duration;

mod file;
mod game;

fn main()
{
    // Load resources

	let theme = match file::Theme::load(Path::new("test.xml")) {
        Ok(o) => o,
        Err(e) => {
            println!("Couldn't load theme: {}", e);
            return;
        }
    };

    // Init SDL2 and its window system

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem
		.window("Tetris", 800, 600)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().build().unwrap();

	canvas.set_draw_color(Color::RGB(0, 255, 255));
	canvas.clear();
	canvas.present();

    // Init Game

	let mut game = game::Data::init(canvas.output_size().unwrap(), theme);

    // Event Loop

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
					game.handle_event(&event);
				}
			}
		}

		game.draw(&mut canvas);

		canvas.present();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
