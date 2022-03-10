use sdl2::rect::Rect;

use super::{field, Size};

pub struct ResizePattern
{
	pub block_size: u32,
	pub field_rect: Rect,
}

impl ResizePattern
{
	fn new(block_size: u32, field_rect: Rect) -> Self
	{
		Self {
			block_size,
			field_rect,
		}
	}
}

pub struct WindowSize
{
	w: u32,
	h: u32,
}

pub const BLOCK_SIZE: u32 = 32;

pub fn calc_window_size(field_dim: Size) -> WindowSize
{
	const THESHOLD: u32 = 20;

	let w = field_dim.0 as u32 * BLOCK_SIZE + 2 * THESHOLD;
	let h = field_dim.1 as u32 * BLOCK_SIZE + 2 * THESHOLD;

	WindowSize { w, h }
}

pub fn new_resize(win_dim: (u32, u32), field_dim: (usize, usize)) -> ResizePattern
{
	const THESHOLD: u32 = 20;
	let block = (win_dim.1 - THESHOLD) / (field_dim.1 as u32 + 2);

	let field_w = block * field_dim.0 as u32;
	let field_h = block * field_dim.1 as u32;

	let field_x = (win_dim.0 - field_w) / 2;
	let field_y = (win_dim.1 - field_h) / 2;

	let field_dim = Rect::new(field_x as i32, field_y as i32, field_w, field_h);

	ResizePattern::new(block, field_dim)
}
