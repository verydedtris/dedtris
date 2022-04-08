use std::{fs::File, io::prelude::*, path::Path};

use log::*;

use crate::{err, error::Error};

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

pub fn find_function<'a>(table: &rlua::Table<'a>, name: &str) -> Result<rlua::Function<'a>, Error>
{
	err!(
		table.get::<_, rlua::Function>(name),
		"Function \"{}\" not found or is not a function.",
		name
	)
}

pub fn find_int(table: &rlua::Table, name: &str) -> Result<i64, Error>
{
	err!(
		table.get::<_, rlua::Integer>(name),
		"Integer \"{}\" not found or is not a integer.",
		name
	)
}

pub fn find_string<'a>(table: &rlua::Table<'a>, name: &str) -> Result<rlua::String<'a>, Error>
{
	err!(
		table.get::<_, rlua::String>(name),
		"String \"{}\" not found or is not a string.",
		name
	)
}

pub fn find_table<'a>(table: &rlua::Table<'a>, name: &str) -> Result<rlua::Table<'a>, Error>
{
	err!(
		table.get::<_, rlua::Table>(name),
		"Table \"{}\" not found or is not a table.",
		name
	)
}

pub fn find_bool<'a>(table: &rlua::Table<'a>, name: &str) -> Result<bool, Error>
{
	err!(
		table.get::<_, bool>(name),
		"Boolean \"{}\" not found or is not a boolean.",
		name
	)
}
