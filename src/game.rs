use std::time::Instant;

use log::info;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use crate::error::Error;

use self::pieces::Direction;
use self::theme_api::StateData;

mod drawer;
mod field;
mod gen;
mod pieces;
mod size;
mod theme;
mod theme_api;

pub fn load_defaults(ctx: &rlua::Context) -> Result<(), Error>
{
	let solve_field = ctx.create_function(|_, data: rlua::LightUserData| {
		let StateData { game, .. }: &mut StateData =
			unsafe { &mut *(data.0 as *mut theme_api::StateData) };

		game.clear_lines();

		Ok(())
	})?;

	let g = ctx.globals();
	g.set("_solveField", solve_field)?;

	Ok(())
}

// -----------------------------------------------------------------------------
// Game State
// -----------------------------------------------------------------------------

pub struct TetrisState<'a, 'b>
{
	// Field
	field_blocks: Vec<Point>,
	field_colors: Vec<Color>,
	field_size: (usize, usize),

	// Piece
	piece_proj: i32,
	piece_loc: Point,
	piece_dim: usize,
	piece_blocks: Vec<Point>,
	piece_colors: Vec<Color>,

	// Drawer
	rblock_size: u32,
	rfield_rect: Rect,
	rblocks_texture: Texture<'a>,

	// Lua context
	lua_ctx: rlua::Context<'b>,

	// Stats
	score: u64,
	time: Instant,
	lines_cleared: u64,
	pieces_placed: u64,
}

impl<'a, 'b, 'c> TetrisState<'a, 'b>
{
	pub fn init(
		tc: &'a TextureCreator<WindowContext>,
		canvas: &mut WindowCanvas,
		lua_ctx: rlua::Context<'b>,
	) -> Result<Self, Error>
	{
		let t = theme::load(&lua_ctx)?;

		let (field_blocks, field_colors, field_size) = field::init(t.field_dim);

		let (piece_blocks, piece_colors, piece_dim, piece_loc, piece_proj) = pieces::init();

		let p = size::new_resize(canvas.output_size().unwrap(), field_size);
		let (rblock_size, rfield_rect, rblocks_texture) = drawer::init(tc, p);

		let score = 0;
		let time = Instant::now();
		let lines_cleared = 0;
		let pieces_placed = 0;

		let mut state = TetrisState {
			field_blocks,
			field_colors,
			field_size,
			piece_proj,
			piece_loc,
			piece_dim,
			piece_blocks,
			piece_colors,
			rblock_size,
			rfield_rect,
			rblocks_texture,
			lua_ctx,
			score,
			time,
			lines_cleared,
			pieces_placed,
		};

		if !state.spawn_piece(canvas)? {
			return Err(Error::from("Piece couldn't spawn."));
		}

		Ok(state)
	}
}

impl TetrisState<'_, '_>
{
	fn spawn_piece(&mut self, canvas: &mut WindowCanvas) -> Result<bool, Error>
	{
		info!("Respawning piece.");

		let t = theme_api::call_lua("spawn_piece", self, canvas)?;

		let (dim, colors, blocks) = theme::parse_pattern(t)?;
		let p = gen::Piece {
			dim,
			colors,
			blocks,
		};

		let fb = &self.field_blocks;
		let fs = self.field_size;

		if let Some((pb, pc, pd, pp, pj)) = pieces::spawn_new(p, fs, fb) {
			self.piece_blocks = pb;
			self.piece_colors = pc;
			self.piece_dim = pd;
			self.piece_loc = pp;
			self.piece_proj = pj;
			return Ok(true);
		}

		Ok(false)
	}

	fn clear_lines(&mut self) -> Vec<i32>
	{
		let fs = self.field_size;
		let fb = &mut self.field_blocks;
		let fc = &mut self.field_colors;

		let lines = field::clear_lines(fs, fb, fc);

		let lc = &mut self.lines_cleared;
		*lc += lines.len() as u64;

		lines
	}

	fn draw_blocks(&mut self, canvas: &mut WindowCanvas)
	{
		let ft = &mut self.rblocks_texture;
		let bs = self.rblock_size;
		let fb = &self.field_blocks;
		let fc = &self.field_colors;

		canvas
			.with_texture_canvas(ft, |canvas| {
				canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
				canvas.clear();

				drawer::draw_blocks(canvas, bs, &fb, &fc);
			})
			.unwrap();
	}

	fn place_piece(&mut self, canvas: &mut WindowCanvas)
	{
		info!("Placing piece.");

		let fc = &mut self.field_colors;
		let fb = &mut self.field_blocks;
		let pb = &self.piece_blocks;
		let pc = &self.piece_colors;
		let pl = self.piece_loc;

		fb.extend(pb.iter().map(|b| Point::new(b.x + pl.x, b.y + pl.y)));
		fc.extend(pc);

		self.clear_lines();
		self.draw_blocks(canvas);

		let pp = &mut self.pieces_placed;
		*pp += 1;
	}

	pub fn drop(&mut self, canvas: &mut WindowCanvas) -> Result<bool, Error>
	{
		info!("Dropping piece.");

		let pj = self.piece_proj;

		self.piece_loc.y = pj;

		self.place_piece(canvas);
		self.spawn_piece(canvas)
	}

	fn rotate(&mut self)
	{
		let fb = &self.field_blocks;
		let fs = self.field_size;
		let pb = &self.piece_blocks;
		let pl = self.piece_loc;
		let pd = self.piece_dim;

		let new_pb: Vec<Point> = pb.iter().map(|b| Point::new(pd as i32 - 1 - b.y, b.x)).collect();

		if field::check_valid_pos(fs, fb, pl, &new_pb) {
			info!("Rotating piece.");

			let p = pieces::project(fs, fb, pl, &new_pb);
			self.piece_blocks = new_pb;
			self.piece_proj = p;
		}
	}

	fn move_piece(&mut self, d: Direction)
	{
		let fb = &self.field_blocks;
		let fs = self.field_size;
		let pb = &self.piece_blocks;
		let pl = self.piece_loc;

		if let Some((pl, proj)) = pieces::move_piece(fs, fb, pl, pb, d) {
			self.piece_loc = pl;
			self.piece_proj = proj;
		}
	}

	fn move_piece_down(&mut self, canvas: &mut WindowCanvas) -> Result<bool, Error>
	{
		let fb = &self.field_blocks;
		let fs = self.field_size;
		let pb = &self.piece_blocks;
		let pl = self.piece_loc;

		if let Some((pl, proj)) = pieces::move_piece(fs, fb, pl, pb, Direction::DOWN) {
			self.piece_loc = pl;
			self.piece_proj = proj;
			return Ok(true);
		}

		self.place_piece(canvas);
		self.spawn_piece(canvas)
	}

	fn draw_field(&self, canvas: &mut WindowCanvas)
	{
		let fr = self.rfield_rect;
		let bt = &self.rblocks_texture;

		canvas.set_draw_color(Color::BLACK);
		canvas.fill_rect(fr).unwrap();

		canvas.copy(bt, None, fr).unwrap();
	}

	pub fn draw_player(&self, canvas: &mut WindowCanvas)
	{
		let pcs = &self.piece_colors;
		let pbs = &self.piece_blocks;
		let fr = self.rfield_rect;
		let bs = self.rblock_size;
		let pos = self.piece_loc;
		let proj = self.piece_proj;

		debug_assert_eq!(pcs.len(), pbs.len());

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

	pub fn output_score(&self)
	{
		println!(
			"Well done! Here are your stats.\nScore: {}\nTime: {}\nLines cleared: {}\nPieces placed: {}",
			self.score,
			self.time.elapsed().as_secs_f64(),
            self.lines_cleared,
            self.pieces_placed,
		);
	}
}

// -----------------------------------------------------------------------------
// Game
// -----------------------------------------------------------------------------

pub fn handle_event(
	event: &Event,
	canvas: &mut WindowCanvas,
	state: &mut TetrisState,
) -> Result<bool, Error>
{
	match event {
		Event::KeyDown {
			keycode: Some(x), ..
		} => match x {
			Keycode::Left => {
				state.move_piece(Direction::LEFT);
			}

			Keycode::Right => {
				state.move_piece(Direction::RIGHT);
			}

			Keycode::Down => {
				return Ok(state.move_piece_down(canvas)?);
			}

			Keycode::Up => {
				state.rotate();
			}

			Keycode::Space => {
				return Ok(state.drop(canvas)?);
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

	Ok(true)
}

pub fn draw(state: &TetrisState, canvas: &mut WindowCanvas)
{
	state.draw_field(canvas);
	state.draw_player(canvas);
}
