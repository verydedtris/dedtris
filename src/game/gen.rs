use sdl2::pixels::Color;
use sdl2::rect::Point;

use crate::{err, propagate};
use crate::error::PError;

use super::theme::parse_pattern;

// -----------------------------------------------------------------------------
// Piece
// -----------------------------------------------------------------------------

#[derive(Default)]
pub struct Piece
{
	pub dim: usize,
	pub colors: Vec<Color>,
	pub blocks: Vec<Point>,
}

// -----------------------------------------------------------------------------
// Piece generator
// -----------------------------------------------------------------------------

pub fn spawn_piece(sp: &rlua::Function) -> Result<Piece, PError>
{
    let t = propagate!(sp.call::<_, rlua::Table>(()), "Function \"spawn_piece\"");

    let (dim, colors, blocks) = parse_pattern(t)?;
    let p = Piece { dim, colors, blocks };

    Ok(p)
}

