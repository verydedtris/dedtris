use bitvec::prelude::BitVec;
use rlua::prelude::*;
use rlua::Function;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use std::convert::TryFrom;
use std::ops::Index;

use crate::err;
use crate::error::*;
use crate::propagate;

// -----------------------------------------------------------------------------
// Parse Structures
// -----------------------------------------------------------------------------

pub enum LogicIndex
{
	SpawnPiece,
	All,
}

pub type LogicTable<'a> = [rlua::Function<'a>; LogicIndex::All as usize];

impl<'a> Index<LogicIndex> for LogicTable<'a>
{
	type Output = rlua::Function<'a>;

	fn index(&self, index: LogicIndex) -> &Self::Output
	{
		&self[index as usize]
	}
}

#[derive(Debug)]
pub struct Theme<'a>
{
	pub bg_color: Color,

	pub field_bg_color: Color,
	pub field_edge_color: Color,

	pub field_dim: (usize, usize),

	pub game_logic: LogicTable<'a>,
}

// -----------------------------------------------------------------------------
// Block Parsing
// -----------------------------------------------------------------------------

pub fn load<'a, 'b>(ctx: &'b rlua::Context<'a>) -> Result<Theme<'a>, PError>
{
	let g = ctx.globals();

	let init = propagate!(
		g.get::<_, rlua::Function<'a>>("init_game"),
		"Function \"init_game\""
	);
	let init = propagate!(
		init.call::<_, rlua::Table>(()),
		"Function \"init_game\""
	);

	let width = propagate!(
		init.get::<_, LuaInteger>("width"),
		"Output field \"width\""
	);
	let width = err!(
		usize::try_from(width),
		"Output field \"width\" has a invalid value."
	);

	let height = propagate!(
		init.get::<_, LuaInteger>("height"),
		"Output field \"height\""
	);
	let height = err!(
		usize::try_from(height),
		"Output field \"height\" has a invalid value."
	);

	let game_logic: LogicTable<'a> = [propagate!(
		g.get::<_, rlua::Function<'a>>("spawn_piece"),
		"Function \"spawn_piece\""
	)];

	Ok(Theme {
		bg_color: Color::WHITE,
		field_bg_color: Color::BLACK,
		field_edge_color: Color::GRAY,
		field_dim: (width, height),
		game_logic,
	})
}

// -----------------------------------------------------------------------------
// Parsing Functions
// -----------------------------------------------------------------------------

pub fn parse_pattern(table: LuaTable) -> Result<(usize, Vec<Color>, Vec<Point>), PError>
{
	let dim = err!(
		table.get::<_, LuaInteger>("size"),
		"Field \"size\" is missing or is invalid."
	);
	let dim = err!(usize::try_from(dim), "Field \"size\" has a invalid value.");

	let template = err!(
		table.get::<_, LuaString>("template"),
		"Field \"template\" is missing."
	);
	let blocks = propagate!(parse_piece_body(template, dim), "Field \"template\" error");

	let color = err!(
		table.get::<_, LuaTable>("color"),
		"Field \"color\" not found or is invalid."
	);
	let color = propagate!(parse_piece_color(color), "Field \"color\" error");
	let colors = vec![color; blocks.len()];

	Ok((dim, colors, blocks))
}

fn parse_piece_body(data: LuaString, pd: usize) -> Result<Vec<Point>, PError>
{
	let ps = pd * pd;

	let mut field = Vec::with_capacity(ps);
	let mut blocks = 0;

	for (i, c) in data.as_bytes().iter().enumerate() {
		match c {
			b'1' => {
				field.push(Point::new((i % pd) as i32, (i / pd) as i32));
				blocks += 1;
			}
			b'0' => blocks += 1,
			b'\n' | b' ' | b'\r' | b'\t' => {}
			_ => {
				return Err(PError::from("Characters must be 0's or 1's."));
			}
		}
	}

	if ps != blocks {
		return Err(PError::from(
			format!(
				"A piece size doesn't match it's given size. is: {}, should be: {}.",
				field.len(),
				ps
			)
			.as_str(),
		));
	}

	Ok(field)
}

fn parse_piece_color(data: LuaTable) -> Result<Color, PError>
{
	let mut rgba = [0u8; 4];

	for (c, y) in ["r", "g", "b", "a"].iter().zip(rgba.iter_mut()) {
		let x = err!(data.get::<_, LuaInteger>(*c), "Missing value*s");
		let x = err!(u8::try_from(x), "Invalid value*s");

		*y = x;
	}

	Ok(Color::RGBA(rgba[0], rgba[1], rgba[2], rgba[3]))
}
