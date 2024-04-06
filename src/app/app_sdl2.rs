use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode};

use super::App;
use crate::{
    display,
    gui::sdl2_gui,
    interrupts::{set_button_left, set_button_middle, set_button_right},
};

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
                    Event::KeyDown {
                        keycode: Some(Keycode::Q),
                        ..
                    } => unsafe { set_button_left(cpu.as_mut()) },
                    Event::KeyDown {
                        keycode: Some(Keycode::W),
                        ..
                    } => unsafe { set_button_middle(cpu.as_mut()) },
                    Event::KeyDown {
                        keycode: Some(Keycode::E),
                        ..
                    } => unsafe { set_button_right(cpu.as_mut()) },
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
