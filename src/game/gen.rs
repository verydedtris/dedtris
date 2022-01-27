use bitvec::prelude::BitVec;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use sdl2::pixels::Color;
use sdl2::rect::Point;

use crate::lua;

use super::theme;

// -----------------------------------------------------------------------------
// Piece
// -----------------------------------------------------------------------------

#[derive(Default)]
pub struct Piece
{
	pub dim: usize,
	pub colors: Vec<Color>,
	pub blocks: Vec<Point>,
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

pub fn init(patterns: Vec<theme::Pattern>) -> (ThreadRng, Vec<Pattern>)
{
    let rng = rand::thread_rng();
    let ps = transform_theme_patterns(patterns);

    (rng, ps)
}

pub fn transform_pattern(pattern: &Pattern) -> Piece
{
	let mut r = Vec::with_capacity(pattern.colors.len());

	for i in 0..pattern.dim * pattern.dim {
		if pattern.template[i as usize] {
			r.push(Point::new(
				(i % pattern.dim) as i32,
				(i / pattern.dim) as i32,
			));
		}
	}

	debug_assert_eq!(pattern.colors.len(), r.len());

	Piece {
		dim: pattern.dim,
		colors: pattern.colors.clone(),
		blocks: r,
	}
}

pub fn spawn_piece(rng: &mut ThreadRng, patterns: &[Pattern]) -> Piece
{
	debug_assert!(!patterns.is_empty());

	let p = patterns.choose(rng).unwrap();
	transform_pattern(p)
}

pub fn transform_theme_patterns(patterns: Vec<theme::Pattern>) -> Vec<Pattern>
{
	patterns
		.into_iter()
		.map(|p| Pattern {
			dim: p.dim,
			colors: p.colors,
			template: p.template,
		})
		.collect()
}
