use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use log::info;

use crate::error::Error;
use crate::err;

// -----------------------------------------------------------------------------
// Lua routines
// -----------------------------------------------------------------------------

pub fn exec_file(ctx: &rlua::Context, path: &Path) -> Result<(), Error>
{
	let mut file = File::open(path)?;

	let mut buffer = Vec::new();
	let s = file.read_to_end(&mut buffer)?;

	info!("Loaded file \"{}\" with size {} Bytes.", path.display(), s);

	ctx.load(&buffer).exec()?;

	Ok(())
}

pub fn find_function<'a>(table: &rlua::Table<'a>, name: &str)
	-> Result<rlua::Function<'a>, Error>
{
	Ok(err!(
		table.get::<_, rlua::Function>(name),
		"Function \"{}\" not found.",
		name
	))
}

pub fn find_int(table: &rlua::Table, name: &str) -> Result<i64, Error>
{
	Ok(err!(
		table.get::<_, rlua::Integer>(name),
		"Integer \"{}\" not found.",
		name
	))
}

pub fn find_string<'a>(table: &rlua::Table<'a>, name: &str) -> Result<rlua::String<'a>, Error>
{
	Ok(err!(
		table.get::<_, rlua::String>(name),
		"String \"{}\" not found.",
		name
	))
}

pub fn find_table<'a>(table: &rlua::Table<'a>, name: &str) -> Result<rlua::Table<'a>, Error>
{
	Ok(err!(
		table.get::<_, rlua::Table>(name),
		"Table \"{}\" not found.",
		name
	))
}
