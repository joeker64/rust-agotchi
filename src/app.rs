pub mod app_pixels;

use crate::e0c6s46::CPU;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub trait App {
    fn new() -> Self;
    fn run(self, cpu: Box<CPU>, rom: Vec<u16>);
}
