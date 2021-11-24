use sdl2::video::Window;
use sdl2::render::WindowCanvas;
use sdl2::event::Event;

pub trait Component {
    fn init(window: &Window) -> Self;
    fn handle_event(&mut self, event: &Event);
    fn draw(&self, canvas: &mut WindowCanvas);
}
