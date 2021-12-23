use bitvec::prelude::BitVec;
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
	pub dim: usize,
	pub colors: Vec<Color>,
	pub template: BitVec,
}

impl Pattern
{
	fn new() -> Self
	{
		Pattern {
			dim: 0,
			colors: Vec::new(),
			template: BitVec::new(),
		}
	}

	fn from_dim(dim: usize) -> Self
	{
		Pattern {
			dim,
			colors: Vec::new(),
			template: BitVec::new(),
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

type Parser = Reader<BufReader<File>>;

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

enum XMLStates
{
	Theme,
	Piece,
	PieceBody(Pattern),
}

impl Theme
{
	pub fn load(path: &Path) -> Result<Theme, ParseError>
	{
		let mut reader = load_xml(path)?;
		let mut buffer = Vec::new();

		let mut dim: Option<(usize, usize)> = None;
		let mut patterns: Vec<Pattern> = Vec::new();

		let mut state = XMLStates::Theme;

		// Parse XML document

		loop {
			match reader.read_event(&mut buffer) {
				Ok(Event::Eof) => break,

				Ok(Event::Start(ref e)) => {
					state = match state {
						XMLStates::Theme => {
							println!("Found Theme tag");
							let (s, d) = parse_root(&mut reader, &e)?;
							dim = Some(d);
							s
						}

						XMLStates::Piece => {
							println!("Found Piece tag");
							parse_piece(&mut reader, &e)?.unwrap_or(state)
						}

						XMLStates::PieceBody(_) => {
							return Err(ParseError::from_str("Missing piece body."))
						}
					}
				}

				Ok(Event::Text(ref e)) => {
					println!("{:?}", e.unescape_and_decode(&reader)?);

					state = match state {
						XMLStates::PieceBody(p) => {
							println!("Found Piece body");
							let (s, p) = parse_piece_body(&mut reader, &e, p)?;
							patterns.push(p);
							s
						}

						_ => state,
					}
				}

				Err(e) => return Err(ParseError::from_pos(&reader, &e)),
				_ => (),
			}

			buffer.clear();
		}

		// Check validity

		let dim = if let Some(d) = dim {
			d
		} else {
			return Err(ParseError::from_str("Missing field size."));
		};

		if patterns.is_empty() {
			return Err(ParseError::from_str("There must be at least 1 pattern."));
		}

		Ok(Theme::from_data(patterns, dim))
	}
}

// -----------------------------------------------------------------------------
// Theme Parser
// -----------------------------------------------------------------------------

fn parse_root(
	r: &mut Parser,
	e: &BytesStart,
) -> Result<(XMLStates, (usize, usize)), ParseError>
{
	if e.name() != b"theme" {
		return Err(ParseError::from_str("No tetris root found."));
	}

	let mut dim = (0usize, 0usize);

	for a in e.attributes() {
		let x = a?;
		let v = x.unescape_and_decode_value(r)?;

		match x.key {
			b"w" => dim.0 = v.parse()?,
			b"h" => dim.1 = v.parse()?,
			_ => (),
		}
	}

	if dim == (0, 0) {
		return Err(ParseError::from_str("Field width or height can't be 0."));
	}

	Ok((XMLStates::Piece, dim))
}

fn parse_piece(
	r: &mut Parser,
	e: &BytesStart,
) -> Result<Option<XMLStates>, ParseError>
{
	if e.name() != b"piece" {
		return Ok(None);
	}

	let mut dim: Option<usize> = None;

	for a in e.attributes() {
		let x = a?;
		let v = x.unescape_and_decode_value(r)?;

		match x.key {
			b"r" => dim = Some(v.parse()?),
			_ => (),
		}
	}

	let dim = if let Some(d) = dim {
		d
	} else {
		return Err(ParseError::from_str("Piece has a missing width."));
	};

	if dim == 0 {
		return Err(ParseError::from_str("Pieces can't have a 0 width."));
	}

	return Ok(Some(XMLStates::PieceBody(Pattern::from_dim(dim))));
}

fn parse_piece_body(
	r: &mut Parser,
	e: &BytesText,
	mut p: Pattern,
) -> Result<(XMLStates, Pattern), ParseError>
{
	let mut data = e.unescape_and_decode(r)?;
	data.retain(|c| !c.is_whitespace());

	let pat_size = p.dim * p.dim;

	if pat_size != data.len() {
		return Err(ParseError::from_str(
			format!(
				"A piece size doesn't match it's given size. is: {}, should be: {}.",
				data.len().to_string(),
				pat_size.to_string()
			)
			.as_str(),
		));
	}

	let mut field = BitVec::with_capacity(pat_size);

	for i in data.chars() {
		match i {
			'1' => field.push(true),
			'0' => field.push(false),
			_ => return Err(ParseError::from_str("Characters must be 0's or 1's.")),
		}
	}

	p.colors = vec![Color::RED; 4];
	p.template = field;

	Ok((XMLStates::Piece, p))
}
