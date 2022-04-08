use std::{convert::TryFrom, ffi::c_void, time::Duration};

use log::info;
use rlua::prelude::*;
use sdl2::{pixels::Color, rect::Point};

use super::{Framework, Piece, TetrisState};
use crate::{error::Error, lua::*};

// -----------------------------------------------------------------------------
// Lua initialization
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Profile
{
	pub bg_color: Color,

	pub field_bg_color:   Color,
	pub field_edge_color: Color,

	pub field_dim: (u32, u32),

	pub start_piece: Piece,

	pub piece_view_size:    usize,
	pub piece_hold_enabled: bool,

	pub piece_tick: Duration,
}

// -----------------------------------------------------------------------------
// Block Parsing
// -----------------------------------------------------------------------------

pub fn load<'a, 'b>(ctx: &'b rlua::Context<'a>) -> Result<Profile, Error>
{
	info!("Evaluating profile.");

	let g = ctx.globals();

	let init = find_function(&g, "init_game")?.call::<_, rlua::Table>(())?;

	let width = u32::try_from(find_int(&init, "width")?)?;
	let height = u32::try_from(find_int(&init, "height")?)?;

	let piece_tick = if let Ok(t) = init.get::<_, LuaInteger>("piece_tick") {
		Duration::from_millis(u64::try_from(t)?)
	} else {
		Duration::from_secs(3_155_760_000) // 100 Years
	};

	let start_piece = parse_pattern(find_table(&init, "start_piece")?)?;

	let piece_view_size = if let Ok(s) =
		init.get::<_, LuaTable>("piece_view").and_then(|t| t.get::<_, LuaInteger>("size"))
	{
		usize::try_from(s)?
	} else {
		0
	};

	let piece_hold_enabled = init
		.get::<_, LuaTable>("piece_hold")
		.and_then(|t| t.get::<_, bool>("enabled"))
		.unwrap_or(false);

	Ok(Profile {
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

// -----------------------------------------------------------------------------
// Lua Functions
// -----------------------------------------------------------------------------

pub struct StateData<'a, 'b, 'c, 'd, 'f, 'g, 'h, 'i>
{
	pub game: &'a mut TetrisState,
	pub fw:   &'b Framework<'c, 'd, 'f, 'g, 'h, 'i>,
}

pub fn call_lua<'a, T>(
	name: &str, state: &mut TetrisState, fw: &Framework<'_, '_, '_, '_, '_, 'a>,
) -> Result<T, Error>
where
	T: rlua::FromLuaMulti<'a>,
{
	info!("Querying \"{}\".", name);

	let ctx = &fw.lua;
	let g = ctx.globals();

	let mut data = StateData { game: state, fw };

	let ptr = &mut data as *mut _ as *mut c_void;
	Ok(find_function(&g, name)?.call::<_, T>(rlua::LightUserData { 0: ptr })?)
}

pub fn load_defaults(ctx: &rlua::Context) -> Result<(), Error>
{
	let solve_field = ctx.create_function(|_, data: rlua::LightUserData| {
		let StateData { game, .. }: &mut StateData = unsafe { &mut *(data.0 as *mut StateData) };

		let v = game.clear_lines();

		Ok(v)
	})?;

	let exit_game = ctx.create_function(|_, data: rlua::LightUserData| {
		let StateData { game, .. }: &mut StateData = unsafe { &mut *(data.0 as *mut StateData) };

		game.exit = true;

		Ok(())
	})?;

	let g = ctx.globals();
	g.set("_solveField", solve_field)?;
	g.set("_finishGame", exit_game)?;

	Ok(())
}
