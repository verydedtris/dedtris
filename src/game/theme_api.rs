use std::ffi::c_void;

use log::info;

use crate::error::Error;
use crate::lua::find_function;

use super::{Framework, TetrisState};

pub struct StateData<'a, 'b, 'c, 'd, 'f, 'g, 'h, 'i>
{
	pub game: &'a mut TetrisState,
	pub fw: &'b Framework<'c, 'd, 'f, 'g, 'h, 'i>,
}

pub fn call_lua<'a, T>(
	name: &str,
	state: &mut TetrisState,
	fw: &Framework<'_, '_, '_, '_, '_, 'a>,
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
