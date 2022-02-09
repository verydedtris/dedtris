use bitvec::prelude::BitVec;
use rlua::prelude::*;
use rlua::Function;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use std::convert::TryFrom;
use std::ops::Index;

use crate::error::Error;

// -----------------------------------------------------------------------------
// Parse Structures
// -----------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Pattern
{
	pub dim: usize,
	pub colors: Vec<Color>,
	pub template: BitVec,
}

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

pub fn load<'a, 'b>(ctx: &'b rlua::Context<'a>) -> Result<Theme<'a>, Error>
{
	let g = ctx.globals();
	let init = g.get::<_, rlua::Function<'a>>("init_game")?.call::<_, rlua::Table>(())?;

	let width = usize::try_from(init.get::<_, LuaInteger>("width")?)?;
	let height = usize::try_from(init.get::<_, LuaInteger>("height")?)?;

	let game_logic: LogicTable<'a> = [g.get::<_, rlua::Function<'a>>("spawn_piece")?];

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

pub fn parse_pattern(table: LuaTable) -> Result<(usize, Vec<Color>, Vec<Point>), Error>
{
	let dim = table.get::<_, LuaInteger>("size")? as usize;

	let template = table.get::<_, LuaString>("template")?;
	let blocks = parse_piece_body(template, dim)?;

	let color = table.get::<_, LuaTable>("color")?;
	let color = parse_piece_color(color)?;
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
		return Err(Error::from(
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

fn parse_piece_color(data: LuaTable) -> Result<Color, Error>
{
	let r = data.get::<_, LuaInteger>("r")?;
	let g = data.get::<_, LuaInteger>("g")?;
	let b = data.get::<_, LuaInteger>("b")?;
	let a = data.get::<_, LuaInteger>("a")?;

	let (r, g, b, a) = if let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
		u8::try_from(r),
		u8::try_from(g),
		u8::try_from(b),
		u8::try_from(a),
	) {
		(r, g, b, a)
	} else {
		return Err(Error::from("Invald colors."));
	};

	Ok(Color::RGBA(r, g, b, a))
}
