use std::time::Instant;

use log::info;
use sdl2::{pixels::Color, rect::Point};

use super::{theme::Theme, Size};
use crate::error::Error;

pub mod field;
pub mod gen;
pub mod pieces;

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

	// Piece queue
	pub piece_queue:     Vec<gen::Piece>,
	pub piece_queue_idx: usize,

	// Piece
	pub player_proj:  i32,
	pub player_pos:   Point,
	pub player_piece: gen::Piece,

	// Stats
	pub time:          Instant,
	pub lines_cleared: u64,
	pub pieces_placed: u64,

	// Exit through Lua
	pub exit: bool,
}

pub fn init_game(field_dim: Size, start_piece: gen::Piece) -> Result<TetrisState, Error>
{
	let field_blocks = Vec::new();
	let field_colors = Vec::new();
	let field_size = field_dim;

	let player = match pieces::spawn_piece(&field_blocks, field_dim, start_piece) {
		Some(v) => v,
		None => return Err(Error::from("No area for piece.")),
	};

	Ok(TetrisState {
		field_blocks,
		field_colors,
		field_size,

		piece_queue: Vec::new(),
		piece_queue_idx: 0,

		player_proj: player.proj,
		player_pos: player.pos,
		player_piece: player.piece,

		time: Instant::now(),
		lines_cleared: 0,
		pieces_placed: 0,

		exit: false,
	})
}

pub fn clear_lines(state: &mut TetrisState) -> Vec<i32>
{
	let fs = state.field_size;
	let fb = &mut state.field_blocks;
	let fc = &mut state.field_colors;

	let lines = field::clear_lines(fs, fb, fc);

	state.lines_cleared += lines.len() as u64;

	lines
}

pub fn rotate(state: &mut TetrisState) -> bool
{
	let fb = &state.field_blocks;
	let fs = state.field_size;
	let p = &state.player_piece;
	let pp = state.player_pos;

	let new_pblocks: Vec<Point> =
		p.blocks.iter().map(|b| Point::new(p.dim as i32 - 1 - b.y, b.x)).collect();

	if field::check_valid_pos(fs, fb, pp, &new_pblocks) {
		info!("Rotating piece.");

		let new_proj = pieces::project(fs, fb, pp, &new_pblocks);

		state.player_piece.blocks = new_pblocks;
		state.player_proj = new_proj;

		true
	} else {
		false
	}
}

pub fn move_piece(state: &mut TetrisState, d: Direction) -> bool
{
	let fb = &state.field_blocks;
	let fs = state.field_size;
	let p = &state.player_piece;
	let pl = state.player_pos;

	info!("Moving to {:?}.", d);

	let new_pl = match d {
		Direction::LEFT => Point::new(pl.x - 1, pl.y),
		Direction::RIGHT => Point::new(pl.x + 1, pl.y),
		Direction::DOWN => Point::new(pl.x, pl.y + 1),
	};

	if field::check_valid_pos(fs, fb, new_pl, &p.blocks) {
		let p = pieces::project(fs, fb, new_pl, &p.blocks);

		state.player_pos = new_pl;
		state.player_proj = p;

		true
	} else {
		false
	}
}

pub fn spawn_piece(state: &mut TetrisState, piece: gen::Piece) -> bool
{
	info!("Spawning piece.");

	let player = if let Some(p) = pieces::spawn_piece(&state.field_blocks, state.field_size, piece)
	{
		p
	} else {
		return false;
	};

	state.player_piece = player.piece;
	state.player_pos = player.pos;
	state.player_proj = player.proj;

	true
}
