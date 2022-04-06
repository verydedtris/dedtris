use std::convert::TryFrom;

use log::info;
use rlua::prelude::*;
use sdl2::{pixels::Color, rect::Point};

use super::Piece;
use crate::{error::*, lua::*};

// -----------------------------------------------------------------------------
// Parse Structures
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Theme
{
	pub bg_color: Color,

	pub field_bg_color:   Color,
	pub field_edge_color: Color,

	pub field_dim: (u32, u32),

	pub start_piece: Piece,

	pub piece_view_size:    usize,
	pub piece_hold_enabled: bool,

	pub piece_tick: u32,
}

// -----------------------------------------------------------------------------
// Block Parsing
// -----------------------------------------------------------------------------

pub fn load<'a, 'b>(ctx: &'b rlua::Context<'a>) -> Result<Theme, Error>
{
	info!("Evaluating theme.");

	let g = ctx.globals();

	let init = find_function(&g, "init_game")?.call::<_, rlua::Table>(())?;

	let width = u32::try_from(init.get::<_, LuaInteger>("width")?)?;
	let height = u32::try_from(init.get::<_, LuaInteger>("height")?)?;

	let piece_tick = u32::try_from(init.get::<_, LuaInteger>("piece_tick")?)?;

	let start_piece = parse_pattern(init.get::<_, LuaTable>("start_piece")?)?;

	let piece_view_size = if let Ok(s) =
		init.get::<_, LuaTable>("piece_view").and_then(|t| t.get::<_, LuaInteger>("size"))
	{
		usize::try_from(s)?
	} else {
		0
	};

	let piece_hold_enabled = init
		.get::<_, LuaTable>("piece_hold")
		.and_then(|t| t.get::<_, LuaInteger>("enabled"))
		.map(|i| i != 0)
		.unwrap_or(false);

	Ok(Theme {
		bg_color: Color::WHITE,
		field_bg_color: Color::BLACK,
		field_edge_color: Color::GRAY,
		field_dim: (width, height),
		piece_tick,
		start_piece,
		piece_view_size,
		piece_hold_enabled,
	})
}

// -----------------------------------------------------------------------------
// Parsing Functions
// -----------------------------------------------------------------------------

pub fn parse_pattern(table: LuaTable) -> Result<Piece, Error>
{
	let dim = u32::try_from(find_int(&table, "size")?)?;

	let blocks = parse_piece_body(find_string(&table, "template")?, dim)?;

	let color = parse_piece_color(find_table(&table, "color")?)?;
	let colors = vec![color; blocks.len()];

	Ok(Piece {
		dim,
		blocks,
		colors,
	})
}

fn parse_piece_body(data: LuaString, pd: u32) -> Result<Vec<Point>, Error>
{
	let ps = pd * pd;

	let mut field = Vec::with_capacity(ps as usize);
	let mut blocks = 0;

	for (i, c) in data.as_bytes().iter().enumerate() {
		match c {
			b'1' => {
				field.push(Point::new(
					(i % pd as usize) as i32,
					(i / pd as usize) as i32,
				));
				blocks += 1;
			},
			b'0' => blocks += 1,
			b'\n' | b' ' | b'\r' | b'\t' => {},
			_ => {
				return Err(Error::from("Characters must be 0's or 1's."));
			},
		}
	}

	if ps != blocks {
		return Err(Error::from(format!(
			"A piece size doesn't match it's given size. is: {}, should be: {}.",
			field.len(),
			ps
		)));
	}

	Ok(field)
}

fn parse_piece_color(data: LuaTable) -> Result<Color, Error>
{
	let mut rgba = [0u8; 4];

	for (c, y) in ["r", "g", "b", "a"].iter().zip(rgba.iter_mut()) {
		*y = u8::try_from(find_int(&data, *c)?)?;
	}

	Ok(Color::RGBA(rgba[0], rgba[1], rgba[2], rgba[3]))
}
