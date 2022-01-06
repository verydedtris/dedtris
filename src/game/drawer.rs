use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

use super::size::ResizePattern;

pub fn init<'a>(tc: &'a TextureCreator<WindowContext>, p: ResizePattern) -> (u32, Rect, Texture<'a>)
{
    let bs = p.block_size;
    let fr = p.field_rect;
    let ft = gen_field(tc, (fr.w as u32, fr.h as u32));

    (bs, fr, ft)
}

pub fn gen_field<'a>(tc: &'a TextureCreator<WindowContext>, dim: (u32, u32)) -> Texture<'a>
{
	tc.create_texture_target(None, dim.0 as u32, dim.1 as u32).unwrap()
}

pub fn draw_blocks(canvas: &mut WindowCanvas, bs: u32, blocks: &[Point], colors: &[Color])
{
	for (c, b) in colors.iter().zip(blocks) {
		canvas.set_draw_color(*c);
		let r = Rect::new(b.x * bs as i32, b.y * bs as i32, bs, bs);

		canvas.fill_rect(r).unwrap();
	}
}
