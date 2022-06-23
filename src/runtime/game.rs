use std::{path::Path, time::Instant};

use log::*;
use sdl2::rect::Point;

use super::{
	drawer, profile_api,
	profile_api::Profile,
	state::{
		self, field,
		flags::{self, Flag},
		pieces, Direction,
	},
	Framework, Piece,
};
use crate::error::Error;

pub struct Game<'a, 'b, 'd, 'e, 'f, 'g>
{
	pub state: state::TetrisState,
	pub rend:  drawer::Renderer<'e>,
	pub fw:    Framework<'a, 'b, 'd, 'e, 'f, 'g>,
}

pub fn init_game<'a, 'b, 'c, 'd, 'e, 'f>(
	fw: Framework<'a, 'b, 'c, 'd, 'e, 'f>, win_dim: (u32, u32), t: Profile,
) -> Result<Game<'a, 'b, 'c, 'd, 'e, 'f>, Error>
{
	let rend = drawer::init_renderer(
		&fw.tex_maker,
		win_dim,
		Path::new("Profiles/default/template.bmp"),
	)?;

	let state = state::init_game(
		t.field_dim,
		t.start_piece,
		t.piece_hold_enabled,
		t.piece_tick,
	)?;

	let mut game = Game { state, rend, fw };
	game.refresh_piece_view(t.piece_view_size)?;

	Ok(game)
}

impl Game<'_, '_, '_, '_, '_, '_>
{
	pub fn request_piece(&mut self) -> Result<Piece, Error>
	{
		let state = &mut self.state;
		let fw = &self.fw;

		let t = profile_api::call_lua("spawn_piece", state, fw)?;
		profile_api::parse_pattern(t)
	}

	pub fn refresh_piece_view(&mut self, size: usize) -> Result<(), Error>
	{
		let mut v = Vec::with_capacity(size);

		while v.len() < size
		{
			let p = self.request_piece()?;
			v.push(p);
		}

		let state = &mut self.state;
		state.piece_queue = v;
		Ok(())
	}

	pub fn spawn_piece(&mut self) -> Result<bool, Error>
	{
		info!("Respawning piece.");

		let p = self.request_piece()?;
		Ok(self.state.spawn_piece(p))
	}

	pub fn place_piece<'a>(&mut self) -> Result<bool, Error>
	{
		info!("Placing piece.");

		let state = &mut self.state;
		let fw = &self.fw;

		// Add blocks to state
		{
			let fb = &mut state.field_blocks;
			let fc = &mut state.field_colors;
			let p = &state.player_piece;
			let pp = state.player_pos;

			fb.extend(p.blocks.iter().map(|b| Point::new(b.x + pp.x, b.y + pp.y)));
			fc.extend(state.player_piece.colors.iter());
		}

		profile_api::call_lua::<()>("on_place", state, fw)?;

		state.pieces_placed += 1;

		self.spawn_piece()
	}

	pub fn swap(&mut self) -> Result<(), Error>
	{
		let state = &mut self.state;

		if !flags::check(&state.flags, Flag::PieceHoldEnabled)
		{
			return Ok(());
		}

		let fd = state.field_size;
		let pl = state.player_pos;

		if let Some(piece) = &mut state.piece_swap
		{
			if field::check_valid_pos(fd, &state.field_blocks, pl, &piece.blocks)
			{
				std::mem::swap(piece, &mut state.player_piece);

				state.player_proj =
					pieces::project(fd, &state.field_blocks, pl, &state.player_piece.blocks);
			}
		}
		else
		{
			let p = self.request_piece()?;

			let state = &mut self.state;
			let p = state.push_piece(p);

			state.piece_swap = Some(
				if field::check_valid_pos(fd, &state.field_blocks, pl, &p.blocks)
				{
					std::mem::replace(&mut state.player_piece, p)
				}
				else
				{
					p
				},
			);

			state.player_proj =
				pieces::project(fd, &state.field_blocks, pl, &state.player_piece.blocks);
		}

		Ok(())
	}

	pub fn drop(&mut self) -> Result<bool, Error>
	{
		info!("Dropping piece.");

		let state = &mut self.state;
		state.player_pos.y = state.player_proj;

		self.place_piece()
	}

	pub fn tick_update(&mut self) -> Result<(), Error>
	{
		let state = &mut self.state;

		if state.player_tick_time <= Instant::now()
		{
			state.player_tick_time += state.player_tick_dur;
			self.move_piece_down()?;
		}

		Ok(())
	}

	pub fn move_piece_down<'a>(&mut self) -> Result<bool, Error>
	{
		let state = &mut self.state;

		if state.move_piece(Direction::DOWN)
		{
			return Ok(true);
		}

		self.place_piece()
	}
}
