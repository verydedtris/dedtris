use bitmaps::Bitmap;
use quick_xml::events::{BytesStart, BytesText, Event};
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
	pub dim: (u32, u32),
	pub colors: Vec<Color>,
	pub rot: Vec<Bitmap<64>>,
}

impl Pattern
{
	fn new(dim: (u32, u32), rot: Vec<Bitmap<64>>) -> Self
	{
		Pattern {
			dim,
			colors: vec![Color::RED; 4],
			rot,
		}
	}
}

#[derive(Debug)]
pub struct Theme
{
	pub bg_color: Color,

	pub field_bg_color: Color,
	pub field_edge_color: Color,

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
			patterns: Vec::new(),
		}
	}

	fn from_patterns(patterns: Vec<Pattern>) -> Theme
	{
		Theme {
			bg_color: Color::WHITE,
			field_bg_color: Color::BLACK,
			field_edge_color: Color::GRAY,
			patterns,
		}
	}

	fn from_data(
		bg_color: Color,
		field_bg_color: Color,
		field_edge_color: Color,
		patterns: Vec<Pattern>,
	) -> Theme
	{
		Theme {
			bg_color,
			field_bg_color,
			field_edge_color,
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
// Theme Parser
// -----------------------------------------------------------------------------

impl Theme
{
	pub fn load(path: &Path) -> Option<Theme>
	{
		let mut reader = if let Ok(reader) = Reader::from_file(path) {
			reader
		} else {
			println!(
				"Couldn't open file \"{}\".",
				path.as_os_str().to_str().unwrap()
			);

			return None;
		};

		match parse_tetris(&mut reader) {
			Ok(t) => Some(t),
			Err(e) => {
				println!("Theme couldn't be parsed. {}", e.err);
				None
			}
		}
	}
}

fn parse_tetris(r: &mut Reader<BufReader<File>>) -> Result<Theme, ParseError>
{
	let mut patterns: Vec<_> = Vec::new();
	let mut buf = Vec::new();

	loop {
		match r.read_event(&mut buf) {
			Ok(Event::Eof) => break,
			Ok(Event::Start(ref e)) => {
				if let b"piece" = e.name() {
					patterns.push(parse_piece(r, e)?);
				}
			}

			Err(e) => return Err(ParseError::from_pos(r, &e)),
			_ => (),
		}

		buf.clear();
	}

	Ok(Theme::from_patterns(patterns))
}

fn parse_piece(r: &mut Reader<BufReader<File>>, e: &BytesStart) -> Result<Pattern, ParseError>
{
	let dim = parse_piece_size(r, &e)?;

	let mut buf = Vec::new();

	let rots = match r.read_event(&mut buf) {
		Ok(Event::Text(e)) => parse_template(r, dim, &e)?,

		Err(e) => return Err(ParseError::from_pos(r, &e)),
		_ => return Err(ParseError::from_str("Missing piece rotations")),
	};

	Ok(Pattern::new(dim, rots))
}

fn parse_piece_size(
	r: &mut Reader<BufReader<File>>,
	e: &BytesStart,
) -> Result<(u32, u32), ParseError>
{
	let mut w = 0;
	let mut h = 0;

	for attrib in e.attributes() {
		let a = attrib?;
		let v = a.unescape_and_decode_value(r)?;

		match a.key {
			b"w" => w = v.parse()?,
			b"h" => h = v.parse()?,
			_ => (),
		}
	}

	if w == 0 {
		return Err(ParseError::from_str("A piece has a missing width."));
	}

	if h == 0 {
		return Err(ParseError::from_str("A piece has a missing height."));
	}

	if w * h > 64 {
		return Err(ParseError::from_str("A piece size exceeds 64 slots."));
	}

	Ok((w, h))
}

fn parse_template(
	r: &mut Reader<BufReader<File>>,
	s: (u32, u32),
	t: &BytesText,
) -> Result<Vec<Bitmap<64>>, ParseError>
{
	let mut data = t.unescape_and_decode(r)?;
	data.retain(|c| !c.is_whitespace());

	let mut rotations: Vec<_> = Vec::new();

	for rot in data.split('#') {
		if s.0 * s.1 != rot.len() as u32 {
			return Err(ParseError::from_str(
				format!(
					"A piece size doesn't match it's given size. is: {}, should be: {}.",
					data.len().to_string(),
					(s.0 * s.1).to_string()
				)
				.as_str(),
			));
		}

		let mut field = Bitmap::<64>::new();

		for i in rot.char_indices() {
			match i.1 {
				'1' => drop(field.set(i.0, true)),
				'0' => (),
				_ => return Err(ParseError::from_str("Characters must be 0's or 1's.")),
			}
		}

		rotations.push(field);
	}

	Ok(rotations)
}
