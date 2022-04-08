use log::error;

pub struct Error;

impl std::fmt::Debug for Error
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "An unrecoverable error has occured.")
	}
}

impl<T: std::fmt::Display> From<T> for Error
{
	fn from(e: T) -> Self
	{
		error!("{}", e);
		Self {}
	}
}

#[macro_export]
macro_rules! err {
    ($x:expr, $msg:expr, $($param:expr),+) => {
        $x.map_err(|_| { error!($msg, $($param,)+); return Error {} })
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
			},
		}
	};

	($x:expr) => {
		match $x {
			Ok(x) => x,
			Err(e) => {
				error!("{}", e);
				return;
			},
		}
	};
}
