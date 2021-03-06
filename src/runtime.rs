use std::{path::Path, time::Duration};

use log::info;
use sdl2::{
	event::{Event, WindowEvent},
	keyboard::Keycode,
	pixels::Color,
	rect::{Point, Rect},
	render::{TextureCreator, WindowCanvas},
	video::WindowContext,
	Sdl, VideoSubsystem,
};

use self::{
	game::Game,
	state::{Direction, TetrisState},
};
use crate::{error::Error, lua};

mod drawer;
mod game;
mod profile_api;
mod state;

#[derive(Debug)]
pub struct Piece
{
	pub dim:    u32,
	pub colors: Vec<Color>,
	pub blocks: Vec<Point>,
}

type Size = (u32, u32);

pub struct Framework<'a, 'b, 'd, 'e, 'f, 'g>
{
	pub sdl:       &'a Sdl,
	pub video:     &'b VideoSubsystem,
	pub canvas:    &'d mut WindowCanvas,
	pub tex_maker: &'e TextureCreator<WindowContext>,
	pub lua:       &'f rlua::Context<'g>,
}

// -----------------------------------------------------------------------------
// Game Runtime
// -----------------------------------------------------------------------------

/// Starts a window and plays the tetris game
///
/// # Arguments
///
/// * `sdl_context` SDL context
/// * `video_sys` Video subsystem
pub fn start_tetris_game(profile: &Path) -> Result<(), Error>
{
	info!("Initializing SDL2 and its subsystems.");

	let sdl_context = sdl2::init()?;
	let video_sys = sdl_context.video()?;

	info!("Initializing Lua plugin enviroment.",);

	let lua = rlua::Lua::new();

	lua.context::<_, Result<(), Error>>(|ctx| {
		let t = {
			profile_api::load_defaults(&ctx)?;
			lua::exec_file(&ctx, profile)?;

			profile_api::load(&ctx)?
		};

		info!("Constructing window.");

		const WINDOW_SIZE: (u32, u32) = (1080, 720);

		let window = video_sys
			.window("Tetris", WINDOW_SIZE.0, WINDOW_SIZE.1)
			.position_centered()
			.resizable()
			.build()?;

		info!("Initializing renderer.");

		let mut canvas = {
			let mut canvas = window.into_canvas().accelerated().target_texture().build()?;

			canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
			canvas.set_draw_color(Color::RGB(0, 255, 255));
			canvas.clear();
			canvas.present();

			canvas
		};

		let tex_maker = canvas.texture_creator();

		info!("Initializing tetris game.");

		let mut game = {
			let fw = Framework {
				sdl:       &sdl_context,
				video:     &video_sys,
				canvas:    &mut canvas,
				tex_maker: &tex_maker,
				lua:       &ctx,
			};

			game::init_game(fw, WINDOW_SIZE, t)?
		};

		// Event Loop

		info!("Beginning Game.");

		let sdl = game.fw.sdl;
		let mut event_pump = sdl.event_pump().unwrap();

		'running: loop
		{
			let canvas = &mut game.fw.canvas;
			canvas.set_draw_color(Color::GRAY);
			canvas.clear();

			for event in event_pump.poll_iter()
			{
				match event
				{
					Event::Quit { .. }
					| Event::KeyDown {
						keycode: Some(Keycode::Escape),
						..
					} => break 'running,
					_ =>
					{
						if !handle_event(&event, &mut game)?
						{
							break 'running;
						}
					},
				}
			}

			update(&mut game)?;
			draw(&mut game);

			let canvas = &mut game.fw.canvas;
			canvas.present();

			::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
		}

		let state = &game.state;
		state.output_score();

		Ok(())
	})
}

pub fn handle_event<'a>(event: &Event, game: &mut Game) -> Result<bool, Error>
{
	match event
	{
		Event::KeyDown {
			keycode: Some(x), ..
		} => match x
		{
			Keycode::Left =>
			{
				let state = &mut game.state;
				state.move_piece(Direction::LEFT);
			},

			Keycode::Right =>
			{
				let state = &mut game.state;
				state.move_piece(Direction::RIGHT);
			},

			Keycode::Down =>
			{
				return Ok(game.move_piece_down()?);
			},

			Keycode::Up =>
			{
				let state = &mut game.state;
				state.rotate();
			},

			Keycode::LShift =>
			{
				game.swap()?;
			},

			Keycode::Space =>
			{
				return Ok(game.drop()?);
			},

			_ => (),
		},

		Event::Window {
			win_event: WindowEvent::Resized(w, h),
			..
		} =>
		{
			let drawer = &mut game.rend;
			drawer.win_dim = (*w as u32, *h as u32);
		},

		_ => (),
	}

	let state = &mut game.state;
	Ok(!state.exit)
}

pub fn update(game: &mut Game) -> Result<(), Error>
{
	game.tick_update()
}

pub fn draw(game: &mut Game)
{
	let state = &game.state;
	let rend = &mut game.rend;
	let fw = &mut game.fw;

	let fd = state.field_size;
	let wd = rend.win_dim;

	let drawer::size::ResizePattern {
		threshold: _,
		block_size,
		field_rect,
	} = drawer::size::new_resize(wd, fd);

	let canvas = &mut fw.canvas;
	let fc = rend.field_bg_color;

	// Draw field
	{
		canvas.set_draw_color(fc);
		canvas.fill_rect(field_rect).unwrap();
	}

	// Draw field blocks
	{
		let btex = &mut rend.block_texture;
		let fbs = &state.field_blocks;
		let fcs = &state.field_colors;

		for (c, b) in fcs.iter().zip(fbs)
		{
			let r = Rect::new(
				field_rect.x + b.x * block_size as i32,
				field_rect.y + b.y * block_size as i32,
				block_size,
				block_size,
			);

			btex.set_color_mod(c.r, c.g, c.b);
			canvas.copy(&btex, None, r).unwrap();
		}
	}

	// Draw player
	{
		let p = &state.player_piece;
		let pl = state.player_pos;
		let proj = state.player_proj;

		let offset_x = field_rect.x + pl.x * block_size as i32;
		let offset_y = field_rect.y + pl.y * block_size as i32;
		rend.draw_blocks(
			canvas,
			Point::new(offset_x, offset_y),
			block_size,
			&p.blocks,
			&p.colors,
		);

		let btex = &mut rend.block_texture;
		btex.set_alpha_mod(127);

		let offset_y = field_rect.y + proj * block_size as i32;
		rend.draw_blocks(
			canvas,
			Point::new(offset_x, offset_y),
			block_size,
			&p.blocks,
			&p.colors,
		);

		let btex = &mut rend.block_texture;
		btex.set_alpha_mod(255);
	}

	// Draw piece view
	{
		let pvs = &state.piece_queue;
		let idx = state.piece_queue_idx;

		let x = field_rect.x + field_rect.w + 10;
		let mut y = field_rect.y;

		for p in pvs[idx..].iter().chain(&pvs[..idx])
		{
			rend.draw_blocks(canvas, Point::new(x, y), block_size, &p.blocks, &p.colors);
			y += (p.dim * block_size + 10) as i32;
		}
	}

	// Draw piece swap
	if let Some(sp) = &state.piece_swap
	{
		let size = block_size * sp.dim;
		let pos = Point::new(field_rect.x - size as i32 - 10, field_rect.y);

		rend.draw_blocks(canvas, pos, block_size, &sp.blocks, &sp.colors);
	}
}
