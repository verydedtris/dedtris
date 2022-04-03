use std::path::Path;

use sdl2::{
	image::LoadTexture,
	pixels::Color,
	rect::{Point, Rect},
	render::{Texture, TextureCreator, WindowCanvas},
	video::WindowContext,
};

use super::Size;
use crate::error::Error;

pub mod size;

pub struct Renderer<'a>
{
	pub win_dim: (u32, u32),

	pub field_bg_color:     Color,
	pub field_border_color: Color,

	pub block_texture: Texture<'a>,
}

pub fn init_renderer<'a>(
	tc: &'a TextureCreator<WindowContext>, win_dim: (u32, u32), block_bmp: &Path,
) -> Result<Renderer<'a>, Error>
{
	let mut block_texture = tc.load_texture(block_bmp)?;
	block_texture.set_blend_mode(sdl2::render::BlendMode::Blend);

	Ok(Renderer::<'a> {
		win_dim,

		field_bg_color: Color::BLACK,
		field_border_color: Color::GRAY,

		block_texture,
	})
}

impl Renderer<'_>
{
	pub fn draw_blocks(
		&mut self, canvas: &mut WindowCanvas, offset: Point, bs: u32, blocks: &[Point],
		colors: &[Color],
	)
	{
		let btex = &mut self.block_texture;

		for (c, b) in colors.iter().zip(blocks) {
			let r = Rect::new(
				offset.x + b.x * bs as i32,
				offset.y + b.y * bs as i32,
				bs,
				bs,
			);

			btex.set_color_mod(c.r, c.g, c.b);
			canvas.copy(&btex, None, r).unwrap();
		}
	}
}
