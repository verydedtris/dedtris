use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use super::color_to_u32;
use super::size::ResizePattern;

pub struct DrawCache
{
	pub block_size: u32,

	pub field_rect: Rect,
	pub field_idx: Vec<(Color, usize)>,
	pub field_blocks: Vec<Rect>,

	pub player_blocks: Vec<(Color, Rect)>,
}

impl DrawCache
{
	pub fn init() -> Self
	{
		Self {
			block_size: 0,
			field_rect: Rect::new(0, 0, 0, 0),
			field_idx: Vec::new(),
			field_blocks: Vec::new(),
			player_blocks: Vec::new(),
		}
	}
}

pub fn set_size(cache: &mut DrawCache, p: ResizePattern)
{
	cache.block_size = p.block_size;
	cache.field_rect = p.field_rect;
}

pub fn clear_field_blocks(cache: &mut DrawCache)
{
	cache.field_idx.clear();
	cache.field_blocks.clear();
}

pub fn set_field_blocks(cache: &mut DrawCache, blocks: &[(i32, i32)], colors: &[Color])
{
	debug_assert!(blocks.len() == colors.len());

	// Add blocks

	cache.field_blocks = blocks
		.iter()
		.map(|b| {
			let bs = cache.block_size;
			let xy = cache.field_rect.top_left();
			Rect::new(xy.x + b.0 * bs as i32, xy.y + b.1 * bs as i32, bs, bs)
		})
		.collect();

	// Add colors

	cache.field_idx = Vec::with_capacity(blocks.len());

	let mut color_idxs: Vec<usize> = (0..colors.len()).collect();
	color_idxs.sort_unstable_by_key(|x| color_to_u32(colors[*x]));

	if let (Some(&first), Some(&last)) = (color_idxs.first(), color_idxs.last()) {
		let mut prev = first;

		for i in &color_idxs {
			if colors[prev] != colors[*i] || *i == last {
				cache.field_idx.push((colors[prev], *i));
				prev = *i;
			}
		}
	}
}

pub fn set_player_blocks(
	cache: &mut DrawCache,
	pos: (i32, i32),
	blocks: &[(i32, i32)],
	colors: &[Color],
)
{
	debug_assert_eq!(blocks.len(), colors.len());

	cache.player_blocks = blocks
		.iter()
		.zip(colors.iter())
		.map(|(b, c)| {
			let bs = cache.block_size;
			let xy = cache.field_rect.top_left();
			(
				*c,
				Rect::new(
					xy.x + (b.0 + pos.0) * bs as i32,
					xy.y + (b.1 + pos.1) * bs as i32,
					bs,
					bs,
				),
			)
		})
		.collect();
}

pub fn draw_field(cache: &DrawCache, canvas: &mut WindowCanvas)
{
	// Draw Outline

	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.fill_rect(cache.field_rect).unwrap();

	// Draw Blocks

	let mut prev = 0;

	for idx in &cache.field_idx {
		canvas.set_draw_color(idx.0);

		let s = &cache.field_blocks[prev..=idx.1];
		canvas.fill_rects(s).unwrap();

		prev = idx.1;
	}
}

pub fn draw_player(cache: &DrawCache, canvas: &mut WindowCanvas)
{
	for (c, r) in &cache.player_blocks {
		canvas.set_draw_color(*c);
		canvas.fill_rect(*r).unwrap();
	}
}
