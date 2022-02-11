use std::fmt::Display;

use log::error;
use rlua::prelude::LuaError;

pub trait Constructable
{
	fn new() -> Self;
}

pub struct PError
{
	pub msg: String,
}

impl Display for PError
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "{}", self.msg)
	}
}

impl From<&str> for PError
{
	fn from(e: &str) -> Self
	{
		Self { msg: e.to_string() }
	}
}

impl From<String> for PError
{
	fn from(e: String) -> Self
	{
		Self { msg: e }
	}
}

#[macro_export]
macro_rules! err {
	($x:expr, $msg:expr) => {
		if let Ok(x) = $x {
			x
		} else {
			return Err(PError::from($msg));
		}
	};
}

#[macro_export]
macro_rules! propagate {
    ($x:expr, $msg:expr, $($param:expr),+) => {
		match $x {
			Ok(x) => x,
			Err(e) => return Err(PError::from(format!("{}: {}", format!($msg, $($param,)+), e))),
		}
    };

	($x:expr, $msg:expr) => {
		match $x {
			Ok(x) => x,
			Err(e) => return Err(PError::from(format!("{}: {}", $msg, e))),
		}
	};

	($x:expr) => {
		match $x {
			Ok(x) => x,
			Err(e) => return Err(PError::from(format!("{}", e))),
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
