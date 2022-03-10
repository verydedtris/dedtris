use std::time::Instant;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

use super::Size;


pub struct TetrisState
{
	// Field
	pub field_blocks: Vec<Point>,
	pub field_colors: Vec<Color>,
	pub field_size: Size,

	// Piece view
	// piece_view_pieces: Vec<gen::Piece>,

	// Piece
	pub piece_proj: i32,
	pub piece_loc: Point,
	pub piece_dim: u32,
	pub piece_blocks: Vec<Point>,
	pub piece_colors: Vec<Color>,

	// Drawer
	// 	rblock_size: u32,
	// 	rfield_rect: Rect,
	// 	rblocks_texture: Texture<'a>,
	// rtextures: Vec<Texture<'a>>,

	// Stats
	pub time: Instant,
	pub lines_cleared: u64,
	pub pieces_placed: u64,

	// Exit through Lua
	pub exit: bool,
}

pub fn init_game(fw: &Framework, t: &Theme) -> Result<TetrisState, Error>
{
	let lua_ctx = fw.lua;

	let FieldComponent {
		blocks: field_blocks,
		colors: field_colors,
		dim: field_size,
	} = field::init(t.field_dim.0, t.field_dim.1);

	let MoveablePieceComponent {
		blocks: piece_blocks,
		colors: piece_colors,
		dim: piece_dim,
		pos: piece_loc,
		projection: piece_proj,
	} = pieces::init();

	// let p = size::new_resize(canvas.output_size().unwrap(), field_size);
	// let (rblock_size, rfield_rect, rblocks_texture) = drawer::init(tc, p);

	let time = Instant::now();
	let lines_cleared = 0;
	let pieces_placed = 0;

	let exit = false;

	let mut state = TetrisState {
		field_blocks,
		field_colors,
		field_size,
		piece_proj,
		piece_loc,
		piece_dim,
		piece_blocks,
		piece_colors,
		time,
		lines_cleared,
		pieces_placed,
		exit,
	};

	if !spawn_piece(&mut state, fw)? {
		return Err(Error::from("Piece couldn't spawn."));
	}

	Ok(state)
}

fn spawn_piece(state: &mut TetrisState, fw: &Framework) -> Result<bool, Error>
{
	info!("Respawning piece.");

	let t = theme_api::call_lua("spawn_piece", state, fw)?;

	let (dim, colors, blocks) = theme::parse_pattern(t)?;
	let p = gen::Piece {
		dim,
		colors,
		blocks,
	};

	let fb = &state.field_blocks;
	let fs = state.field_size;

	if let Some(pieces::NewPiece {
		blocks: pb,
		colors: pc,
		dim: pd,
		pos: pp,
		projection: pj,
	}) = pieces::spawn_new(p, fs, fb)
	{
		state.piece_blocks = pb;
		state.piece_colors = pc;
		state.piece_dim = pd;
		state.piece_loc = pp;
		state.piece_proj = pj;
		return Ok(true);
	}

	Ok(false)
}

fn clear_lines(state: &mut TetrisState) -> Vec<i32>
{
	let fs = state.field_size;
	let fb = &mut state.field_blocks;
	let fc = &mut state.field_colors;

	let lines = field::clear_lines(fs, fb, fc);

	let lc = &mut state.lines_cleared;
	*lc += lines.len() as u64;

	lines
}
