use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use log::error;

pub use self::theme::Theme;
pub use self::theme::Pattern;

mod theme;

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

type Result<T> = core::result::Result<T, Error>;

pub struct Lua
{
	runtime: rlua::Lua,
}

impl Lua
{
	pub fn new(p: &Path) -> Result<Self>
	{
		let runtime = rlua::Lua::new();

		load_file(&runtime, p)?;

		Ok(Self { runtime })
	}
}

impl Lua
{
	pub fn get_theme(&self) -> Result<Theme>
	{
		theme::load(&self.runtime)
	}
}

fn load_file(l: &rlua::Lua, p: &Path) -> Result<()>
{
	let mut file = File::open(p)?;

	let mut buffer = Vec::new();
	file.read_to_end(&mut buffer)?;

	l.context(|ctx| ctx.load(&buffer).exec())?;

	Ok(())
}
