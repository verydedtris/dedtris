use log::error;
use rlua::prelude::LuaError;

pub struct Error;

trait Constructable
{
	fn new() -> Self;
}

macro_rules! create_error {
	($type:tt, $msg:expr) => {
		pub struct $type;

		impl Constructable for $type
		{
			fn new() -> Self
			{
				Self {}
			}
		}

		impl From<$type> for Error
		{
			fn from(_: $type) -> Self
			{
				error!($msg);
				Self {}
			}
		}
	};
}

create_error!(
	LoadDefaultLuaError,
	"Default lua theme could not be loaded."
);

create_error!(
	InitGameNotFoundLuaError,
	"The function \"init_game\" has not been found."
);

create_error!(
	InitGameErrorLuaError,
	"The function \"init_game\" has a internal error."
);

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

// -----------------------------------------------------------------------------
// Result Conversion
// -----------------------------------------------------------------------------

trait ConvError<T, E>
{
	fn to_err<O>(self) -> Result<T, O>
	where
		O: Constructable;
}

impl<T, E> ConvError<T, E> for Result<T, E>
{
	fn to_err<O>(self) -> Result<T, O>
	where
		O: Constructable,
	{
		self.map_err(|_| O::new())
	}
}
