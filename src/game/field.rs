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
			rect: Rect::new(pos.0, pos.1, width, height),
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
	}

	pub fn check_valid(&self, piece: &Piece) -> bool
	{
		!piece.blocks.iter().any(|b| {
			self.blocks.contains(b)
				|| b.0 < 0 || b.0 >= self.width as i32
				|| b.1 >= self.height as i32
		})
	}
}

// -----------------------------------------------------------------------------
// Movable Piece
// -----------------------------------------------------------------------------

pub enum Direction
{
	LEFT,
	RIGHT,
	DOWN,
}

pub struct Piece
{
	pub color: Vec<Color>,
	pub blocks: Vec<(i32, i32)>,
}

impl Piece
{
	pub fn init() -> Piece
	{
		Piece {
			color: vec![],
			blocks: vec![],
		}
	}

	pub fn new(blocks: Vec<(i32, i32)>, color: Vec<Color>) -> Piece
	{
		Piece { color, blocks }
	}
}

impl Piece
{
	pub fn move_piece(&mut self, field: &Field, d: Direction) -> bool
	{
		let p: Piece = Piece::new(
			match d {
				Direction::LEFT => self.blocks.iter().map(|b| (b.0 - 1, b.1)).collect(),
				Direction::RIGHT => self.blocks.iter().map(|b| (b.0 + 1, b.1)).collect(),
				Direction::DOWN => self.blocks.iter().map(|b| (b.0, b.1 + 1)).collect(),
			},
			self.color.clone(),
		);

		let v = field.check_valid(&p);

		if v {
			*self = p;
		}

		v
	}
}
