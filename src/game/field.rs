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
	pub fn init(field_dim: (usize, usize)) -> Self
	{
		Field {
			rect: Rect::new(0, 0, 0, 0),
			blocks: Vec::new(),
			colors: Vec::new(),
			field_dim,
			block_size: 0,
		}
	}
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

	pub fn add_pieces(&mut self, blocks: &[(i32, i32)], color: &[Color])
	{
		debug_assert!(self.check_valid(blocks));
		debug_assert_eq!(blocks.len(), color.len());

		self.colors.extend(color);
		self.blocks.extend_from_slice(blocks);
	}

	pub fn count_lines(&self) -> Vec<i32>
	{
		let mut lines = vec![0i32; self.field_dim.1];

		for b in &self.blocks {
			lines[b.1 as usize] += 1;
		}

		lines
	}

	pub fn lines_list(&self) -> Vec<i32>
	{
		self.count_lines()
			.iter()
			.enumerate()
			.filter_map(|(i, l)| {
				if *l >= self.field_dim.0 as i32 {
					Some(i as i32)
				} else {
					None
				}
			})
			.collect()
	}

	pub fn clear_lines(&mut self)
	{
		let lines = self.lines_list();

		println!("Filled lines: {:?}", lines);

		let mut removed = 0;
		for i in 0..self.blocks.len() {
			let i = i - removed;

			if let Some(ii) = lines.iter().position(|l| *l >= self.blocks[i].1) {
				println!("Line: {}", lines[ii]);

				if self.blocks[i].1 == lines[ii] as i32 {
					self.blocks.swap_remove(i);
					self.colors.swap_remove(i);
					removed += 1;
				} else {
					let shift = lines.len() - ii;
					println!("Shift: {}", shift);
					self.blocks[i].1 += shift as i32;
				}
			}
		}
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
