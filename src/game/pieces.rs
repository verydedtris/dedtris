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

	pub dim: usize,
	pub colors: Vec<Color>,
	pub blocks: Vec<(i32, i32)>,
}

impl PlayerPiece
{
	pub fn new(field: &Field, pos: (i32, i32), piece: gen::Piece) -> Option<Self>
	{
		if field.check_valid(&piece.blocks) {
			Some(Self {
				pos,
				dim: piece.dim,
				colors: piece.colors,
				blocks: piece.blocks,
			})
		} else {
			None
		}
	}

	pub fn output_blocks(&mut self) -> (Vec<(i32, i32)>, Vec<Color>)
	{
		(
			self.blocks.iter().map(|b| (b.0 + self.pos.0, b.1 + self.pos.1)).collect(),
			std::mem::take(&mut self.colors),
		)
	}
}

impl PlayerPiece
{
	pub fn rotate(&mut self, field: &Field) -> bool
	{
		let p: Vec<(i32, i32)> =
			self.blocks.iter().map(|b| (self.dim as i32 - 1 - b.1, b.0)).collect();

		let v = field.check_valid_pos(self.pos, &p);

		if v {
			self.blocks = p;
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

		let v = field.check_valid_pos(p, &self.blocks);

		if v {
			self.pos = p;
		}

		v
	}
}
