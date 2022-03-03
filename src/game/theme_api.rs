use sdl2::render::WindowCanvas;

use crate::TetrisState;
use crate::Error;

pub struct StateData<'a, 'b, 'c, 'd, 'e>
{
    pub canvas: &'e mut WindowCanvas,
    pub game: &'d mut TetrisState<'a, 'b, 'c>,
}
