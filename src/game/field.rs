use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Field
{
	pub rect: Rect,

	pub blocks: Vec<(i32, i32)>,
	pub colors: Vec<Color>,

	pub width: u32,
	pub height: u32,
	pub block_size: u32,
}

impl Field
{
	pub fn init(pos: (i32, i32), block_size: u32) -> Self
	{
		const W: u32 = 10;
		const H: u32 = 20;

		let width = W * block_size;
		let height = H * block_size;

		Field {
			rect: Rect::new(
				pos.0,
                pos.1,
				width,
				height,
			),
			blocks: vec![],
			colors: vec![],
			width: W,
			height: H,
			block_size,
		}
	}

	pub fn add_pieces(&mut self, blocks: &[(i32, i32)], color: Color)
	{
		self.colors.resize(self.colors.len() + blocks.len(), color);

		self.blocks.reserve(blocks.len());
		for i in blocks {
			self.blocks.push(*i);
		}

        self.blocks.sort_unstable();
	}
}
