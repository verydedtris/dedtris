use bitvec::prelude::BitVec;
use sdl2::pixels::Color;

use super::theme;

// -----------------------------------------------------------------------------
// Piece
// -----------------------------------------------------------------------------

pub struct Piece
{
	pub dim: usize,
	pub colors: Vec<Color>,
	pub blocks: Vec<(i32, i32)>,
}

// -----------------------------------------------------------------------------
// Piece generator
// -----------------------------------------------------------------------------

pub struct Pattern
{
	pub dim: usize,
	pub colors: Vec<Color>,
	pub template: BitVec,
}

impl Pattern
{
	fn from_data(dim: usize, colors: Vec<Color>, template: BitVec) -> Pattern
	{
		Pattern {
			dim,
			colors,
			template,
		}
	}
}

pub struct Pieces
{
	pub templates: Vec<Pattern>,
}

impl Pieces
{
	pub fn init(ps: Vec<theme::Pattern>) -> Self
	{
		let mut templates = Vec::new();

		for i in ps {
			templates.push(Pattern::from_data(i.dim, i.colors, i.template));
		}

		Pieces { templates }
	}

	pub fn spawn_piece(&self, temp_idx: usize) -> Piece
	{
		debug_assert!(self.templates.len() > temp_idx);

		let t = &self.templates[temp_idx];

		let mut r = Vec::new();
		r.reserve(t.colors.len());

		for i in 0..t.dim * t.dim {
			if t.template[i as usize] {
				r.push(((i % t.dim) as i32, (i / t.dim) as i32));
			}
		}

		debug_assert_eq!(t.colors.len(), r.len());

		Piece {
			dim: t.dim,
			colors: t.colors.clone(),
			blocks: r,
		}
	}
}
