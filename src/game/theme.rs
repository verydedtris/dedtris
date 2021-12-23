use std::path::Path;

use bitvec::prelude::BitVec;
use quick_xml::events::{BytesStart, BytesText, Event};
use sdl2::pixels::Color;

use crate::file;

// -----------------------------------------------------------------------------
// Theme Layout
// -----------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Pattern
{
	pub dim: usize,
	pub colors: Vec<Color>,
	pub template: BitVec,
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

impl Pattern
{
	fn from_dim(dim: usize) -> Self
	{
		Pattern {
			dim,
			colors: Vec::new(),
			template: BitVec::new(),
		}
	}
}

impl Default for Theme
{
	fn default() -> Self
	{
		Theme {
			bg_color: Color::WHITE,
			field_bg_color: Color::BLACK,
			field_edge_color: Color::GRAY,
			field_dim: (0, 0),
			patterns: Vec::new(),
		}
	}
}

impl Theme
{
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
// XML File Parser
// -----------------------------------------------------------------------------

enum XMLStates
{
	Theme,
	Piece,
	PieceBody(Pattern),
}

impl Theme
{
	pub fn load(path: &Path) -> Result<Theme, file::ParseError>
	{
		let mut reader = file::load_xml(path)?;
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
							return Err(file::ParseError::from_str("Missing piece body."))
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

				Err(e) => return Err(file::ParseError::from_pos(&reader, &e)),
				_ => (),
			}

			buffer.clear();
		}

		// Check validity

		let dim = if let Some(d) = dim {
			d
		} else {
			return Err(file::ParseError::from_str("Missing field size."));
		};

		if patterns.is_empty() {
			return Err(file::ParseError::from_str(
				"There must be at least 1 pattern.",
			));
		}

		Ok(Theme::from_data(patterns, dim))
	}
}

// -----------------------------------------------------------------------------
// Theme Parser
// -----------------------------------------------------------------------------

fn parse_root(
	r: &mut file::Parser,
	e: &BytesStart,
) -> Result<(XMLStates, (usize, usize)), file::ParseError>
{
	if e.name() != b"theme" {
		return Err(file::ParseError::from_str("No tetris root found."));
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
		return Err(file::ParseError::from_str(
			"Field width or height can't be 0.",
		));
	}

	Ok((XMLStates::Piece, dim))
}

fn parse_piece(r: &mut file::Parser, e: &BytesStart)
	-> Result<Option<XMLStates>, file::ParseError>
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
		return Err(file::ParseError::from_str("Piece has a missing width."));
	};

	if dim == 0 {
		return Err(file::ParseError::from_str("Pieces can't have a 0 width."));
	}

	return Ok(Some(XMLStates::PieceBody(Pattern::from_dim(dim))));
}

fn parse_piece_body(
	r: &mut file::Parser,
	e: &BytesText,
	mut p: Pattern,
) -> Result<(XMLStates, Pattern), file::ParseError>
{
	let mut data = e.unescape_and_decode(r)?;
	data.retain(|c| !c.is_whitespace());

	let pat_size = p.dim * p.dim;

	if pat_size != data.len() {
		return Err(file::ParseError::from_str(
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
			_ => return Err(file::ParseError::from_str("Characters must be 0's or 1's.")),
		}
	}

	p.colors = vec![Color::RED; 4];
	p.template = field;

	Ok((XMLStates::Piece, p))
}
