use sdl2::pixels::Color;
use sdl2::rect::Point;

use super::field;
use super::gen;

pub enum Direction {
    DOWN,
    LEFT,
    RIGHT,
}

// -----------------------------------------------------------------------------
// Movable Piece
// -----------------------------------------------------------------------------

pub fn init(
	p: gen::Piece,
	field_dim: (usize, usize),
	field_blocks: &[Point],
) -> Option<(Vec<Point>, Vec<Color>, usize, Point, i32)>
{
	spawn_new(p, field_dim, field_blocks)
}

pub fn spawn_new(
	p: gen::Piece,
	field_dim: (usize, usize),
	field_blocks: &[Point],
) -> Option<(Vec<Point>, Vec<Color>, usize, Point, i32)>
{
	let pb = p.blocks;
	let pc = p.colors;
	let pd = p.dim;
	let pp = Point::new(((field_dim.0 - pd) / 2) as i32, 0);

	if !field::check_valid_pos(field_dim, field_blocks, pp, &pb) {
		return None;
	}

	let pj = pproject(field_dim, field_blocks, pp, &pb);

	Some((pb, pc, pd, pp, pj))
}

pub fn pproject(
	field_dim: (usize, usize),
	field_blocks: &[Point],
	pos: Point,
	blocks: &[Point],
) -> i32
{
	let mut y = pos.y + 1;

	while field::check_valid_pos(field_dim, field_blocks, Point::new(pos.x, y), blocks) {
		y += 1;
	}

	y - 1
}

pub fn pmove_piece(
	fd: (usize, usize),
	fb: &[Point],
	pl: Point,
	pb: &[Point],
	d: Direction,
) -> Option<(Point, i32)>
{
	let new_pl = match d {
		Direction::LEFT => Point::new(pl.x - 1, pl.y),
		Direction::RIGHT => Point::new(pl.x + 1, pl.y),
		Direction::DOWN => Point::new(pl.x, pl.y + 1),
	};

	if field::check_valid_pos(fd, fb, new_pl, pb) {
		let p = pproject(fd, fb, new_pl, pb);
		Some((new_pl, p))
	} else {
		None
	}
}
