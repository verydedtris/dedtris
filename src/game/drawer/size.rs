use sdl2::rect::Rect;
use super::Size;

pub fn calc_threshold(win_dim: (u32, u32)) -> u32
{
	u32::min(win_dim.0 / 10, win_dim.1 / 10)
}

