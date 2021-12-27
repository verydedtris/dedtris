use bitvec::prelude::BitVec;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
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
	rng: ThreadRng,

	pub templates: Vec<Pattern>,
	pub choice_left: Vec<usize>,
}

impl Pieces
{
	pub fn init(ps: Vec<theme::Pattern>) -> Self
	{
		let mut rng = rand::thread_rng();
		let choice_left = respawn_bag(ps.len(), &mut rng);
		let templates = ps.into_iter().map(|p| Pattern::new(p.dim, p.colors, p.template)).collect();

		Pieces {
			rng,
			templates,
			choice_left,
		}
	}
}

pub fn get_next_piece(ps: &mut Pieces) -> Piece
{
	if ps.choice_left.is_empty() {
		ps.choice_left = respawn_bag(ps.templates.len(), &mut ps.rng);
	}

	debug_assert!(!ps.choice_left.is_empty());

	let i = ps.rng.gen_range(0..ps.choice_left.len());
	let p = spawn_piece_idx(&ps, ps.choice_left[i]);

	ps.choice_left.remove(i);
	p
}

pub fn spawn_piece(pattern: &Pattern) -> Piece
{
	let mut r = Vec::with_capacity(pattern.colors.len());

	for i in 0..pattern.dim * pattern.dim {
		if pattern.template[i as usize] {
			r.push(((i % pattern.dim) as i32, (i / pattern.dim) as i32));
		}
	}

	debug_assert_eq!(pattern.colors.len(), r.len());

	Piece::new(pattern.dim, pattern.colors.clone(), r)
}

pub fn spawn_piece_idx(ps: &Pieces, temp_idx: usize) -> Piece
{
	debug_assert!(ps.templates.len() > temp_idx);

	spawn_piece(&ps.templates[temp_idx])
}

fn respawn_bag(size: usize, rng: &mut ThreadRng) -> Vec<usize>
{
	let mut bag: Vec<usize> = (0..size).into_iter().cycle().take(size * 2).collect();
	bag.shuffle(rng);

	bag
}
