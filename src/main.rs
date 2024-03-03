mod e0c6s46;

use e0c6s46::display::update_display;
use winit::{
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
};
use winit_input_helper::WinitInputHelper;

use crate::e0c6s46::*;

fn main() {
    // env_logger::init();
    let event_loop = EventLoop::new();
    let mut screen: pixels::Pixels;
    let mut window: winit::window::Window;
    let mut input = WinitInputHelper::new();

    match (e0c6s46::display::create_display(&event_loop)) {
        Ok(data) => (screen, window) = data,
        Err(err) => panic!("Error: {}", err),
    };

    let mut cpu: CPU = create_e06s46_cpu();
    let mut rom: Vec<u16> = Vec::new();
    match read_rom("tama.b") {
        Ok(data) => rom = data,
        Err(err) => println!("Error: {}", err),
    }

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            unsafe {
                update_display(screen.frame_mut(), &mut cpu);
            }
            // let frame = screen.frame_mut();
            // frame[0] = 0xff;
            // frame[3] = 0xff;
            screen.render();
        }
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            unsafe {
                cpu.step_cpu(&rom);
            }
            window.request_redraw();
        }
    });
}
