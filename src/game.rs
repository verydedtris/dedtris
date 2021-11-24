use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

use super::component::*;

mod pieces;
use pieces::*;

mod field;
use field::*;

pub struct Data
{
	field: Field,
	pieces: Pieces,
}

impl Data
{
	fn draw_field(&self, canvas: &mut WindowCanvas)
	{
		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.fill_rect(self.field.rect).unwrap();
	}

	fn draw_pieces(&self, canvas: &mut WindowCanvas)
	{
		for (b, c) in self.field.blocks.iter().zip(self.field.colors.iter()) {
			canvas.set_draw_color(*c);

			let rect = Rect::new(
				self.field.rect.x + b.0 * self.field.block_size as i32,
				self.field.rect.y + b.1 * self.field.block_size as i32,
				self.field.block_size,
				self.field.block_size,
			);

			canvas.fill_rect(rect).unwrap();
		}
	}
}

impl Component for Data
{
	fn init(window: &Window) -> Self
	{
		const THESHOLD: u32 = 20;

		let win_size = window.size();
		let block = (win_size.1 - THESHOLD) / 20;

		Data {
			pieces: Pieces::init(),
            field: Field::init((0, 0), block),
		}
	}

	fn handle_event(&mut self, event: &Event)
	{
		match event {
			Event::KeyDown {
				keycode: Some(Keycode::O),
				..
			} => {
				self.field.add_pieces(&self.pieces.spawn_piece(0), Color::GREEN);
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
