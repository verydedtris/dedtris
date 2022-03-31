use sdl2::rect::Rect;
use super::Size;

pub fn calc_threshold(win_dim: (u32, u32)) -> u32
{
	u32::min(win_dim.0 / 10, win_dim.1 / 10)
}

pub struct ResizePattern
{
	pub block_size: u32,
	pub field_rect: Rect,
}

pub fn new_resize(win_dim: (u32, u32), field_dim: Size) -> ResizePattern
{
	let threshold = calc_threshold(win_dim);

	let block_size = u32::min(
		(win_dim.0 - threshold) / field_dim.0,
		(win_dim.1 - threshold) / field_dim.1,
	);

	let field_rect = {
		let (w, h) = (block_size * field_dim.0, block_size * field_dim.1);
		Rect::new(
			((win_dim.0 - w) / 2) as i32,
			((win_dim.1 - h) / 2) as i32,
			w,
			h,
		)
	};

	ResizePattern {
		block_size,
		field_rect,
	}
}
