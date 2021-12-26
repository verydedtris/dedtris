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

impl DrawCache
{
	pub fn set_size(&mut self, p: ResizePattern)
	{
		self.block_size = p.block_size;
		self.field_rect = p.field_rect;
	}

	pub fn clear_field_blocks(&mut self)
	{
		self.field_idx.clear();
		self.field_blocks.clear();
	}

	pub fn set_field_blocks(&mut self, blocks: &[(i32, i32)], colors: &[Color])
	{
		debug_assert!(blocks.len() > 0 && blocks.len() == colors.len());

		// Clear cache

		self.clear_field_blocks();

		// Create indices and sort

		let mut color_idxs: Vec<usize> = (0..colors.len()).collect();
		color_idxs.sort_unstable_by_key(|x| color_to_u32(colors[*x]));

		// Add colors

		let mut prev = color_idxs[color_idxs.len() - 1];

		for i in &color_idxs {
			if colors[prev] != colors[*i] {
				self.field_idx.push((colors[prev], *i));
				prev = *i;
			}
		}

		self.field_idx.push((
			colors[color_idxs[color_idxs.len() - 1]],
			color_idxs[color_idxs.len() - 1],
		));

		// Add blocks

		let bs = self.block_size;
		let xy = self.field_rect.top_left();

		let test = blocks
			.iter()
			.map(move |b| Rect::new(xy.x + b.0 * bs as i32, xy.y + b.1 * bs as i32, bs, bs));

		self.field_blocks.extend(test);
	}

	pub fn set_player_blocks(&mut self, pos: (i32, i32), blocks: &[(i32, i32)], colors: &[Color])
	{
		debug_assert_eq!(blocks.len(), colors.len());

		self.player_blocks.clear();

		let bs = self.block_size;
		let xy = self.field_rect.top_left();

		let test = blocks.iter().zip(colors.iter()).map(move |(b, c)| {
			(
				*c,
				Rect::new(
					xy.x + (b.0 + pos.0) * bs as i32,
					xy.y + (b.1 + pos.1) * bs as i32,
					bs,
					bs,
				),
			)
		});

		self.player_blocks.extend(test);
	}
}

impl DrawCache
{
	pub fn draw_field(&self, canvas: &mut WindowCanvas)
	{
		// Draw Outline

		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.fill_rect(self.field_rect).unwrap();

		// Draw Blocks

		let mut prev = 0;

		for idx in &self.field_idx {
			canvas.set_draw_color(idx.0);

			let s = &self.field_blocks[prev..=idx.1];
			canvas.fill_rects(s).unwrap();

			prev = idx.1;
		}
	}

	pub fn draw_player(&self, canvas: &mut WindowCanvas)
	{
		for (c, r) in &self.player_blocks {
			canvas.set_draw_color(*c);
			canvas.fill_rect(*r).unwrap();
		}
	}
}
