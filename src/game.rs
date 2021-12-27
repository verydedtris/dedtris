use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::game::pieces::Direction;

use self::drawer::DrawCache;
use self::field::Field;
use self::gen::Pieces;
use self::pieces::PlayerPiece;

pub use self::theme::Theme;

mod drawer;
mod field;
mod gen;
mod pieces;
mod size;
mod theme;

// -----------------------------------------------------------------------------
// Error
// -----------------------------------------------------------------------------

pub struct GameError
{
	err: String,
}

impl std::fmt::Display for GameError
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "{}", self.err)
	}
}

impl From<&str> for GameError
{
	fn from(err: &str) -> Self
	{
		GameError {
			err: err.to_string(),
		}
	}
}

// -----------------------------------------------------------------------------
// Game
// -----------------------------------------------------------------------------

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
	piece: PlayerPiece,

	draw_cache: DrawCache,
}

impl Instance
{
	pub fn init(dim: (u32, u32), t: Theme) -> Result<Self, GameError>
	{
		let field = Field::init(t.field_dim);
		let mut pieces = Pieces::init(t.patterns);
		let mut draw_cache = DrawCache::init();

		set_layout_size(&mut draw_cache, &field, dim);

		let piece = match spawn_piece(&mut draw_cache, &mut pieces, &field) {
			Some(x) => x,
			_ => return Err(GameError::from("Piece couldn't be spawned in.")),
		};

		Ok(Self {
			field,
			pieces,
			piece,
			draw_cache,
		})
	}

	pub fn handle_event(&mut self, event: &Event)
	{
		match event {
			Event::KeyDown {
				keycode: Some(x), ..
			} => match x {
				Keycode::Left => {
					move_piece(
						&mut self.piece,
						&mut self.draw_cache,
						&self.field,
						Direction::LEFT,
					);
				}

				Keycode::Right => {
					move_piece(
						&mut self.piece,
						&mut self.draw_cache,
						&self.field,
						Direction::RIGHT,
					);
				}

				Keycode::Down => {
					let respawn = move_piece(
						&mut self.piece,
						&mut self.draw_cache,
						&self.field,
						Direction::DOWN,
					);

					if respawn {
						return;
					}

					place_piece(
						std::mem::take(&mut self.piece),
						&mut self.field,
						&mut self.draw_cache,
					);

					if let Some(p) =
						spawn_piece(&mut self.draw_cache, &mut self.pieces, &self.field)
					{
						self.piece = p;
					} else {
						println!("Game Over.");
					}
				}

				Keycode::Up => {
					rotate(&mut self.piece, &mut self.draw_cache, &self.field);
				}

				_ => (),
			},

			Event::Window {
				win_event: WindowEvent::Resized(w, h),
				..
			} => {
				set_layout_size(&mut self.draw_cache, &self.field, (*w as u32, *h as u32));
			}

			_ => (),
		}
	}

	pub fn draw(&self, canvas: &mut WindowCanvas)
	{
		drawer::draw_field(&self.draw_cache, canvas);
		drawer::draw_player(&self.draw_cache, canvas);
	}
}

// -----------------------------------------------------------------------------
// Game actions
// -----------------------------------------------------------------------------

fn spawn_piece(cache: &mut DrawCache, pieces: &mut Pieces, field: &Field) -> Option<PlayerPiece>
{
	let selected = gen::get_next_piece(pieces);
	let piece = PlayerPiece::new(&field, (0, 0), selected)?;

	drawer::set_player_blocks(cache, piece.pos, &piece.piece.blocks, &piece.piece.colors);

	Some(piece)
}

fn move_piece(p: &mut PlayerPiece, cache: &mut DrawCache, field: &Field, d: Direction) -> bool
{
	let b = pieces::move_piece(p, &field, d);

	if b {
		drawer::set_player_blocks(cache, p.pos, &p.piece.blocks, &p.piece.colors);
	}

	b
}

fn rotate(p: &mut PlayerPiece, cache: &mut DrawCache, field: &Field) -> bool
{
	let b = pieces::rotate(p, &field);

	if b {
		drawer::set_player_blocks(cache, p.pos, &p.piece.blocks, &p.piece.colors);
	}

	b
}

fn place_piece(p: PlayerPiece, field: &mut Field, cache: &mut DrawCache)
{
	let blocks = p.piece.move_delta(p.pos);
	let colors = p.piece.colors;

	field.add_pieces(&blocks, &colors);

	field.clear_lines();

	drawer::set_field_blocks(cache, &field.blocks, &field.colors);
}

fn set_layout_size(cache: &mut DrawCache, field: &Field, dim: (u32, u32))
{
	let r = size::new_resize(dim, field.field_dim);
	drawer::set_size(cache, r);
}
