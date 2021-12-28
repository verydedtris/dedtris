use super::field::{self, Field};
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
	pub projection: i32,
	pub piece: gen::Piece,
}

impl PlayerPiece
{
	pub fn new(field: &Field, pos: (i32, i32), piece: gen::Piece) -> Option<Self>
	{
		if field::check_valid(field, &piece.blocks) {
			let projection = project(pos, &piece, field);
			Some(Self {
				pos,
				piece,
				projection,
			})
		} else {
			None
		}
	}
}

fn project(pos: (i32, i32), p: &gen::Piece, field: &Field) -> i32
{
	let mut y = pos.1 + 1;

	while field::check_valid_pos(field, (pos.0, y), &p.blocks) {
		y += 1;
	}

	y - 1
}

pub fn drop(pp: &mut PlayerPiece)
{
    pp.pos.1 = pp.projection;
}

pub fn rotate(pp: &mut PlayerPiece, field: &Field) -> bool
{
	let p = pp.piece.rotate();
	let v = field::check_valid_pos(field, pp.pos, &p);

	if v {
		pp.piece.blocks = p;
        pp.projection = project(pp.pos, &pp.piece, field);
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

	let v = field::check_valid_pos(field, p, &pp.piece.blocks);

	if v {
		pp.pos = p;
        pp.projection = project(pp.pos, &pp.piece, field);
	}

	v
}
