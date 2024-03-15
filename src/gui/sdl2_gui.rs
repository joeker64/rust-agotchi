use sdl2::{pixels::Color, rect::Rect};

use crate::CPU;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const PIXEL_SIZE: u32 = 9;
const LCD_HEIGHT: usize = 16;
const LCD_WIDTH: usize = 32;

pub struct Sdl2Display {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub window_context: sdl2::VideoSubsystem,
    pub context: sdl2::Sdl,
}

impl Sdl2Display {
    pub fn new() -> Self {
        let sdl2_context = sdl2::init().unwrap();
        let window_context = sdl2_context.video().unwrap();
        let window = window_context
            .window("Rust-agotchi", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        // canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.present();
        return Self {
            canvas,
            window_context,
            context: sdl2_context,
        };
    }
    pub fn update_display(&mut self, cpu: &CPU) {
        self.canvas.clear();
        let mut p_row: u16 = 0;

        for (position, pixel) in (*cpu).display.lcd_matrix.iter().enumerate() {
            if position % LCD_WIDTH == 0 {
                p_row += 1;
            }
            // println!("row: {}, pos: {}, value: {}",p_row, position % LCD_WIDTH + 1, pixel);
            if *pixel == 1 {
                self.canvas.set_draw_color(Color::RGB(0, 0, 128));
                self.canvas.fill_rect(Rect::new(
                    ((position % LCD_WIDTH) * 10 + 50 + 50) as i32,
                    ((p_row - 1) * 10 + 50 + 50) as i32,
                    PIXEL_SIZE,
                    PIXEL_SIZE,
                ));
            } else {
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                self.canvas.fill_rect(Rect::new(
                    ((position % LCD_WIDTH) * 10 + 50 + 50) as i32,
                    ((p_row - 1) * 10 + 50 + 50) as i32,
                    PIXEL_SIZE,
                    PIXEL_SIZE,
                ));
            }
        }
        self.canvas.present();
    }
}
