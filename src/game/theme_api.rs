use std::ffi::c_void;

use log::info;

use super::{Framework, TetrisState};
use crate::{error::Error, lua::find_function};

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
