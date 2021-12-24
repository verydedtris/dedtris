use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::game::pieces::Direction;

use self::field::Field;
use self::gen::Pieces;
use self::pieces::PlayerPiece;

pub use self::theme::Theme;

mod field;
mod gen;
mod pieces;
mod size;
mod theme;

pub struct Instance
{
	field: Field,
	pieces: Pieces,
	piece: Option<PlayerPiece>,
}

impl Instance
{
	pub fn init(dim: (u32, u32), t: Theme) -> Self
	{
		let mut d = Instance {
			field: Field::init(t.field_dim),
			pieces: Pieces::init(t.patterns),
			piece: None,
		};

		let p = size::new_resize(dim, d.field.field_dim);

		d.field.block_size = p.block_size;
		d.field.rect = p.field_rect;

		d.spawn_piece();

		d
	}

	pub fn handle_event(&mut self, event: &Event)
	{
		match event {
			Event::KeyDown {
				keycode: Some(x), ..
			} => {
				if let Some(p) = &mut self.piece {
					match x {
						Keycode::Left => {
							println!("Moved to left");
							p.move_piece(&self.field, Direction::LEFT);
						}

						Keycode::Right => {
							println!("Moved to right");
							p.move_piece(&self.field, Direction::RIGHT);
						}

						Keycode::Down => {
							println!("Moved down");

							if p.move_piece(&self.field, Direction::DOWN) {
								return;
							}

							self.push_piece();
							self.field.clear_lines();

							if !self.spawn_piece() {
								println!("Game Over.");
							}
						}

						Keycode::Up => {
							println!("Rotated");
							p.rotate(&self.field);
						}

						_ => (),
					}
				}
			}

			Event::Window {
				win_event: WindowEvent::Resized(w, h),
				..
			} => {
				let p = size::new_resize((*w as u32, *h as u32), self.field.field_dim);

				self.field.block_size = p.block_size;
				self.field.rect = p.field_rect;
			}

			_ => (),
		}
	}

	pub fn draw(&self, canvas: &mut WindowCanvas)
	{
		self.draw_field(canvas);
		self.draw_pieces(canvas);
		self.draw_piece(canvas);
	}
}

// -----------------------------------------------------------------------------
// Game actions
// -----------------------------------------------------------------------------

impl Instance
{
	fn spawn_piece(&mut self) -> bool
	{
		println!("Added piece");

		let selected = self.pieces.spawn_piece(2);
		self.piece = PlayerPiece::new(&self.field, (0, 0), selected);

		self.piece.is_some()
	}

	fn push_piece(&mut self)
	{
		println!("Pushed piece");

		let piece = self.piece.as_mut().unwrap().delta_blocks();
		self.field.add_pieces(&piece.0, &piece.1);
	}

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
			self.field.draw_blocks_delta(canvas, p.pos, &p.piece.blocks, &p.piece.colors);
		}
	}
}
