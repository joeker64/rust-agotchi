mod e0c6s46;

use crate::e0c6s46::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use std::{thread, time};
fn main() {
    let mut screen: sdl2::render::Canvas<sdl2::video::Window>;
    match (e0c6s46::display::create_display()){
        Ok(data) => screen = data,
        Err(err) => println!("Error: {}", err),
    };
    unsafe{
        e0c6s46::run_cpu();
    }
    thread::sleep(time::Duration::from_secs(2));
}