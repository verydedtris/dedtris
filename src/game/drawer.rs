use sdl2::{
	pixels::Color,
	rect::{Point, Rect},
	render::{Texture, TextureCreator, WindowCanvas},
	video::WindowContext,
};

use super::{state::gen, theme::Theme, Framework, Size};
use crate::error::Error;

pub mod player;
pub mod size;

pub struct Renderer<'a>
{
	pub block_size: u32,

	pub field_rect:         Rect,
	pub field_bg_color:     Color,
	pub field_border_color: Color,

	pub pieces_texture: Texture<'a>,

	pub player_texture: Texture<'a>,
	pub player_angle:   f64,
}

pub fn init_renderer<'a>(
	tc: &'a TextureCreator<WindowContext>, canvas: &mut WindowCanvas, win_dim: (u32, u32),
	field_dim: Size, start_piece: &gen::Piece,
) -> Result<Renderer<'a>, Error>
{
	let rp = size::new_resize(win_dim, field_dim);

	let pieces_texture = recreate_texture(tc, (rp.field_rect.w as u32, rp.field_rect.h as u32));
	let player_texture =
		player::create_piece_textures(tc, canvas, rp.block_size, std::slice::from_ref(start_piece))
			.pop()
			.unwrap();

	Ok(Renderer::<'a> {
		block_size: rp.block_size,

		field_rect: rp.field_rect,
		field_bg_color: Color::BLACK,
		field_border_color: Color::GRAY,

		pieces_texture,

		player_texture,
		player_angle: 0.,
	})
}

pub fn new_player_texture<'a>(
	rend: &mut Renderer<'a>, tc: &'a TextureCreator<WindowContext>, canvas: &mut WindowCanvas,
	piece: &gen::Piece,
)
{
	let bs = rend.block_size;

	rend.player_texture =
		player::create_piece_textures(tc, canvas, bs, std::slice::from_ref(piece)).pop().unwrap();
	rend.player_angle = 0.;
}

pub fn recreate_texture<'a>(tc: &'a TextureCreator<WindowContext>, field_dim: Size) -> Texture<'a>
{
	tc.create_texture_target(None, field_dim.0, field_dim.1).unwrap()
}

pub fn draw_blocks(canvas: &mut WindowCanvas, bs: u32, blocks: &[Point], colors: &[Color])
{
	for (c, b) in colors.iter().zip(blocks) {
		canvas.set_draw_color(*c);
		let r = Rect::new(b.x * bs as i32, b.y * bs as i32, bs, bs);

		canvas.fill_rect(r).unwrap();
	}
}
