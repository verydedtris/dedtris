use std::ffi::c_void;

use log::info;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use crate::error::*;
use crate::lua::*;

use super::theme::parse_pattern;
use super::TetrisState;

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

pub fn spawn_piece(state: &mut TetrisState) -> Result<Piece, Error>
{
	info!("Querying \"spawn_piece\".");

	let ctx = &state.lua_ctx;
	let g = ctx.globals();

	let ptr = state as *mut _ as *mut c_void;
	let t =
		find_function(&g, "spawn_piece")?.call::<_, rlua::Table>(rlua::LightUserData { 0: ptr })?;

	let (dim, colors, blocks) = parse_pattern(t)?;
	let p = Piece {
		dim,
		colors,
		blocks,
	};

	Ok(p)
}
