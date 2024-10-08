use super::App;
use crate::{
    gui::pixels_gui::{self},
    CPU,
};
use winit::{
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
};
use winit_input_helper::WinitInputHelper;

pub struct PixelApp {
    event_loop: winit::event_loop::EventLoop<()>,
}

impl App for PixelApp {
    fn new() -> Self {
        return PixelApp {
            event_loop: EventLoop::new(),
        };
    }
    fn run(self, mut cpu: Box<CPU>, rom: Vec<u16>) {
        let mut input = WinitInputHelper::new();
        let mut display = pixels_gui::pixel_display::create_display(&self.event_loop);
        self.event_loop.run(move |event, _, control_flow| {
            if let Event::RedrawRequested(_) = event {
                display.update_display(cpu.as_ref());
                display.screen.render();
            }
            if input.update(&event) {
                if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                unsafe {
                    cpu.as_mut().step_cpu(&rom);
                }
            }
            display.window.request_redraw();
        });
    }
}
