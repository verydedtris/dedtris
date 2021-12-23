use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Field
{
	pub rect: Rect,

	pub blocks: Vec<(i32, i32)>,
	pub colors: Vec<Color>,

	pub field_dim: (usize, usize),
	pub block_size: u32,
}

impl Field
{
	pub fn draw_blocks_delta(
		&self,
		canvas: &mut WindowCanvas,
		pos: (i32, i32),
		blocks: &[(i32, i32)],
		colors: &[Color],
	)
	{
		for (b, c) in blocks.iter().zip(colors.iter()) {
			canvas.set_draw_color(*c);

			let rect = Rect::new(
				self.rect.x + (pos.0 + b.0) * self.block_size as i32,
				self.rect.y + (pos.1 + b.1) * self.block_size as i32,
				self.block_size,
				self.block_size,
			);

			canvas.fill_rect(rect).unwrap();
		}
	}

	pub fn draw_blocks(&self, canvas: &mut WindowCanvas, blocks: &[(i32, i32)], colors: &[Color])
	{
		self.draw_blocks_delta(canvas, (0, 0), blocks, colors)
	}

	pub fn init(block_size: u32, dim: (usize, usize)) -> Self
	{
		Field {
			rect: Rect::new(0, 0, dim.0 as u32 * block_size, dim.1 as u32 * block_size),
			blocks: Vec::default(),
			colors: Vec::default(),
			field_dim: dim,
			block_size,
		}
	}

	pub fn add_pieces(&mut self, blocks: &[(i32, i32)], color: &[Color])
	{
		debug_assert!(self.check_valid(blocks));
		debug_assert_eq!(blocks.len(), color.len());

		self.colors.extend(color);
		self.blocks.extend_from_slice(blocks);
	}

	pub fn check_valid_pos(&self, pos: (i32, i32), blocks: &[(i32, i32)]) -> bool
	{
		!blocks.iter().any(|block| {
			let b = (block.0 + pos.0, block.1 + pos.1);

			self.blocks.contains(&b)
				|| b.0 < 0 || b.0 >= self.field_dim.0 as i32
				|| b.1 >= self.field_dim.1 as i32
		})
	}

	pub fn check_valid(&self, blocks: &[(i32, i32)]) -> bool
	{
		self.check_valid_pos((0, 0), blocks)
	}
}
