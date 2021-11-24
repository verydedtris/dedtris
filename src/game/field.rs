use sdl2::rect::Rect;

use super::super::component::*;

pub struct Field
{
	pub coords: Rect,
}

impl Component for Field
{
	fn init() -> Self
	{
		Field {
			coords: 
		}
	}

	fn draw(&self, canvas: &mut sdl2::render::WindowCanvas)
	{
		todo!()
	}

	fn handle_event(&mut self, event: &sdl2::event::Event)
	{
		todo!()
	}
}
