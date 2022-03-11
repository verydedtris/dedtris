use log::info;
use sdl2::{pixels::Color, rect::Point};

use super::{field, gen, Size};

// -----------------------------------------------------------------------------
// Movable Piece
// -----------------------------------------------------------------------------

pub struct MoveablePieceComponent
{
	pub blocks:     Vec<Point>,
	pub colors:     Vec<Color>,
	pub dim:        u32,
	pub pos:        Point,
	pub projection: i32,
}

pub fn init() -> MoveablePieceComponent
{
	MoveablePieceComponent {
		blocks:     Vec::new(),
		colors:     Vec::new(),
		dim:        0,
		pos:        Point::new(0, 0),
		projection: 0,
	}
}

pub fn project(field_dim: Size, field_blocks: &[Point], pos: Point, blocks: &[Point]) -> i32
{
	let mut y = pos.y + 1;

	while field::check_valid_pos(field_dim, field_blocks, Point::new(pos.x, y), blocks) {
		y += 1;
	}

	y - 1
}
