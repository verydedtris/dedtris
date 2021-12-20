use bitvec::prelude::BitVec;
use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::{Error, Reader};
use sdl2::pixels::Color;
use std::fs::File;
use std::io::BufReader;
use std::num::ParseIntError;
use std::path::Path;

// -----------------------------------------------------------------------------
// Data
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Pattern
{
	pub dim: usize,
	pub colors: Vec<Color>,
	pub template: BitVec,
}

impl Pattern
{
	fn new(dim: usize, template: BitVec) -> Self
	{
		Pattern {
			dim,
			colors: vec![Color::RED; 4],
			template,
		}
	}
}

#[derive(Debug)]
pub struct Theme
{
	pub bg_color: Color,

	pub field_bg_color: Color,
	pub field_edge_color: Color,

	pub field_dim: (usize, usize),

	pub patterns: Vec<Pattern>,
}

impl Theme
{
	fn new() -> Theme
	{
		Theme {
			bg_color: Color::WHITE,
			field_bg_color: Color::BLACK,
			field_edge_color: Color::GRAY,
			field_dim: (0, 0),
			patterns: Vec::new(),
		}
	}

	fn from_data(patterns: Vec<Pattern>, field_dim: (usize, usize)) -> Theme
	{
		Theme {
			bg_color: Color::WHITE,
			field_bg_color: Color::BLACK,
			field_edge_color: Color::GRAY,
			field_dim,
			patterns,
		}
	}
}

// -----------------------------------------------------------------------------
// Error
// -----------------------------------------------------------------------------

struct ParseError
{
	err: String,
}

impl std::fmt::Debug for ParseError
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "{}", self.err)
	}
}

impl ParseError
{
	fn from_str(s: &str) -> Self
	{
		ParseError { err: s.to_string() }
	}

	fn from_pos(r: &Reader<BufReader<File>>, e: &Error) -> Self
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

fn load_xml(path: &Path) -> Result<Reader<BufReader<File>>, ParseError>
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

fn parse_block<'a>(
	r: &'a mut Reader<BufReader<File>>,
	name_list: &[&[u8]],
) -> Result<Option<(usize, Attributes<'a>)>, ParseError>
{
	let mut buf = Vec::new();

	loop {
		match r.read_event(&mut buf) {
			Ok(Event::Eof) => break,
			Ok(Event::Start(ref e)) => {
				if let Some(x) = name_list.iter().position(|x| *x == e.name()) {
					return Ok(Some((x, e.attributes())));
				}
			}

			Err(e) => return Err(ParseError::from_pos(r, &e)),
			_ => (),
		}

		buf.clear();
	}

	Ok(None)
}

fn parse_attrib<'a>(
	r: &'a mut Reader<BufReader<File>>,
	e: Attributes<'a>,
) -> Result<Vec<(&'a [u8], String)>, ParseError>
{
	let res = Vec::new();

	for attrib in e {
		let a = attrib?;
		let v = a.unescape_and_decode_value(r)?;

		res.push((a.key, v));
	}

	Ok(res)
}

fn parse_text(r: &mut Reader<BufReader<File>>) -> Result<String, ParseError>
{
	let mut buf = Vec::new();

	match r.read_event(&mut buf) {
		Ok(Event::Text(ref e)) => Ok(e.unescape_and_decode(r)?),

		Err(e) => Err(ParseError::from_pos(r, &e)),
		_ => Err(ParseError::from_str("Expected text.")),
	}
}

// -----------------------------------------------------------------------------
// Theme Parser
// -----------------------------------------------------------------------------

impl Theme
{
	pub fn load(path: &Path) -> Result<Theme, ParseError>
	{
		let mut reader = load_xml(path)?;
		Ok(parse_root(&mut reader)?)
	}
}

fn parse_root(r: &mut Reader<BufReader<File>>) -> Result<Theme, ParseError>
{
	while let Some(i) = parse_block(r, &[b"tetris"])? {
		match i.0 {
			0 => return parse_tetris(r, i.1),
			_ => (),
		}
	}

	Err(ParseError::from_str("No tetris root found."))
}

fn parse_tetris(r: &mut Reader<BufReader<File>>, e: Attributes) -> Result<Theme, ParseError>
{
	let mut dim = (0usize, 0usize);

	for a in parse_attrib(r, e)? {
		match a.0 {
			b"w" => dim.0 = a.1.parse()?,
			b"h" => dim.1 = a.1.parse()?,
		}
	}

	if dim == (0, 0) {
		return Err(ParseError::from_str("Field width or height can't be 0."));
	}

	let mut patterns: Vec<_> = Vec::new();

	while let Some(i) = parse_block(r, &[b"piece"])? {
		match i.0 {
			0 => {
				patterns.push(parse_piece(r, i.1)?);
			}
			_ => (),
		}
	}

	Ok(Theme::from_data(patterns, dim))
}

fn parse_piece(r: &mut Reader<BufReader<File>>, e: Attributes) -> Result<Pattern, ParseError>
{
	let mut dim = 0usize;

	for a in parse_attrib(r, e)? {
		match a.0 {
			b"r" => dim = a.1.parse()?,
		}
	}

	if dim == 0 {
		return Err(ParseError::from_str(
			"Pieces can't have a 0 width or a piece has a missing width.",
		));
	}

	let field = parse_template(dim, parse_text(r)?)?;

	Ok(Pattern::new(dim, field))
}

fn parse_template(s: usize, mut data: String) -> Result<BitVec, ParseError>
{
	data.retain(|c| !c.is_whitespace());

	if s * s != data.len() {
		return Err(ParseError::from_str(
			format!(
				"A piece size doesn't match it's given size. is: {}, should be: {}.",
				data.len().to_string(),
				(s * s).to_string()
			)
			.as_str(),
		));
	}

	let mut field = BitVec::with_capacity(s * s);

	for i in data.char_indices() {
		match i.1 {
			'1' => field.set(i.0, true),
			'0' => field.set(i.0, false),
			_ => return Err(ParseError::from_str("Characters must be 0's or 1's.")),
		}
	}

	Ok(field)
}
