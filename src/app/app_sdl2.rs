use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode};

use super::App;
use crate::{display, gui::sdl2_gui};

pub struct Sdl2App {
    event_loop: sdl2::EventPump,
    display: sdl2_gui::Sdl2Display,
}

impl App for Sdl2App {
    fn new() -> Self {
        let display = sdl2_gui::Sdl2Display::new();
        Self {
            event_loop: display.context.event_pump().unwrap(),
            display,
        }
    }
    fn run(mut self, mut cpu: Box<crate::CPU>, rom: Vec<u16>) {
        'running: loop {
            for event in self.event_loop.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            unsafe {
                cpu.as_mut().step_cpu(&rom);
            }
            self.display.update_display(cpu.as_ref());
        }
    }
}
