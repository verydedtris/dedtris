use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::file::Theme;
use crate::game::pieces::Direction;

use self::field::Field;
use self::gen::Pieces;
use self::pieces::PlayerPiece;

mod field;
mod gen;
mod pieces;

pub struct Data
{
	field: Field,
	pieces: Pieces,
	piece: Option<PlayerPiece>,
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
		self.field.draw_blocks(canvas, &self.field.blocks, &self.field.colors);
	}

	fn draw_piece(&self, canvas: &mut WindowCanvas)
	{
		if let Some(p) = &self.piece {
			self.field.draw_blocks_delta(canvas, p.pos, &p.blocks, &p.colors);
		}
	}
}

impl Data
{
	pub fn init(dim: (u32, u32), t: Theme) -> Self
	{
		const THESHOLD: u32 = 20;
		let block = (dim.1 - THESHOLD) / 20;

		let d = Data {
			field: Field::init(block, t.field_dim),
			pieces: Pieces::init(t.patterns),
			piece: None,
		};

		d
	}

	pub fn handle_event(&mut self, event: &Event)
	{
		match event {
			Event::KeyDown {
				keycode: Some(Keycode::N),
				..
			} => {
				let selected = self.pieces.spawn_piece(2);
				self.piece = PlayerPiece::new(&self.field, (0, 0), selected);
				println!("Added piece");

				if self.piece.is_none() {
					println!("Game Over.");
				}
			}
			Event::KeyDown {
				keycode: Some(Keycode::Left),
				..
			} => {
				if let Some(p) = &mut self.piece {
					p.move_piece(&self.field, Direction::LEFT);
					println!("Moved to left");
				}
			}
			Event::KeyDown {
				keycode: Some(Keycode::Right),
				..
			} => {
				if let Some(p) = &mut self.piece {
					p.move_piece(&self.field, Direction::RIGHT);
					println!("Moved to right");
				}
			}
			Event::KeyDown {
				keycode: Some(Keycode::Down),
				..
			} => {
				if let Some(p) = &mut self.piece {
					if p.move_piece(&self.field, Direction::DOWN) {
						return;
					}

                    let piece = p.output_blocks();
					self.field.add_pieces(&piece.0, &piece.1);

					let selected = self.pieces.spawn_piece(2);
					self.piece = PlayerPiece::new(&self.field, (0, 0), selected);
					println!("Added piece");

					if self.piece.is_none() {
						println!("Game Over.");
					}
				}

				println!("Moved down");
			}
			Event::KeyDown {
				keycode: Some(Keycode::Up),
				..
			} => {
				if let Some(p) = &mut self.piece {
					p.rotate(&self.field);
					println!("Rotated");
				}
			}

			_ => {}
		}
	}

	pub fn draw(&self, canvas: &mut WindowCanvas)
	{
		self.draw_field(canvas);
		self.draw_pieces(canvas);
		self.draw_piece(canvas);
	}
}