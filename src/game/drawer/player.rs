use sdl2::{
	rect::Rect,
	render::{Texture, TextureCreator, WindowCanvas},
	video::WindowContext,
};

use crate::game::state::gen;

pub fn create_player_texture<'a>(
	tc: &'a TextureCreator<WindowContext>, canvas: &mut WindowCanvas, block_size: u32,
	piece: &gen::Piece,
) -> Texture<'a>
{
	let size = piece.dim * block_size;
	let mut t =
		tc.create_texture_target(sdl2::pixels::PixelFormatEnum::ARGB8888, size, size).unwrap();

	t.set_blend_mode(sdl2::render::BlendMode::Blend);

	canvas
		.with_texture_canvas(&mut t, |canvas| {
			let pb = &piece.blocks;
			let pc = &piece.colors[0];

			let rs: Vec<Rect> = pb
				.iter()
				.map(|block| {
					Rect::new(
						block.x * block_size as i32,
						block.y * block_size as i32,
						block_size,
						block_size,
					)
				})
				.collect();

			canvas.set_draw_color(*pc);
			canvas.fill_rects(&rs).unwrap();
		})
		.unwrap();

	t
}
