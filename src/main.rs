mod app;
mod e0c6s46;
mod gui;

use app::App;

use crate::e0c6s46::*;

fn main() {
    // let app = app::app_pixels::PixelApp::new();
    let app = app::app_sdl2::Sdl2App::new();
    let cpu: CPU = create_e06s46_cpu();
    let mut rom: Vec<u16> = Vec::new();
    match read_rom("../tama.b") {
        Ok(data) => rom = data,
        Err(err) => println!("Error: {}", err),
    }
    app.run(Box::new(cpu), rom);
    // env_logger::init();
}
