use sdl2::pixels::Color;
use sdl2::rect::Point;

pub fn init(field_dim: (usize, usize)) -> (Vec<Point>, Vec<Color>, (usize, usize))
{
	let fb = Vec::new();
	let fc = Vec::new();
	let d = field_dim;

	(fb, fc, d)
}

pub fn lines_list(fd: (usize, usize), fb: &[Point]) -> Vec<i32>
{
	count_lines(fd.1, fb)
		.iter()
		.enumerate()
		.filter_map(|(i, l)| {
			if *l >= fd.0 as i32 {
				Some(i as i32)
			} else {
				None
			}
		})
		.collect()
}

pub fn clear_lines(fd: (usize, usize), fb: &mut Vec<Point>, fc: &mut Vec<Color>) -> Vec<i32>
{
	let lines = lines_list(fd, fb);

	let mut removed = 0;
	for i in 0..fb.len() {
		let i = i - removed;

		if let Some(ii) = lines.iter().position(|l| *l >= fb[i].y) {
			if fb[i].y == lines[ii] as i32 {
				fb.swap_remove(i);
				fc.swap_remove(i);
				removed += 1;
			} else {
				let shift = lines.len() - ii;
				fb[i].y += shift as i32;
			}
		}
	}

	lines
}

pub fn check_valid_pos(
	field_dim: (usize, usize),
	field_blocks: &[Point],
	pos: Point,
	blocks: &[Point],
) -> bool
{
	!blocks.iter().any(|block| {
		let b = Point::new(block.x + pos.x, block.y + pos.y);

		b.x < 0
			|| b.x >= field_dim.0 as i32
			|| b.y >= field_dim.1 as i32
			|| field_blocks.contains(&b)
	})
}

pub fn count_lines(height: usize, blocks: &[Point]) -> Vec<i32>
{
	let mut lines = vec![0i32; height];

	for b in blocks {
		lines[b.y as usize] += 1;
	}

	lines
}
