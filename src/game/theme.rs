use log::info;
use rlua::prelude::*;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use std::convert::TryFrom;
use std::ops::Index;

use crate::lua::*;
use crate::error::*;

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
pub struct Theme
{
	pub bg_color: Color,

	pub field_bg_color: Color,
	pub field_edge_color: Color,

	pub field_dim: (usize, usize),
}

// -----------------------------------------------------------------------------
// Block Parsing
// -----------------------------------------------------------------------------

pub fn load<'a, 'b>(ctx: &'b rlua::Context<'a>) -> Result<Theme, Error>
{
	info!("Evaluating theme.");

	let g = ctx.globals();

	let init = find_function(&g, "init_game")?.call::<_, rlua::Table>(())?;

	let width = usize::try_from(init.get::<_, LuaInteger>("width")?)?;
	let height = usize::try_from(init.get::<_, LuaInteger>("height")?)?;

	Ok(Theme {
		bg_color: Color::WHITE,
		field_bg_color: Color::BLACK,
		field_edge_color: Color::GRAY,
		field_dim: (width, height),
	})
}

// -----------------------------------------------------------------------------
// Parsing Functions
// -----------------------------------------------------------------------------

pub fn parse_pattern(table: LuaTable) -> Result<(usize, Vec<Color>, Vec<Point>), Error>
{
	let dim = usize::try_from(find_int(&table, "size")?)?;

	let blocks = parse_piece_body(find_string(&table, "template")?, dim)?;

	let color = parse_piece_color(find_table(&table, "color")?)?;
	let colors = vec![color; blocks.len()];

	Ok((dim, colors, blocks))
}

fn parse_piece_body(data: LuaString, pd: usize) -> Result<Vec<Point>, Error>
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
				return Err(Error::from("Characters must be 0's or 1's."));
			}
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
