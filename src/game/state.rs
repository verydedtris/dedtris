use std::time::Instant;

use log::info;
use sdl2::{pixels::Color, rect::Point};

use self::{field::FieldComponent, pieces::MoveablePieceComponent};
use super::{theme::Theme, Size};
use crate::error::Error;

mod field;
mod gen;
mod pieces;

#[derive(Debug)]
pub enum Direction
{
	DOWN,
	LEFT,
	RIGHT,
}

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

	// Stats
	pub time:          Instant,
	pub lines_cleared: u64,
	pub pieces_placed: u64,

	// Exit through Lua
	pub exit: bool,
}

pub fn init_game(t: &Theme) -> Result<TetrisState, Error>
{
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

	let time = Instant::now();
	let lines_cleared = 0;
	let pieces_placed = 0;

	let exit = false;

	Ok(TetrisState {
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
	})
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

pub fn move_piece(state: &mut TetrisState, d: Direction) -> bool
{
	let fb = &state.field_blocks;
	let fs = state.field_size;
	let pb = &state.piece_blocks;
	let pl = state.piece_loc;

	info!("Moving to {:?}.", d);

	let new_pl = match d {
		Direction::LEFT => Point::new(pl.x - 1, pl.y),
		Direction::RIGHT => Point::new(pl.x + 1, pl.y),
		Direction::DOWN => Point::new(pl.x, pl.y + 1),
	};

	if field::check_valid_pos(fs, fb, new_pl, pb) {
		let p = pieces::project(fs, fb, new_pl, pb);

		state.piece_loc = new_pl;
		state.piece_proj = p;

		true
	} else {
		false
	}
}

pub fn spawn_piece(
	state: &mut TetrisState,
	piece_blocks: Vec<Point>,
	piece_colors: Vec<Color>,
	piece_dim: u32,
) -> bool
{
	info!("Spawning piece.");

	let fb = &state.field_blocks;
	let fs = state.field_size;

	let pos = Point::new(((fs.0 - piece_dim) / 2) as i32, 0);

	if !field::check_valid_pos(fs, fb, pos, &piece_blocks) {
		return false;
	}

	let projection = pieces::project(fs, fb, pos, &piece_blocks);

	state.piece_blocks = piece_blocks;
	state.piece_colors = piece_colors;
	state.piece_dim = piece_dim;
	state.piece_loc = pos;
	state.piece_proj = projection;

	true
}
