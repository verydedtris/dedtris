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
	drawer::Renderer,
	state::{Direction, TetrisState},
};
use crate::{error::Error, lua};

mod drawer;
mod state;
mod theme;
mod theme_api;

type Size = (u32, u32);

pub struct Framework<'a, 'b, 'd, 'e, 'f, 'g>
{
	sdl:       &'a Sdl,
	video:     &'b VideoSubsystem,
	canvas:    &'d mut WindowCanvas,
	tex_maker: &'e TextureCreator<WindowContext>,
	lua:       &'f rlua::Context<'g>,
}

// -----------------------------------------------------------------------------
// Game Runtime
// -----------------------------------------------------------------------------

/// Starts a window and plays the tetris game
///
/// # Arguments
///
/// * `sdl_context` - SDL context
/// * `video_sys` - Video subsystem
pub fn start_tetris_game(sdl_context: &Sdl, video_sys: &VideoSubsystem) -> Result<(), Error>
{
	info!("Initializing Lua plugin enviroment.",);

	let lua = rlua::Lua::new();

	lua.context::<_, Result<(), Error>>(|ctx| {
		// Load theme file

		theme_api::load_defaults(&ctx)?;
		lua::exec_file(&ctx, Path::new("Themes/default.lua"))?;
		lua::exec_file(&ctx, Path::new("Themes/test.lua"))?;

		let t = theme::load(&ctx)?;

		// Construct window

		info!("Constructing window.");

        let size = drawer::calc_window_size(t.field_dim);

		let window = video_sys
			.window("Tetris", size.w, size.h)
			.position_centered()
			//.resizable() // Simpler to debug
			.build()?;

		let mut canvas = window.into_canvas().accelerated().target_texture().build()?;

		let tex_maker = canvas.texture_creator();

		canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
		canvas.set_draw_color(Color::RGB(0, 255, 255));
		canvas.clear();
		canvas.present();

		// Create framework

		let mut fw = Framework {
			sdl:       sdl_context,
			video:     video_sys,
			canvas:    &mut canvas,
			tex_maker: &tex_maker,
			lua:       &ctx,
		};

		// Init Game

		info!("Initializing tetris game.");

		let mut game = state::init_game(&t)?;
		let mut renderer = drawer::init_renderer(&fw, &t)?;

		if !spawn_piece(&mut game, &fw)? {
			return Err(Error::from("No area for piece."));
		}

		// Event Loop

		info!("Beginning Game.");

		let sdl = fw.sdl;

		let mut event_pump = sdl.event_pump().unwrap();
		let mut i = 0;

		'running: loop {
			i = (i + 1) % 255;

			let canvas = &mut fw.canvas;

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
						if !handle_event(&event, &mut fw, &mut renderer, &mut game)? {
							break 'running;
						}
					},
				}
			}

			draw(&game, &renderer, &mut fw);

			let canvas = &mut fw.canvas;

			canvas.present();
			::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}

		output_score(&game);

		Ok(())
	})
}

fn regen_blocks(fw: &mut Framework, state: &TetrisState, drawer: &mut Renderer<'_>)
{
	let ft = &mut drawer.pieces_texture;
	let bs = drawer.block_size;
	let fb = &state.field_blocks;
	let fc = &state.field_colors;

	let canvas = &mut fw.canvas;

	canvas
		.with_texture_canvas(ft, |c| {
			c.set_draw_color(Color::RGBA(0, 0, 0, 0));
			c.clear();

			drawer::draw_blocks(c, bs, &fb, &fc);
		})
		.unwrap();
}

pub fn spawn_piece(state: &mut TetrisState, fw: &Framework) -> Result<bool, Error>
{
	info!("Respawning piece.");

	let t = theme_api::call_lua("spawn_piece", state, fw)?;
	let (dim, colors, blocks) = theme::parse_pattern(t)?;

	Ok(state::spawn_piece(state, blocks, colors, dim))
}

pub fn place_piece(
	state: &mut TetrisState, drawer: &mut Renderer<'_>, fw: &mut Framework,
) -> Result<(), Error>
{
	info!("Placing piece.");

	let fc = &mut state.field_colors;
	let fb = &mut state.field_blocks;
	let pb = &state.piece_blocks;
	let pc = &state.piece_colors;
	let pl = state.piece_loc;

	fb.extend(pb.iter().map(|b| Point::new(b.x + pl.x, b.y + pl.y)));
	fc.extend(pc);

	theme_api::call_lua::<()>("on_place", state, fw)?;

	regen_blocks(fw, state, drawer);

	let pp = &mut state.pieces_placed;
	*pp += 1;

	Ok(())
}

pub fn drop(
	state: &mut TetrisState, drawer: &mut Renderer<'_>, fw: &mut Framework,
) -> Result<bool, Error>
{
	info!("Dropping piece.");

	let pj = state.piece_proj;

	state.piece_loc.y = pj;

	place_piece(state, drawer, fw)?;
	spawn_piece(state, fw)
}

fn move_piece_down(
	state: &mut TetrisState, drawer: &mut Renderer<'_>, fw: &mut Framework,
) -> Result<bool, Error>
{
	if state::move_piece(state, Direction::DOWN) {
		return Ok(true);
	}

	place_piece(state, drawer, fw)?;
	spawn_piece(state, fw)
}

pub fn output_score(state: &TetrisState)
{
	println!(
		"Well done! Here are your stats.\nScore: {}\nTime: {}\nLines cleared: {}\nPieces placed: \
		 {}",
		state.lines_cleared as f64
			/ (state.time.elapsed().as_secs_f64() * state.pieces_placed as f64),
		state.time.elapsed().as_secs_f64(),
		state.lines_cleared,
		state.pieces_placed,
	);
}

// -----------------------------------------------------------------------------
// Game
// -----------------------------------------------------------------------------

pub fn handle_event(
	event: &Event, fw: &mut Framework, drawer: &mut Renderer<'_>, state: &mut TetrisState,
) -> Result<bool, Error>
{
	match event {
		Event::KeyDown {
			keycode: Some(x), ..
		} => match x {
			Keycode::Left => {
				state::move_piece(state, Direction::LEFT);
			},

			Keycode::Right => {
				state::move_piece(state, Direction::RIGHT);
			},

			Keycode::Down => {
				return Ok(move_piece_down(state, drawer, fw)?);
			},

			Keycode::Up => {
				state::rotate(state);
			},

			Keycode::Space => {
				return Ok(drop(state, drawer, fw)?);
			},

			_ => (),
		},

		Event::Window {
			win_event: WindowEvent::Resized(w, h),
			..
		} => {
			let fd = state.field_size;
			drawer::resize_game(drawer, (*w as u32, *h as u32), fd);
		},

		_ => (),
	}

	Ok(!state.exit)
}

pub fn draw(state: &TetrisState, drawer: &Renderer<'_>, fw: &mut Framework)
{
	draw_field(fw, drawer);
	draw_blocks(drawer, fw);
	draw_player(state, drawer, fw);
}

pub fn draw_blocks(drawer: &Renderer<'_>, fw: &mut Framework)
{
	let pt = &drawer.pieces_texture;
	let fr = drawer.field_rect;

	let canvas = &mut fw.canvas;

	canvas.copy(pt, None, fr).unwrap();
}

pub fn draw_player(state: &TetrisState, drawer: &Renderer<'_>, fw: &mut Framework)
{
	let pcs = &state.piece_colors;
	let pbs = &state.piece_blocks;
	let pos = state.piece_loc;
	let proj = state.piece_proj;
	let fr = drawer.field_rect;
	let bs = drawer.block_size;

	debug_assert_eq!(pcs.len(), pbs.len());

	let canvas = &mut fw.canvas;

	for (c, b) in pcs.iter().zip(pbs) {
		let color = Color::RGBA(c.r, c.g, c.b, c.a / 2);
		let block = Rect::new(
			fr.x + (b.x + pos.x) * bs as i32,
			fr.y + (b.y + proj) * bs as i32,
			bs,
			bs,
		);

		canvas.set_draw_color(color);
		canvas.fill_rect(block).unwrap();

		let color = *c;
		let block = Rect::new(
			fr.x + (b.x + pos.x) * bs as i32,
			fr.y + (b.y + pos.y) * bs as i32,
			bs,
			bs,
		);

		canvas.set_draw_color(color);
		canvas.fill_rect(block).unwrap();
	}
}

pub fn draw_field(fw: &mut Framework, drawer: &Renderer<'_>)
{
	let fr = drawer.field_rect;
	let bt = &drawer.pieces_texture;

	let canvas = &mut fw.canvas;

	canvas.set_draw_color(Color::BLACK);
	canvas.fill_rect(fr).unwrap();

	canvas.copy(bt, None, fr).unwrap();
}
