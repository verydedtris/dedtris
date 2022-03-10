use std::path::Path;
use std::time::{Duration, Instant};

use log::info;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::{Sdl, VideoSubsystem};

use crate::error::Error;
use crate::lua;

use self::field::FieldComponent;
use self::pieces::{Direction, MoveablePieceComponent};
use self::state::TetrisState;
use self::theme::Theme;
use self::theme_api::StateData;

mod drawer;
mod state;
mod theme;
mod theme_api;

type Size = (u32, u32);

pub fn load_defaults(ctx: &rlua::Context) -> Result<(), Error>
{
	let solve_field = ctx.create_function(|_, data: rlua::LightUserData| {
		let StateData { game, .. }: &mut StateData =
			unsafe { &mut *(data.0 as *mut theme_api::StateData) };

		let v = clear_lines(game);

		Ok(v)
	})?;

	let exit_game = ctx.create_function(|_, data: rlua::LightUserData| {
		let StateData { game, .. }: &mut StateData =
			unsafe { &mut *(data.0 as *mut theme_api::StateData) };

		game.exit = true;

		Ok(())
	})?;

	let g = ctx.globals();
	g.set("_solveField", solve_field)?;
	g.set("_finishGame", exit_game)?;

	Ok(())
}

pub struct Framework<'a, 'b, 'd, 'e, 'f, 'g>
{
	sdl: &'a Sdl,
	video: &'b VideoSubsystem,
	canvas: &'d mut WindowCanvas,
	tex_maker: &'e TextureCreator<WindowContext>,
	lua: &'f rlua::Context<'g>,
}

// -----------------------------------------------------------------------------
// Game State
// -----------------------------------------------------------------------------

pub fn start_tetris_game(sdl_context: &Sdl, video_sys: &VideoSubsystem) -> Result<(), Error>
{
	info!("Initializing Lua plugin enviroment.",);

	let lua = rlua::Lua::new();

	info!("Constructing window.");

	let window = video_sys
		.window("Tetris", 800, 600)
		.position_centered()
		//.resizable() // Simpler to debug
		.build()?;

	let mut canvas = window.into_canvas().accelerated().target_texture().build()?;

	let tex_maker = canvas.texture_creator();

	canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
	canvas.set_draw_color(Color::RGB(0, 255, 255));
	canvas.clear();
	canvas.present();

	lua.context::<_, Result<(), Error>>(|ctx| {
		let mut fw = Framework {
			sdl: sdl_context,
			video: video_sys,
			canvas: &mut canvas,
			tex_maker: &tex_maker,
			lua: &ctx,
		};

		// Load theme file

		let lua_ctx = fw.lua;

		load_defaults(&lua_ctx)?;
		lua::exec_file(&lua_ctx, Path::new("Themes/default.lua"))?;
		lua::exec_file(&lua_ctx, Path::new("Themes/test.lua"))?;

		// Init Game

		info!("Initializing tetris game.");

		let t = theme::load(&lua_ctx)?;

		let mut game = init_game(&fw, &t)?;
		let mut renderer = init_renderer(&fw, &t)?;

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
						if !handle_event(&event, &fw, &mut game)? {
							break 'running;
						}
					}
				}
			}

			// draw(&game, &mut canvas);

			let canvas = &mut fw.canvas;

			canvas.present();
			::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}

		output_score(&game);

		Ok(())
	})
}

pub fn place_piece(state: &mut TetrisState, fw: &Framework) -> Result<(), Error>
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

	// state.draw_blocks(canvas);

	let pp = &mut state.pieces_placed;
	*pp += 1;

	Ok(())
}

pub fn drop(state: &mut TetrisState, fw: &Framework) -> Result<bool, Error>
{
	info!("Dropping piece.");

	let pj = state.piece_proj;

	state.piece_loc.y = pj;

	place_piece(state, fw)?;
	spawn_piece(state, fw)
}

fn rotate(state: &mut TetrisState)
{
	let fb = &state.field_blocks;
	let fs = state.field_size;
	let pb = &state.piece_blocks;
	let pl = state.piece_loc;
	let pd = state.piece_dim;

	let new_pb: Vec<Point> = pb.iter().map(|b| Point::new(pd as i32 - 1 - b.y, b.x)).collect();

	if field::check_valid_pos(fs, fb, pl, &new_pb) {
		info!("Rotating piece.");

		let p = pieces::project(fs, fb, pl, &new_pb);
		state.piece_blocks = new_pb;
		state.piece_proj = p;
	}
}

fn move_piece(state: &mut TetrisState, d: Direction)
{
	let fb = &state.field_blocks;
	let fs = state.field_size;
	let pb = &state.piece_blocks;
	let pl = state.piece_loc;

	if let Some((pl, proj)) = pieces::move_piece(fs, fb, pl, pb, d) {
		state.piece_loc = pl;
		state.piece_proj = proj;
	}
}

fn move_piece_down(state: &mut TetrisState, fw: &Framework) -> Result<bool, Error>
{
	let fb = &state.field_blocks;
	let fs = state.field_size;
	let pb = &state.piece_blocks;
	let pl = state.piece_loc;

	if let Some((pl, proj)) = pieces::move_piece(fs, fb, pl, pb, Direction::DOWN) {
		state.piece_loc = pl;
		state.piece_proj = proj;
		return Ok(true);
	}

	place_piece(state, fw)?;
	spawn_piece(state, fw)
}

// fn draw_field(&self, canvas: &mut WindowCanvas)
// {
// 	let fr = self.rfield_rect;
// 	let bt = &self.rblocks_texture;
//
// 	canvas.set_draw_color(Color::BLACK);
// 	canvas.fill_rect(fr).unwrap();
//
// 	canvas.copy(bt, None, fr).unwrap();
// }
//
// pub fn draw_player(&self, canvas: &mut WindowCanvas)
// {
// 	let pcs = &self.piece_colors;
// 	let pbs = &self.piece_blocks;
// 	let fr = self.rfield_rect;
// 	let bs = self.rblock_size;
// 	let pos = self.piece_loc;
// 	let proj = self.piece_proj;
//
// 	debug_assert_eq!(pcs.len(), pbs.len());
//
// 	for (c, b) in pcs.iter().zip(pbs) {
// 		let color = Color::RGBA(c.r, c.g, c.b, c.a / 2);
// 		let block = Rect::new(
// 			fr.x + (b.x + pos.x) * bs as i32,
// 			fr.y + (b.y + proj) * bs as i32,
// 			bs,
// 			bs,
// 		);
//
// 		canvas.set_draw_color(color);
// 		canvas.fill_rect(block).unwrap();
//
// 		let color = *c;
// 		let block = Rect::new(
// 			fr.x + (b.x + pos.x) * bs as i32,
// 			fr.y + (b.y + pos.y) * bs as i32,
// 			bs,
// 			bs,
// 		);
//
// 		canvas.set_draw_color(color);
// 		canvas.fill_rect(block).unwrap();
// 	}
// }

pub fn output_score(state: &TetrisState)
{
	println!(
			"Well done! Here are your stats.\nScore: {}\nTime: {}\nLines cleared: {}\nPieces placed: {}",
			state.lines_cleared as f64 / (state.time.elapsed().as_secs_f64() * state.pieces_placed as f64),
			state.time.elapsed().as_secs_f64(),
            state.lines_cleared,
            state.pieces_placed,
		);
}

// -----------------------------------------------------------------------------
// Game
// -----------------------------------------------------------------------------

pub fn handle_event(event: &Event, fw: &Framework, state: &mut TetrisState) -> Result<bool, Error>
{
	match event {
		Event::KeyDown {
			keycode: Some(x), ..
		} => match x {
			Keycode::Left => {
				move_piece(state, Direction::LEFT);
			}

			Keycode::Right => {
				move_piece(state, Direction::RIGHT);
			}

			Keycode::Down => {
				return Ok(move_piece_down(state, fw)?);
			}

			Keycode::Up => {
				rotate(state);
			}

			Keycode::Space => {
				return Ok(drop(state, fw)?);
			}

			_ => (),
		},

		Event::Window {
			win_event: WindowEvent::Resized(w, h),
			..
		} => {
			// set_layout_size(&mut self.draw_cache, &self.field, (*w as u32, *h as u32));
		}

		_ => (),
	}

	Ok(!state.exit)
}

// pub fn draw(state: &TetrisState, canvas: &mut WindowCanvas)
// {
// 	state.draw_field(canvas);
// 	state.draw_player(canvas);
// }
