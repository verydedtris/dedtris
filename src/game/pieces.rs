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

#[derive(Default)]
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
}

pub fn rotate(pp: &mut PlayerPiece, field: &Field) -> bool
{
	let p = pp.piece.rotate();
	let v = field.check_valid_pos(pp.pos, &p);

	if v {
		pp.piece.blocks = p;
	}

	v
}

pub fn move_piece(pp: &mut PlayerPiece, field: &Field, d: Direction) -> bool
{
	let p = match d {
		Direction::LEFT => (pp.pos.0 - 1, pp.pos.1),
		Direction::RIGHT => (pp.pos.0 + 1, pp.pos.1),
		Direction::DOWN => (pp.pos.0, pp.pos.1 + 1),
	};

	let v = field.check_valid_pos(p, &pp.piece.blocks);

	if v {
		pp.pos = p;
	}

	v
}
