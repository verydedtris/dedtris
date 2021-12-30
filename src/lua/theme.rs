use bitvec::prelude::BitVec;
use rlua::prelude::*;
use rlua::Function;
use sdl2::pixels::Color;

use std::convert::TryFrom;

use super::Error;

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

impl Pattern
{
	pub fn new(dim: usize, colors: Vec<Color>, template: BitVec) -> Self
	{
		Self {
			dim,
			colors,
			template,
		}
	}
}

#[derive(Debug)]
pub struct Theme
{
	pub bg_color: Color,

	pub field_bg_color: Color,
	pub field_edge_color: Color,

	pub field_dim: (usize, usize),

	pub patterns: Vec<Pattern>,
}

impl Default for Theme
{
	fn default() -> Self
	{
		Theme {
			bg_color: Color::WHITE,
			field_bg_color: Color::BLACK,
			field_edge_color: Color::GRAY,
			field_dim: (0, 0),
			patterns: Vec::new(),
		}
	}
}

impl Theme
{
	fn from_data(patterns: Vec<Pattern>, field_dim: (usize, usize)) -> Theme
	{
		Theme {
			bg_color: Color::WHITE,
			field_bg_color: Color::BLACK,
			field_edge_color: Color::GRAY,
			field_dim,
			patterns,
		}
	}
}

// -----------------------------------------------------------------------------
// Block Parsing
// -----------------------------------------------------------------------------

pub fn load(r: &rlua::Lua) -> super::Result<Theme>
{
	r.context(|ctx| {
		let g = ctx.globals();

		let f: Function = g.get("load_config")?;
		let res = f.call::<_, LuaTable>(())?;

        parse_theme(res)
	})
}

// -----------------------------------------------------------------------------
// Parsing Functions
// -----------------------------------------------------------------------------

fn parse_theme(table: LuaTable) -> super::Result<Theme>
{
	let width = table.get::<_, LuaInteger>("width")? as usize;
	let height = table.get::<_, LuaInteger>("height")? as usize;

	let pieces = table.get::<_, rlua::Table>("pieces")?;
	let pieces = {
		let mut patterns = Vec::new();

		for p in pieces.sequence_values::<rlua::Table>() {
			let p = parse_pattern(p?)?;
			patterns.push(p);
		}

		patterns
	};

	Ok(Theme::from_data(pieces, (width, height)))
}

fn parse_pattern(table: LuaTable) -> super::Result<Pattern>
{
	let size = table.get::<_, LuaInteger>("size")? as usize;

	let template = table.get::<_, LuaString>("template")?;
	let (template, blocks) = parse_piece_body(template, size)?;

	let color = table.get::<_, LuaTable>("color")?;
	let color = parse_piece_color(color)?;
	let color = vec![color; blocks];

	Ok(Pattern::new(size, color, template))
}

fn parse_piece_body(data: LuaString, pd: usize) -> super::Result<(BitVec, usize)>
{
	let ps = pd * pd;

	let mut field = BitVec::with_capacity(ps);
	let mut blocks = 0;

	for i in data.as_bytes() {
		match i {
			b'1' => {
				field.push(true);
				blocks += 1;
			}
			b'0' => field.push(false),
			b'\n' | b' ' | b'\r' | b'\t' => {}
			_ => {
				return Err(Error::from("Characters must be 0's or 1's."));
			}
		}
	}

	if ps != field.len() {
		return Err(Error::from(
			format!(
				"A piece size doesn't match it's given size. is: {}, should be: {}.",
				field.len(),
				ps
			)
			.as_str(),
		));
	}

	Ok((field, blocks))
}

fn parse_piece_color(data: LuaTable) -> super::Result<Color>
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
