use bitmaps::Bitmap;
use sdl2::pixels::Color;

type Template = (Bitmap<64>, Color);

pub struct Pieces
{
	pub templates: Vec<Template>,
	pub template_size: usize,
}

impl Pieces
{
	pub fn init() -> Self
	{
		Pieces {
			template_size: 4,
			templates: vec![
				// Load defaults
				(Bitmap::<64>::from_value(0b0000001001100100u64), Color::RED),
				(Bitmap::<64>::from_value(0b0000010001100010u64), Color::BLUE),
				(
					Bitmap::<64>::from_value(0b0000001001110000u64),
					Color::GREEN,
				),
				(
					Bitmap::<64>::from_value(0b0000010001000110u64),
					Color::YELLOW,
				),
				(Bitmap::<64>::from_value(0b0000001000100110u64), Color::GRAY),
				(Bitmap::<64>::from_value(0b0100010001000100u64), Color::CYAN),
			],
		}
	}

	pub fn spawn_piece(&self, temp_idx: usize) -> Vec<(i32, i32)>
	{
		debug_assert!(self.templates.len() > temp_idx);

		let t = &self.templates[temp_idx];

		let mut r: Vec<(i32, i32)> = vec![];
		r.reserve(4);

		for i in 0..self.template_size * self.template_size {
			if t.0.get(i.into()) {
				r.push((
					(i % self.template_size) as i32,
					(i / self.template_size) as i32,
				));
			}
		}

		r
	}

}
