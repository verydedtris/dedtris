use bitvec::prelude::BitVec;
use sdl2::pixels::Color;

use super::theme;

// -----------------------------------------------------------------------------
// Piece
// -----------------------------------------------------------------------------

#[derive(Default)]
pub struct Piece
{
	pub dim: usize,
	pub colors: Vec<Color>,
	pub blocks: Vec<(i32, i32)>,
}

impl Piece
{
	pub fn new(dim: usize, colors: Vec<Color>, blocks: Vec<(i32, i32)>) -> Self
	{
		Self {
			dim,
			colors,
			blocks,
		}
	}
}

impl Piece
{
	pub fn rotate(&self) -> Vec<(i32, i32)>
	{
		self.blocks.iter().map(|b| (self.dim as i32 - 1 - b.1, b.0)).collect()
	}

	pub fn move_delta(&self, d: (i32, i32)) -> Vec<(i32, i32)>
	{
		self.blocks.iter().map(|b| (b.0 + d.0, b.1 + d.1)).collect()
	}
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
	fn new(dim: usize, colors: Vec<Color>, template: BitVec) -> Pattern
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
		Pieces {
			templates: ps.into_iter().map(|p| Pattern::new(p.dim, p.colors, p.template)).collect(),
		}
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

		Piece::new(t.dim, t.colors.clone(), r)
	}
}
