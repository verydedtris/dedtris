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

pub struct SpawnedPiece
{
	pub piece: gen::Piece,
	pub pos:   Point,
	pub proj:  i32,
}

pub fn spawn_piece(
	field_blocks: &[Point], field_dim: Size, piece: gen::Piece,
) -> Option<SpawnedPiece>
{
	let pos = Point::new(((field_dim.0 - piece.dim) / 2) as i32, 0);

	if !field::check_valid_pos(field_dim, field_blocks, pos, &piece.blocks) {
		return None;
	}

	let proj = project(field_dim, field_blocks, pos, &piece.blocks);

	Some(SpawnedPiece { piece, proj, pos })
}
