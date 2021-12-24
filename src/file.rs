use quick_xml::{Error, Reader};
use std::fs::File;
use std::io::BufReader;
use std::num::ParseIntError;
use std::path::Path;

pub type Parser = Reader<BufReader<File>>;

// -----------------------------------------------------------------------------
// Error
// -----------------------------------------------------------------------------

pub struct ParseError
{
	err: String,
}

impl std::fmt::Display for ParseError
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "{}", self.err)
	}
}

impl ParseError
{
	pub fn from_str(s: &str) -> Self
	{
		ParseError { err: s.to_string() }
	}

	pub fn from_pos(r: &Reader<BufReader<File>>, e: &Error) -> Self
	{
		ParseError::from_str(format!("Error at position {}: {:?}", r.buffer_position(), e).as_str())
	}
}

impl From<Error> for ParseError
{
	fn from(e: Error) -> Self
	{
		ParseError { err: e.to_string() }
	}
}

impl From<ParseIntError> for ParseError
{
	fn from(e: ParseIntError) -> Self
	{
		ParseError { err: e.to_string() }
	}
}

// -----------------------------------------------------------------------------
// XML Parser
// -----------------------------------------------------------------------------

pub fn load_xml(path: &Path) -> Result<Reader<BufReader<File>>, ParseError>
{
	if let Ok(reader) = Reader::from_file(path) {
		Ok(reader)
	} else {
		Err(ParseError::from_str(
			format!(
				"Couldn't open file \"{}\".",
				path.as_os_str().to_str().unwrap()
			)
			.as_str(),
		))
	}
}
