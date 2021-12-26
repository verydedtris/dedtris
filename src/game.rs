use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::game::pieces::Direction;

use self::drawer::DrawCache;
use self::field::Field;
use self::gen::Pieces;
use self::pieces::PlayerPiece;

use self::size::ResizePattern;
pub use self::theme::Theme;

mod drawer;
mod field;
mod gen;
mod pieces;
mod size;
mod theme;

fn color_to_u32(c: Color) -> u32
{
	0u32 | c.r as u32 | (c.g as u32) << 8 | (c.b as u32) << 16 | (c.a as u32) << 24
}

fn u32_to_color(n: u32) -> Color
{
	Color::RGBA(
		(n & 0xFF) as u8,
		(n & 0xFF00) as u8,
		(n & 0xFF0000) as u8,
		(n & 0xFF000000) as u8,
	)
}

pub struct Instance
{
	field: Field,
	pieces: Pieces,
	piece: Option<PlayerPiece>,

	draw_cache: DrawCache,
}

impl Instance
{
	pub fn init(dim: (u32, u32), t: Theme) -> Self
	{
		let mut inst = Instance {
			field: Field::init(t.field_dim),
			pieces: Pieces::init(t.patterns),
			piece: None,
			draw_cache: DrawCache::init(),
		};

		let r = inst.new_resize(dim);
		inst.draw_cache.set_size(r);

		inst.piece = inst.spawn_piece();
		if let Some(p) = &inst.piece {
			inst.draw_cache.set_player_blocks(p.pos, &p.piece.blocks, &p.piece.colors);
		}

		inst
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
							self.draw_cache.set_player_blocks(p.pos, &p.piece.blocks, &p.piece.colors);
						}

						Keycode::Right => {
							println!("Moved to right");
							p.move_piece(&self.field, Direction::RIGHT);
							self.draw_cache.set_player_blocks(p.pos, &p.piece.blocks, &p.piece.colors);
						}

						Keycode::Down => {
							println!("Moved down");

							if p.move_piece(&self.field, Direction::DOWN) {
								self.draw_cache.set_player_blocks(p.pos, &p.piece.blocks, &p.piece.colors);
								return;
							}

							let piece = p.delta_blocks();
							self.field.add_pieces(&piece.0, &piece.1);

							if self.field.clear_lines() {
								self.draw_cache.clear_field_blocks();
							}

							self.draw_cache
								.set_field_blocks(&self.field.blocks, &self.field.colors);

							let selected = self.pieces.bag_piece();
							let new_p = PlayerPiece::new(&self.field, (0, 0), selected);

							if new_p.is_none() {
								println!("Game Over.");
							}

							*p = new_p.unwrap();
							self.draw_cache.set_player_blocks(p.pos, &p.piece.blocks, &p.piece.colors);
						}

						Keycode::Up => {
							println!("Rotated");
							p.rotate(&self.field);
							self.draw_cache.set_player_blocks(p.pos, &p.piece.blocks, &p.piece.colors);
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
				self.draw_cache.set_size(p);
			}

			_ => (),
		}
	}

	pub fn draw(&self, canvas: &mut WindowCanvas)
	{
		self.draw_cache.draw_field(canvas);
		self.draw_cache.draw_player(canvas);
	}
}

// -----------------------------------------------------------------------------
// Game actions
// -----------------------------------------------------------------------------

impl Instance
{
	fn new_resize(&self, win_target: (u32, u32)) -> ResizePattern
	{
		size::new_resize(win_target, self.field.field_dim)
	}

	fn spawn_piece(&mut self) -> Option<PlayerPiece>
	{
		let selected = self.pieces.bag_piece();
		PlayerPiece::new(&self.field, (0, 0), selected)
	}
}
