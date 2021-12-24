use sdl2::pixels::Color;

use super::field::Field;
use super::gen;

// -----------------------------------------------------------------------------
// Movable Piece
// -----------------------------------------------------------------------------

pub enum Direction
{
	LEFT,
	RIGHT,
	DOWN,
}

pub struct PlayerPiece
{
	pub pos: (i32, i32),
	pub piece: gen::Piece,
}

impl PlayerPiece
{
	pub fn new(field: &Field, pos: (i32, i32), piece: gen::Piece) -> Option<Self>
	{
		if field.check_valid(&piece.blocks) {
			Some(Self { pos, piece })
		} else {
			None
		}
	}

	pub fn delta_blocks(&mut self) -> (Vec<(i32, i32)>, Vec<Color>)
	{
		(
			self.piece.move_delta(self.pos),
			std::mem::take(&mut self.piece.colors),
		)
	}
}

impl PlayerPiece
{
	pub fn rotate(&mut self, field: &Field) -> bool
	{
		let p = self.piece.rotate();
		let v = field.check_valid_pos(self.pos, &p);

		if v {
			self.piece.blocks = p;
		}

		v
	}

	pub fn move_piece(&mut self, field: &Field, d: Direction) -> bool
	{
		let p = match d {
			Direction::LEFT => (self.pos.0 - 1, self.pos.1),
			Direction::RIGHT => (self.pos.0 + 1, self.pos.1),
			Direction::DOWN => (self.pos.0, self.pos.1 + 1),
		};

		let v = field.check_valid_pos(p, &self.piece.blocks);

		if v {
			self.pos = p;
		}

		v
	}
}
