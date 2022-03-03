use std::ffi::c_void;

use log::info;
use sdl2::render::WindowCanvas;

use crate::TetrisState;
use crate::error::Error;
use crate::lua::find_function;

pub struct StateData<'a, 'b, 'd, 'e>
{
    pub canvas: &'e mut WindowCanvas,
    pub game: &'d mut TetrisState<'a, 'b>,
}

pub fn call_lua<'a>(
	name: &str,
	state: &mut TetrisState<'_, 'a>,
	canvas: &mut WindowCanvas,
) -> Result<rlua::Table<'a>, Error>
{
	info!("Querying \"{}\".", name);

	let ctx = &state.lua_ctx;
	let g = ctx.globals();

	let mut data = StateData {
		game: state,
		canvas,
	};

	let ptr = &mut data as *mut _ as *mut c_void;
	Ok(find_function(&g, name)?.call::<_, rlua::Table>(rlua::LightUserData { 0: ptr })?)
}

