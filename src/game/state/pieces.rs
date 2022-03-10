use log::info;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use super::field;
use super::gen;
use super::Size;

#[derive(Debug)]
pub enum Direction
{
	DOWN,
	LEFT,
	RIGHT,
}

// -----------------------------------------------------------------------------
// Movable Piece
// -----------------------------------------------------------------------------

pub struct MoveablePieceComponent
{
	pub blocks: Vec<Point>,
	pub colors: Vec<Color>,
	pub dim: u32,
	pub pos: Point,
	pub projection: i32,
}

pub fn init() -> MoveablePieceComponent
{
	MoveablePieceComponent {
		blocks: Vec::new(),
		colors: Vec::new(),
		dim: 0,
		pos: Point::new(0, 0),
		projection: 0,
	}
}

pub struct NewPiece
{
	pub blocks: Vec<Point>,
	pub colors: Vec<Color>,
	pub dim: u32,
	pub pos: Point,
	pub projection: i32,
}

pub fn spawn_new(p: gen::Piece, field_dim: Size, field_blocks: &[Point]) -> Option<NewPiece>
{
	info!("Spawning piece.");

	let blocks = p.blocks;
	let colors = p.colors;
	let dim = p.dim;
	let pos = Point::new(((field_dim.0 - dim) / 2) as i32, 0);

	if !field::check_valid_pos(field_dim, field_blocks, pos, &blocks) {
		return None;
	}

	let projection = project(field_dim, field_blocks, pos, &blocks);

	Some(NewPiece {
		blocks,
		colors,
		dim,
		pos,
		projection,
	})
}

pub fn project(field_dim: Size, field_blocks: &[Point], pos: Point, blocks: &[Point]) -> i32
{
	let mut y = pos.y + 1;

	while field::check_valid_pos(field_dim, field_blocks, Point::new(pos.x, y), blocks) {
		y += 1;
	}

	y - 1
}

pub fn move_piece(
	fd: Size,
	fb: &[Point],
	pl: Point,
	pb: &[Point],
	d: Direction,
) -> Option<(Point, i32)>
{
	info!("Moving to {:?}.", d);

	let new_pl = match d {
		Direction::LEFT => Point::new(pl.x - 1, pl.y),
		Direction::RIGHT => Point::new(pl.x + 1, pl.y),
		Direction::DOWN => Point::new(pl.x, pl.y + 1),
	};

	if field::check_valid_pos(fd, fb, new_pl, pb) {
		let p = project(fd, fb, new_pl, pb);
		Some((new_pl, p))
	} else {
		None
	}
}
