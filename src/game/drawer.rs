use sdl2::{
	pixels::Color,
	rect::{Point, Rect},
	render::{Texture, WindowCanvas},
};

use super::{theme::Theme, Framework};
use crate::error::Error;

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
	fw: &Framework<'_, '_, '_, 'a, '_, '_>,
	t: &Theme,
) -> Result<Renderer<'a>, Error>
{
	let window = fw.canvas.window();
	let tc = fw.tex_maker;

	let field_block_size = t.field_dim;

	let (width, height) = window.drawable_size();

	let block_size = {
		let mut size = 4;

		loop {
			let new_size = size + 4;

			if new_size * field_block_size.0 > width || new_size * field_block_size.1 > height {
				break size;
			}

			size = new_size;
		}
	};

	let field_rect = {
		let (w, h) = (
			block_size * field_block_size.0,
			block_size * field_block_size.1,
		);
		Rect::new(((width - w) / 2) as i32, ((height - h) / 2) as i32, w, h)
	};
	let field_bg_color = Color::BLACK;
	let field_border_color = Color::GRAY;

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

pub fn draw_blocks(canvas: &mut WindowCanvas, bs: u32, blocks: &[Point], colors: &[Color])
{
	for (c, b) in colors.iter().zip(blocks) {
		canvas.set_draw_color(*c);
		let r = Rect::new(b.x * bs as i32, b.y * bs as i32, bs, bs);

		canvas.fill_rect(r).unwrap();
	}
}
