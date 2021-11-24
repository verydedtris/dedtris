use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

use super::component::*;

mod pieces;
use pieces::*;

pub struct Data
{
	width: u32,
	height: u32,

	field: Rect,

	pieces: Pieces,
}

impl Data
{
	fn draw_field(&self, canvas: &mut WindowCanvas)
	{
		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.fill_rect(self.field).unwrap();
	}

	fn draw_pieces(&self, canvas: &mut WindowCanvas)
	{
		for (b, c) in self.pieces.blocks.iter().zip(self.pieces.colors.iter()) {
			canvas.set_draw_color(*c);

			let rect = Rect::new(
				self.field.x + b.0 * self.pieces.block_size as i32,
				self.field.y + b.1 * self.pieces.block_size as i32,
				self.pieces.block_size,
				self.pieces.block_size,
			);

			canvas.fill_rect(rect).unwrap();
		}
	}
}

impl Component for Data
{
	fn init(window: &Window) -> Self
	{
		const W: u32 = 10;
		const H: u32 = 20;
		const THESHOLD: u32 = 20;

		let win_size = window.size();

		let block = (win_size.1 - THESHOLD) / 20;

		let width = W * block;
		let height = H * block;

		Data {
			width: W,
			height: H,
			field: Rect::new(
				((win_size.0 - width) / 2) as i32,
				((win_size.1 - height) / 2) as i32,
				width,
				height,
			),
			pieces: Pieces::init(block),
		}
	}

	fn handle_event(&mut self, event: &Event)
	{
		match event {
			Event::KeyDown {
				keycode: Some(Keycode::O),
				..
			} => {
				self.pieces.add_pieces(&[(5, 5)], Color::GREEN);
				println!("Added piece");
			}
			_ => {}
		}
	}

	fn draw(&self, canvas: &mut WindowCanvas)
	{
		self.draw_field(canvas);
		self.draw_pieces(canvas);
	}
}
