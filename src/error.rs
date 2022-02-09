use rlua::prelude::LuaError;
use log::{error, info};

pub struct Error {}

impl From<std::io::Error> for Error
{
	fn from(e: std::io::Error) -> Self
	{
		error!("Theme load error: {}", e);
		Self {}
	}
}

impl From<LuaError> for Error
{
	fn from(e: LuaError) -> Self
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

impl From<std::num::TryFromIntError> for Error
{
    fn from(e: std::num::TryFromIntError) -> Self
    {
        error!("{}", e);
        Self {}
    }
}
