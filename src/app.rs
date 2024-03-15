pub mod app_pixels;
pub mod app_sdl2;
use crate::e0c6s46::CPU;

pub trait App {
    fn new() -> Self;
    fn run(self, cpu: Box<CPU>, rom: Vec<u16>);
}
