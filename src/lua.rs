use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use log::{error, info};

use crate::error::PError;
use crate::{err, propagate};

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

pub fn exec_file(ctx: &rlua::Context, path: &Path) -> Result<(), PError>
{
	let mut file = propagate!(File::open(path));

	let mut buffer = Vec::new();
	let s = propagate!(file.read_to_end(&mut buffer));

    info!("Loaded file \"{}\" with size {} Bytes.", path.display(), s);

	propagate!(ctx.load(&buffer).exec());

	Ok(())
}
