use std::time::Instant;

use log::info;
use sdl2::{pixels::Color, rect::Point};

use super::{Piece, Size};
use crate::error::Error;

pub mod field;
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
	pub piece_queue:     Vec<Piece>,
	pub piece_queue_idx: usize,

	// Piece swap
	pub piece_swap: Option<Piece>,

	// Piece
	pub player_proj:  i32,
	pub player_pos:   Point,
	pub player_piece: Piece,

	// Stats
	pub time:          Instant,
	pub lines_cleared: u64,
	pub pieces_placed: u64,

	// Exit through Lua
	pub exit: bool,
}

pub fn init_game(field_dim: Size, start_piece: Piece) -> Result<TetrisState, Error>
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

		piece_swap: None,

		player_proj: player.proj,
		player_pos: player.pos,
		player_piece: player.piece,

		time: Instant::now(),
		lines_cleared: 0,
		pieces_placed: 0,

		exit: false,
	})
}

impl TetrisState
{
	pub fn clear_lines(&mut self) -> Vec<i32>
	{
		let fb = &mut self.field_blocks;
		let fc = &mut self.field_colors;
		let fs = self.field_size;

		let lines = field::clear_lines(fs, fb, fc);
		self.lines_cleared += lines.len() as u64;

		lines
	}

	pub fn rotate(&mut self) -> bool
	{
		let p = &self.player_piece;

		let new_pblocks: Vec<Point> =
			p.blocks.iter().map(|b| Point::new(p.dim as i32 - 1 - b.y, b.x)).collect();

		let fb = &self.field_blocks;
		let fs = self.field_size;
		let pp = self.player_pos;

		if !field::check_valid_pos(fs, fb, pp, &new_pblocks) {
			return false;
		}

		let p = pieces::project(fs, fb, pp, &new_pblocks);

		self.player_piece.blocks = new_pblocks;
		self.player_proj = p;

		true
	}

	pub fn move_piece(&mut self, d: Direction) -> bool
	{
		let pl = self.player_pos;

		info!("Moving to {:?}.", d);

		let new_pl = match d {
			Direction::LEFT => Point::new(pl.x - 1, pl.y),
			Direction::RIGHT => Point::new(pl.x + 1, pl.y),
			Direction::DOWN => Point::new(pl.x, pl.y + 1),
		};

		let fb = &self.field_blocks;
		let fs = self.field_size;
		let p = &self.player_piece;

		if !field::check_valid_pos(fs, fb, new_pl, &p.blocks) {
			return false;
		}

		let p = pieces::project(fs, fb, new_pl, &p.blocks);

		self.player_pos = new_pl;
		self.player_proj = p;

		true
	}

	pub fn output_score(&self)
	{
		println!(
			"Well done! Here are your stats.\nScore: {}\nTime: {}\nLines cleared: {}\nPieces \
			 placed: {}",
			self.lines_cleared as f64
				/ (self.time.elapsed().as_secs_f64() * self.pieces_placed as f64),
			self.time.elapsed().as_secs_f64(),
			self.lines_cleared,
			self.pieces_placed,
		);
	}

	pub fn spawn_piece_direct(&mut self, piece: Piece) -> bool
	{
		let fbs = &self.field_blocks;
		let fs = self.field_size;

		let player = if let Some(p) = pieces::spawn_piece(fbs, fs, piece) {
			p
		} else {
			return false;
		};

		self.player_piece = player.piece;
		self.player_pos = player.pos;
		self.player_proj = player.proj;

		true
	}

	pub fn push_piece(&mut self, piece: Piece) -> Piece
	{
		let pvb = &mut self.piece_queue;
		let idx = self.piece_queue_idx;

		if pvb.len() > 0 {
			let p = std::mem::replace(&mut pvb[idx], piece);
			self.piece_queue_idx = (idx + 1) % pvb.len();
			p
		} else {
			piece
		}
	}

	pub fn spawn_piece(&mut self, piece: Piece) -> bool
	{
		info!("Spawning piece.");

		let piece = self.push_piece(piece);
		self.spawn_piece_direct(piece)
	}
}
