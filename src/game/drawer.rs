use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use super::color_to_u32;
use super::size::ResizePattern;

pub struct DrawCache
{
	pub bs: u32,

	pub fr: Rect,
	pub fis: Vec<(Color, usize)>,
	pub fbs: Vec<Rect>,

	pub pcs: Vec<Color>,
	pub pbs: Vec<Rect>,
}

impl DrawCache
{
	pub fn init() -> Self
	{
		Self {
			bs: 0,
			fr: Rect::new(0, 0, 0, 0),
			fis: Vec::new(),
			fbs: Vec::new(),
			pcs: Vec::new(),
			pbs: Vec::new(),
		}
	}
}

pub fn set_size(cache: &mut DrawCache, p: ResizePattern)
{
	cache.bs = p.block_size;
	cache.fr = p.field_rect;
}

pub fn clear_field_blocks(cache: &mut DrawCache)
{
	cache.fis.clear();
	cache.fbs.clear();
}

pub fn set_field_blocks(cache: &mut DrawCache, blocks: &[(i32, i32)], colors: &[Color])
{
	debug_assert!(blocks.len() == colors.len());

	cache.fbs = Vec::with_capacity(blocks.len());
	cache.fis = Vec::new();

	let mut cis: Vec<usize> = (0..colors.len()).collect();
	cis.sort_unstable_by_key(|x| color_to_u32(colors[*x]));

	if let (Some(&first), Some(&last)) = (cis.first(), cis.last()) {
		let mut prev = first;

		for (i, idx) in cis.iter().enumerate() {
			cache.fbs.push(Rect::new(
				cache.fr.x + blocks[*idx].0 * cache.bs as i32,
				cache.fr.y + blocks[*idx].1 * cache.bs as i32,
				cache.bs,
				cache.bs,
			));

			if colors[prev] != colors[*idx] || *idx == last {
				cache.fis.push((colors[prev], i));
				prev = *idx;
			}
		}
	}
}

pub fn set_player_blocks(
	cache: &mut DrawCache,
	pos: (i32, i32),
	projection: i32,
	blocks: &[(i32, i32)],
	colors: &[Color],
)
{
	debug_assert_eq!(blocks.len(), colors.len());

	cache.pcs = colors.to_owned();
	cache.pbs = blocks
		.iter()
		.map(|b| {
			let bs = cache.bs;
			let xy = cache.fr.top_left();
			Rect::new(
				xy.x + (b.0 + pos.0) * bs as i32,
				xy.y + (b.1 + projection) * bs as i32,
				bs,
				bs,
			)
		})
		.chain(blocks.iter().map(|b| {
			let bs = cache.bs;
			let xy = cache.fr.top_left();
			Rect::new(
				xy.x + (b.0 + pos.0) * bs as i32,
				xy.y + (b.1 + pos.1) * bs as i32,
				bs,
				bs,
			)
		}))
		.collect();
}

pub fn draw_field(cache: &DrawCache, canvas: &mut WindowCanvas)
{
	// Draw Outline

	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.fill_rect(cache.fr).unwrap();

	// Draw Blocks

	let mut prev = 0;

	for idx in &cache.fis {
		canvas.set_draw_color(idx.0);

		let s = &cache.fbs[prev..=idx.1];
		canvas.fill_rects(s).unwrap();

		prev = idx.1;
	}
}

pub fn draw_player(cache: &DrawCache, canvas: &mut WindowCanvas)
{
	debug_assert_eq!(cache.pcs.len() * 2, cache.pbs.len());

	for (c, b) in cache.pcs.iter().zip(&cache.pbs) {
		let c = Color::RGBA(c.r, c.g, c.b, c.a / 2);
		canvas.set_draw_color(c);
		canvas.fill_rect(*b).unwrap();
	}

	for (c, b) in cache.pcs.iter().zip(cache.pbs[cache.pcs.len()..].iter()) {
		canvas.set_draw_color(*c);
		canvas.fill_rect(*b).unwrap();
	}
}
