use log::info;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use crate::error::*;
use crate::lua::*;

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

pub fn spawn_piece(ctx: &rlua::Context) -> Result<Piece, Error>
{
    info!("Querying \"spawn_piece\".");

    let g = ctx.globals();
	let t = find_function(&g, "spawn_piece")?.call::<_, rlua::Table>(())?;

	let (dim, colors, blocks) = parse_pattern(t)?;
	let p = Piece {
		dim,
		colors,
		blocks,
	};

	Ok(p)
}
