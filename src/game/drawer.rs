use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use crate::error::Error;

use super::size::ResizePattern;
use super::theme::Theme;
use super::Framework;

pub fn init<'a>(tc: &'a TextureCreator<WindowContext>, p: ResizePattern)
	-> (u32, Rect, Texture<'a>)
{
	let bs = p.block_size;
	let fr = p.field_rect;
	let ft = gen_field(tc, (fr.w as u32, fr.h as u32));

	(bs, fr, ft)
}

pub fn gen_field<'a>(tc: &'a TextureCreator<WindowContext>, dim: (u32, u32)) -> Texture<'a>
{
	tc.create_texture_target(None, dim.0 as u32, dim.1 as u32)
		.unwrap()
}

pub fn draw_blocks(canvas: &mut WindowCanvas, bs: u32, blocks: &[Point], colors: &[Color])
{
	for (c, b) in colors.iter().zip(blocks) {
		canvas.set_draw_color(*c);
		let r = Rect::new(b.x * bs as i32, b.y * bs as i32, bs, bs);

		canvas.fill_rect(r).unwrap();
	}
}

pub struct Renderer<'a>
{
	pub block_size: u32,

	// block_template: Texture<'a>,
	pub field_rect: Rect,
	pub field_bg_color: Color,
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
			let new_size = size * 2;

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

	let pieces_texture = tc
		.create_texture_target(None, field_rect.w as u32, field_rect.h as u32)
		.unwrap();

	Ok(Renderer::<'a> {
		block_size,
		field_rect,
		field_bg_color,
		field_border_color,
		pieces_texture,
	})
}

// pub fn draw_player(&self, canvas: &mut WindowCanvas)
// {
// 	let pcs = &self.piece_colors;
// 	let pbs = &self.piece_blocks;
// 	let fr = self.rfield_rect;
// 	let bs = self.rblock_size;
// 	let pos = self.piece_loc;
// 	let proj = self.piece_proj;
//
// 	debug_assert_eq!(pcs.len(), pbs.len());
//
// 	for (c, b) in pcs.iter().zip(pbs) {
// 		let color = Color::RGBA(c.r, c.g, c.b, c.a / 2);
// 		let block = Rect::new(
// 			fr.x + (b.x + pos.x) * bs as i32,
// 			fr.y + (b.y + proj) * bs as i32,
// 			bs,
// 			bs,
// 		);
//
// 		canvas.set_draw_color(color);
// 		canvas.fill_rect(block).unwrap();
//
// 		let color = *c;
// 		let block = Rect::new(
// 			fr.x + (b.x + pos.x) * bs as i32,
// 			fr.y + (b.y + pos.y) * bs as i32,
// 			bs,
// 			bs,
// 		);
//
// 		canvas.set_draw_color(color);
// 		canvas.fill_rect(block).unwrap();
// 	}
// }
