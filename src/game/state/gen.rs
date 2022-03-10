use sdl2::pixels::Color;
use sdl2::rect::Point;

// -----------------------------------------------------------------------------
// Piece
// -----------------------------------------------------------------------------

#[derive(Default)]
pub struct Piece
{
	pub dim: u32,
	pub colors: Vec<Color>,
	pub blocks: Vec<Point>,
}
