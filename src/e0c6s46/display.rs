extern crate sdl2;
use crate::e0c6s46::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn create_display() -> Result<sdl2::render::Canvas<sdl2::video::Window>, String>{
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    return Ok(canvas);
}
pub fn update_display(){

}

pub fn set_lcd_matrix_values(cpu: *mut super::CPU, pointer: u16, value: u16){

}

pub fn set_lcd_icon_values(cpu: *mut super::CPU, pointer: u16, value: u16){

}

// pub fn test_sdl2() -> Result<(), String>{
//     let sdl_context = sdl2::init()?;
//     let video_subsystem = sdl_context.video()?;

//     let window = video_subsystem
//         .window("rust-sdl2 demo: Video", 800, 600)
//         .position_centered()
//         .opengl()
//         .build()
//         .map_err(|e| e.to_string())?;

//     let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

//     canvas.set_draw_color(Color::RGB(255, 0, 0));
//     canvas.clear();
//     canvas.present();
//     let mut event_pump = sdl_context.event_pump()?;
//     let mut x: bool = true;
//     while x{
//         for event in event_pump.poll_iter() {
//             match event {

//                 Event::Quit { .. }
//                 | Event::KeyDown {
//                     keycode: Some(Keycode::Escape),
//                     ..
//                 } => x = false,
//                 _ => break,
//             };
//     }

//     }
//     canvas.clear();
//     canvas.present();
//     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
//     // The rest of the game loop goes here...
//     return Ok(())
// }