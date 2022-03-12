use sdl2::{
	pixels::Color,
	rect::{Point, Rect},
	render::{Texture, WindowCanvas},
};

use self::size::ResizePattern;
use super::{theme::Theme, Framework, Size};
use crate::error::Error;

mod size;

pub struct Renderer<'a>
{
	pub block_size: u32,

	// block_template: Texture<'a>,
	pub field_rect:         Rect,
	pub field_bg_color:     Color,
	pub field_border_color: Color,

	pub pieces_texture: Texture<'a>,
	// fall_piece_locs: Vec<Point>,
}

pub fn init_renderer<'a>(
	fw: &Framework<'_, '_, '_, 'a, '_, '_>, t: &Theme,
) -> Result<Renderer<'a>, Error>
{
	let window = fw.canvas.window();
	let tc = fw.tex_maker;

	let fs = t.field_dim;

	let field_bg_color = Color::BLACK;
	let field_border_color = Color::GRAY;

	let wd = window.drawable_size();

	let ResizePattern {
		block_size,
		field_rect,
	} = size::new_resize(wd, fs);

	let pieces_texture =
		tc.create_texture_target(None, field_rect.w as u32, field_rect.h as u32).unwrap();

	Ok(Renderer::<'a> {
		block_size,
		field_rect,
		field_bg_color,
		field_border_color,
		pieces_texture,
	})
}

pub fn resize_game(drawerer: &mut Renderer<'_>, win_dim: (u32, u32), field_dim: Size)
{
	let ResizePattern {
		block_size,
		field_rect,
	} = size::new_resize(win_dim, field_dim);

    drawerer.block_size = block_size;
    drawerer.field_rect = field_rect;
}

pub struct WindowSize
{
	pub w: u32,
	pub h: u32,
}

pub fn calc_window_size(field_dim: Size) -> WindowSize
{
	const BLOCK_SIZE: u32 = 28;

    let threshold = u32::min(field_dim.0, field_dim.1) * BLOCK_SIZE / 4;

	let w = field_dim.0 * BLOCK_SIZE + threshold;
	let h = field_dim.1 * BLOCK_SIZE + threshold;

	WindowSize { w, h }
}

pub fn draw_blocks(canvas: &mut WindowCanvas, bs: u32, blocks: &[Point], colors: &[Color])
{
	for (c, b) in colors.iter().zip(blocks) {
		canvas.set_draw_color(*c);
		let r = Rect::new(b.x * bs as i32, b.y * bs as i32, bs, bs);

		canvas.fill_rect(r).unwrap();
	}
}
