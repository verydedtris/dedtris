use std::ops::Index;

use bitvec::prelude::BitVec;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use crate::error::Error;

use super::theme::{self, parse_pattern};

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

pub fn spawn_piece(sp: &rlua::Function) -> Result<Piece, Error>
{
    let t = sp.call::<_, rlua::Table>(())?;

    let (dim, colors, blocks) = parse_pattern(t)?;
    let p = Piece { dim, colors, blocks };

    Ok(p)
}

