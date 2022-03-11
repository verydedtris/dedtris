use std::time::Instant;

use log::info;
use sdl2::{pixels::Color, rect::Point};

use self::{
	field::FieldComponent,
	pieces::{Direction, MoveablePieceComponent},
};
use super::{theme::Theme, Framework, Size};
use crate::{
	error::Error,
	game::{theme, theme_api},
};

mod field;
mod gen;
mod pieces;

pub struct TetrisState
{
	// Field
	pub field_blocks: Vec<Point>,
	pub field_colors: Vec<Color>,
	pub field_size:   Size,

	// Piece view
	// piece_view_pieces: Vec<gen::Piece>,

	// Piece
	pub piece_proj:   i32,
	pub piece_loc:    Point,
	pub piece_dim:    u32,
	pub piece_blocks: Vec<Point>,
	pub piece_colors: Vec<Color>,

	// Drawer
	// 	rblock_size: u32,
	// 	rfield_rect: Rect,
	// 	rblocks_texture: Texture<'a>,
	// rtextures: Vec<Texture<'a>>,

	// Stats
	pub time:          Instant,
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

pub fn spawn_piece(state: &mut TetrisState, fw: &Framework) -> Result<bool, Error>
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

pub fn clear_lines(state: &mut TetrisState) -> Vec<i32>
{
	let fs = state.field_size;
	let fb = &mut state.field_blocks;
	let fc = &mut state.field_colors;

	let lines = field::clear_lines(fs, fb, fc);

	let lc = &mut state.lines_cleared;
	*lc += lines.len() as u64;

	lines
}

pub fn rotate(state: &mut TetrisState)
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

pub fn move_piece(state: &mut TetrisState, d: Direction)
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
