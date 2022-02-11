use std::num::TryFromIntError;

use log::error;
use rlua::prelude::LuaError;

pub struct Error;

impl From<&str> for Error
{
	fn from(e: &str) -> Self
	{
		error!("{}", e);
		Self {}
	}
}

impl From<String> for Error
{
	fn from(e: String) -> Self
	{
		error!("{}", e);
		Self {}
	}
}

impl From<LuaError> for Error
{
	fn from(e: LuaError) -> Self
	{
		error!("{}", e);
		Self {}
	}
}

impl From<TryFromIntError> for Error
{
	fn from(e: TryFromIntError) -> Self
	{
		error!("{}", e);
		Self {}
	}
}

impl From<std::io::Error> for Error
{
	fn from(e: std::io::Error) -> Self
	{
		error!("{}", e);
		Self {}
	}
}

#[macro_export]
macro_rules! err {
    ($x:expr, $msg:expr, $($param:expr),+) => {
		match $x {
			Ok(x) => x,
			Err(_) => return Err(Error::from(format!("{}", format!($msg, $($param,)+)))),
		}
    };

	($x:expr, $msg:expr) => {
		if let Ok(x) = $x {
			x
		} else {
			return Err(Error::from($msg));
		}
	};
}

#[macro_export]
macro_rules! end {
	($x:expr, $msg:expr) => {
		match $x {
			Ok(x) => x,
			Err(e) => {
				error!("{}: {}", $msg, e);
				return;
			}
		}
	};

	($x:expr) => {
		match $x {
			Ok(x) => x,
			Err(e) => {
				error!("{}", e);
				return;
			}
		}
	};
}
