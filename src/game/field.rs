use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Field
{
	pub rect: Rect,

	pub blocks: Vec<(i32, i32)>,
	pub colors: Vec<Color>,

	pub field_dim: (usize, usize),
	pub block_size: u32,
}

impl Field
{
	pub fn init(field_dim: (usize, usize)) -> Self
	{
		Field {
			rect: Rect::new(0, 0, 0, 0),
			blocks: Vec::new(),
			colors: Vec::new(),
			field_dim,
			block_size: 0,
		}
	}
}

pub fn add_pieces(field: &mut Field, blocks: &[(i32, i32)], color: &[Color])
{
	debug_assert!(check_valid(field, blocks));
	debug_assert_eq!(blocks.len(), color.len());

	field.colors.extend(color);
	field.blocks.extend_from_slice(blocks);
}

pub fn count_lines(field: &Field) -> Vec<i32>
{
	let mut lines = vec![0i32; field.field_dim.1];

	for b in &field.blocks {
		lines[b.1 as usize] += 1;
	}

	lines
}

pub fn lines_list(field: &Field) -> Vec<i32>
{
	count_lines(field)
		.iter()
		.enumerate()
		.filter_map(|(i, l)| {
			if *l >= field.field_dim.0 as i32 {
				Some(i as i32)
			} else {
				None
			}
		})
		.collect()
}

pub fn clear_lines(field: &mut Field) -> bool
{
	let lines = lines_list(field);

	let mut removed = 0;
	for i in 0..field.blocks.len() {
		let i = i - removed;

		if let Some(ii) = lines.iter().position(|l| *l >= field.blocks[i].1) {
			if field.blocks[i].1 == lines[ii] as i32 {
				field.blocks.swap_remove(i);
				field.colors.swap_remove(i);
				removed += 1;
			} else {
				let shift = lines.len() - ii;
				field.blocks[i].1 += shift as i32;
			}
		}
	}

	!lines.is_empty()
}

pub fn check_valid_pos(field: &Field, pos: (i32, i32), blocks: &[(i32, i32)]) -> bool
{
	!blocks.iter().any(|block| {
		let b = (block.0 + pos.0, block.1 + pos.1);

		field.blocks.contains(&b)
			|| b.0 < 0 || b.0 >= field.field_dim.0 as i32
			|| b.1 >= field.field_dim.1 as i32
	})
}

pub fn check_valid(field: &Field, blocks: &[(i32, i32)]) -> bool
{
	check_valid_pos(field, (0, 0), blocks)
}
