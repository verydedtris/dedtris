use std::{path::Path, time::Duration};

use log::info;
use sdl2::{
	event::{Event, WindowEvent},
	keyboard::Keycode,
	pixels::Color,
	rect::{Point, Rect},
	render::{Texture, TextureCreator, WindowCanvas},
	video::WindowContext,
	Sdl, VideoSubsystem,
};

use self::{
	drawer::Renderer,
	state::{gen::Piece, Direction, TetrisState},
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
		let t = {
			theme_api::load_defaults(&ctx)?;
			lua::exec_file(&ctx, Path::new("Themes/default.lua"))?;
			lua::exec_file(&ctx, Path::new("Themes/test.lua"))?;

			theme::load(&ctx)?
		};

		info!("Constructing window.");

		let window_size = (1080, 720);

		let window = video_sys
			.window("Tetris", window_size.0, window_size.1)
			.position_centered()
			// .resizable() // Simpler to debug
			.build()?;

		info!("Initializing renderer.");

		let mut canvas = {
			let mut canvas = window.into_canvas().accelerated().target_texture().build()?;

			canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
			canvas.set_draw_color(Color::RGB(0, 255, 255));
			canvas.clear();
			canvas.present();

			println!("{:?}", canvas.info().texture_formats);

			canvas
		};

		let tex_maker = canvas.texture_creator();

		let mut fw = Framework {
			sdl:       sdl_context,
			video:     video_sys,
			canvas:    &mut canvas,
			tex_maker: &tex_maker,
			lua:       &ctx,
		};

		info!("Initializing tetris game.");

		let mut renderer = drawer::init_renderer(
			&fw.tex_maker,
			fw.canvas,
			window_size,
			t.field_dim,
			&t.start_piece,
		)?;
		let mut game = state::init_game(t.field_dim, t.start_piece)?;

		refresh_piece_view(&mut fw, &mut game, &mut renderer, t.piece_view_size)?;

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

			draw(&game, &mut renderer, &mut fw);
			fw.canvas.present();

			::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}

		output_score(&game);

		Ok(())
	})
}

pub fn handle_event<'a>(
	event: &Event, fw: &mut Framework<'_, '_, '_, 'a, '_, '_>, drawer: &mut Renderer<'a>,
	state: &mut TetrisState,
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
				rotate(state, drawer);
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
			resize_game(state, fw, drawer, (*w as u32, *h as u32));
		},

		_ => (),
	}

	Ok(!state.exit)
}

// -----------------------------------------------------------------------------
// Game actions
// -----------------------------------------------------------------------------

pub fn rotate(state: &mut TetrisState, drawer: &mut Renderer<'_>)
{
	if state::rotate(state) {
		drawer.player_angle = (drawer.player_angle as u32 + 90 % 360) as f64;
	}
}

pub fn refresh_piece_view<'a>(
	fw: &mut Framework<'_, '_, '_, 'a, '_, '_>, state: &mut TetrisState, drawer: &mut Renderer<'a>,
	size: usize,
) -> Result<(), Error>
{
	let b = request_pieces(fw, state, drawer, size)?;

	state.piece_queue = b.pieces;
	drawer.piece_view_textures = b.textures;

	Ok(())
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

pub fn spawn_piece<'a>(
	state: &mut TetrisState, rend: &mut Renderer<'a>, fw: &mut Framework<'_, '_, '_, 'a, '_, '_>,
) -> Result<bool, Error>
{
	info!("Respawning piece.");

	let t = theme_api::call_lua("spawn_piece", state, fw)?;
	let p = theme::parse_pattern(t)?;

	let canvas = &mut fw.canvas;
	let tc = &fw.tex_maker;

	drawer::new_player_texture(rend, tc, canvas, &p);

	Ok(state::spawn_piece(state, p))
}

pub fn place_piece<'a>(
	state: &mut TetrisState, rend: &mut Renderer<'a>, fw: &mut Framework<'_, '_, '_, 'a, '_, '_>,
) -> Result<bool, Error>
{
	info!("Placing piece.");

	let fb = &mut state.field_blocks;
	let fc = &mut state.field_colors;
	let p = &state.player_piece;
	let pp = state.player_pos;

	// Add blocks to state
	{
		fb.extend(p.blocks.iter().map(|b| Point::new(b.x + pp.x, b.y + pp.y)));
		fc.extend(state.player_piece.colors.iter());
	}

	theme_api::call_lua::<()>("on_place", state, fw)?;

	// Redraw field blocks
	{
		regen_blocks(fw, state, rend);
	}

	state.pieces_placed += 1;

	regen_blocks(fw, state, rend);

	spawn_piece(state, rend, fw)
}

pub fn drop<'a>(
	state: &mut TetrisState, rend: &mut Renderer<'a>, fw: &mut Framework<'_, '_, '_, 'a, '_, '_>,
) -> Result<bool, Error>
{
	info!("Dropping piece.");

	state.player_pos.y = state.player_proj;

	place_piece(state, rend, fw)
}

fn move_piece_down<'a>(
	state: &mut TetrisState, drawer: &mut Renderer<'a>, fw: &mut Framework<'_, '_, '_, 'a, '_, '_>,
) -> Result<bool, Error>
{
	if state::move_piece(state, Direction::DOWN) {
		return Ok(true);
	}

	place_piece(state, drawer, fw)
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
// Queries
// -----------------------------------------------------------------------------

pub struct Buffer<'a>
{
	pub pieces:   Vec<Piece>,
	pub textures: Vec<Texture<'a>>,
}

pub fn request_pieces<'a>(
	fw: &mut Framework<'_, '_, '_, 'a, '_, '_>, state: &mut TetrisState, drawer: &mut Renderer<'a>,
	size: usize,
) -> Result<Buffer<'a>, Error>
{
	let mut pieces = Vec::with_capacity(size);

	while pieces.len() < size {
		let t = theme_api::call_lua("spawn_piece", state, fw)?;
		let p = theme::parse_pattern(t)?;

		pieces.push(p);
	}

	let textures = drawer::player::create_piece_textures(
		&fw.tex_maker,
		&mut fw.canvas,
		drawer.block_size,
		&pieces,
	);

	Ok(Buffer { pieces, textures })
}

// -----------------------------------------------------------------------------
// Rendering
// -----------------------------------------------------------------------------

pub fn draw(state: &TetrisState, drawer: &mut Renderer<'_>, fw: &mut Framework)
{
	let canvas = &mut fw.canvas;
	let p = &state.player_piece;
	let pos = state.player_pos;
	let proj = state.player_proj;
	let pt = &mut drawer.player_texture;
	let pft = &drawer.pieces_texture;
	let pvts = &drawer.piece_view_textures;
	let fr = drawer.field_rect;
	let bs = drawer.block_size;
	let angle = drawer.player_angle;

	// Draw field
	{
		canvas.set_draw_color(Color::BLACK);
		canvas.fill_rect(fr).unwrap();
	}

	// Draw field blocks
	{
		canvas.copy(pft, None, fr).unwrap();
	}

	// Draw player
	{
		let x = fr.x + pos.x * bs as i32;
		let y = fr.y + proj * bs as i32;
		let size = p.dim * bs;

		canvas
			.copy_ex(
				pt,
				None,
				Rect::new(x, y, size, size),
				angle,
				None,
				false,
				false,
			)
			.unwrap();

		let mask: Vec<Rect> = p
			.blocks
			.iter()
			.map(|block| Rect::new(x + block.x * bs as i32, y + block.y * bs as i32, bs, bs))
			.collect();

		canvas.set_draw_color(Color::RGBA(0, 0, 0, 127));
		canvas.fill_rects(&mask).unwrap();

		let y = fr.y + pos.y * bs as i32;

		canvas
			.copy_ex(
				pt,
				None,
				Rect::new(x, y, size, size),
				angle,
				None,
				false,
				false,
			)
			.unwrap();

	}

	// Draw piece view
	{
		let x = fr.x + fr.w + 5;
		let mut y = fr.y;

		for t in pvts.iter() {
			let wh = t.query();

			canvas.copy(t, None, Rect::new(x, y, wh.width, wh.height)).unwrap();
			y += wh.height as i32 + 5;
		}
	}
}

// -----------------------------------------------------------------------------
// Scaling
// -----------------------------------------------------------------------------

pub fn resize_game<'a>(
	state: &TetrisState, fw: &mut Framework<'_, '_, '_, 'a, '_, '_>, drawer: &mut Renderer<'a>,
	win_dim: (u32, u32),
)
{
	let fd = state.field_size;
	let tc = fw.tex_maker;

	let rp = drawer::size::new_resize(win_dim, fd);

	let new_tex = drawer::recreate_texture(tc, (rp.field_rect.w as u32, rp.field_rect.h as u32));

	drawer.block_size = rp.block_size;
	drawer.field_rect = rp.field_rect;
	drawer.pieces_texture = new_tex;

	regen_blocks(fw, &state, drawer);
}
