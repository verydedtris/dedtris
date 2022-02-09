use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use log::error;

// -----------------------------------------------------------------------------
// Error
// -----------------------------------------------------------------------------

pub struct Error {}

impl From<std::io::Error> for Error
{
	fn from(e: std::io::Error) -> Self
	{
		error!("Theme load error: {}", e);
		Self {}
	}
}

impl From<rlua::Error> for Error
{
	fn from(e: rlua::Error) -> Self
	{
		error!("Lua load error: {}", e);
		Self {}
	}
}

impl From<&str> for Error
{
	fn from(e: &str) -> Self
	{
		error!("{}", e);
		Self {}
	}
}

// -----------------------------------------------------------------------------
// Lua routines
// -----------------------------------------------------------------------------

pub fn exec_file(ctx: &rlua::Context, path: &Path) -> Result<(), Error>
{
	let mut file = File::open(path)?;

	let mut buffer = Vec::new();
	file.read_to_end(&mut buffer)?;

	ctx.load(&buffer).exec()?;

	Ok(())
}
