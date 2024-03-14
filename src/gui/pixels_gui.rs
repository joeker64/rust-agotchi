use crate::e0c6s46::CPU;
use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::LogicalSize, window::WindowBuilder};

const LCD_WIDTH: usize = 32;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub struct pixel_display {
    pub screen: pixels::Pixels,
    pub window: winit::window::Window,
}

impl pixel_display {
    pub fn create_display(event_loop: &winit::event_loop::EventLoop<()>) -> Self {
        let window = {
            let size = LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);
            WindowBuilder::new()
                .with_title("Hello Pixels")
                .with_inner_size(size)
                .with_resizable(false)
                .with_min_inner_size(size)
                .build(event_loop)
                .unwrap()
        };
        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WINDOW_WIDTH / 10, WINDOW_HEIGHT / 10, surface_texture)
        };
        let screen: pixels::Pixels;
        match pixels {
            Ok(data) => screen = data,
            Err(_) => panic!("ERROR"),
        }
        return pixel_display { window, screen };
    }
    pub fn update_display(&mut self, cpu: &CPU) {
        let frame = self.screen.frame_mut();
        let mut p_row: u16 = 0;
        for (position, pixel) in cpu.display.lcd_matrix.iter().enumerate() {
            if position % LCD_WIDTH == 0 {
                p_row += 1;
            }
            if *pixel == 1 {
                frame[(((position % LCD_WIDTH * 4) + 40) + ((p_row as usize + 5) * 320))] = 0xff;
                // frame[(position * 4 + 121) + ((p_row - 1) * 320) as usize] = 0x00;
                // frame[(position * 4 + 122) + ((p_row - 1) * 320) as usize] = 0x00;
                frame[(position % LCD_WIDTH * 4 + 43) + ((p_row as usize + 5) * 320)] = 0xff;
            } else {
                // frame[(position * 4 + 120) + ((p_row - 1) * 320) as usize] = 0xff;
                // frame[(position * 4 + 121) + ((p_row - 1) * 320) as usize] = 0x00;
                // frame[(position * 4 + 122) + ((p_row - 1) * 320) as usize] = 0x00;
                frame[(position % LCD_WIDTH * 4 + 43) + ((p_row as usize + 5) * 320)] = 0x00;
            }
        }
    }
}
