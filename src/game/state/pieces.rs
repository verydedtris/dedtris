use log::info;
use sdl2::{pixels::Color, rect::Point};

use super::{field, gen, Size};

// -----------------------------------------------------------------------------
// Movable Piece
// -----------------------------------------------------------------------------

pub fn project(field_dim: Size, field_blocks: &[Point], pos: Point, blocks: &[Point]) -> i32
{
	let mut y = pos.y + 1;

	while field::check_valid_pos(field_dim, field_blocks, Point::new(pos.x, y), blocks) {
		y += 1;
	}

	y - 1
}
