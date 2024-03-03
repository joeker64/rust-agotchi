extern crate nalgebra as na;
extern crate sdl2;
use crate::e0c6s46::*;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const ICON_NUMBER: u8 = 8;
const LCD_HEIGHT: usize = 16;
const LCD_WIDTH: usize = 32;
const PIXEL_SIZE: u32 = 9;
const SEGMENT_POSITION: [u8; 40] = [
    0, 1, 2, 3, 4, 5, 6, 7, 32, 8, 9, 10, 11, 12, 13, 14, 15, 33, 34, 35, 31, 30, 29, 28, 27, 26,
    25, 24, 36, 23, 22, 21, 20, 19, 18, 17, 16, 37, 38, 39,
];
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

pub struct display_values {
    pub terminal: u16,
    pub segment: u16,
    pub icon_buffer: [u16; ICON_NUMBER as usize],
    pub lcd_matrix: na::SMatrix<u16, LCD_WIDTH, LCD_HEIGHT>,
}

pub fn init_display_values() -> display_values {
    return display_values {
        terminal: 0,
        segment: 0,
        icon_buffer: [0; ICON_NUMBER as usize],
        lcd_matrix: na::SMatrix::from_element(0),
    };
}

pub fn create_display(
    event_loop: &EventLoop<()>,
) -> Result<(pixels::Pixels, winit::window::Window), Error> {
    let window = {
        let size = LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_resizable(false)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WINDOW_WIDTH / 10, WINDOW_HEIGHT / 10, surface_texture)?
    };

    Ok((pixels, window))
}

pub unsafe fn update_display(frame: &mut [u8], cpu: &mut super::CPU) {
    // frame[(2 * 4 + 120) + ((3 - 1) * 320) as usize] = 0xff;
    // frame[(2 * 4 + 121) + ((3 - 1) * 320) as usize] = 0xff;
    // frame[(2 * 4 + 122) + ((3 - 1) * 320) as usize] = 0x00;
    // frame[(2 * 4 + 123) + ((3 - 1) * 320) as usize] = 0xff;
    let mut p_row: u16 = 0;
    for (position, pixel) in (*cpu).display.lcd_matrix.iter().enumerate() {
        if position % LCD_WIDTH == 0 {
            p_row += 1;
        }
        // println!(
        //     "row: {}, pos: {}, value: {}",
        //     p_row,
        //     position % LCD_WIDTH + 1,
        //     pixel
        // );
        if *pixel == 1 {
            frame[(((position%LCD_WIDTH * 4) + 40) + ((p_row as usize + 5) * 320))] = 0xff;
            // frame[(position * 4 + 121) + ((p_row - 1) * 320) as usize] = 0x00;
            // frame[(position * 4 + 122) + ((p_row - 1) * 320) as usize] = 0x00;
            frame[(position%LCD_WIDTH  * 4 + 43) + ((p_row as usize + 5) * 320)] = 0xff;
        } else {
            // frame[(position * 4 + 120) + ((p_row - 1) * 320) as usize] = 0xff;
            // frame[(position * 4 + 121) + ((p_row - 1) * 320) as usize] = 0x00;
            // frame[(position * 4 + 122) + ((p_row - 1) * 320) as usize] = 0x00;
            frame[(position%LCD_WIDTH  * 4 + 43) + ((p_row as usize + 5) * 320)] = 0x00;
        }
    }
}

unsafe fn set_lcd_matrix_values(
    cpu: *mut super::CPU,
    x_coordinate: u16,
    y_coordinate: u16,
    value: u16,
) {
    (*cpu).display.lcd_matrix[(x_coordinate as usize, y_coordinate as usize)] = value;
}

unsafe fn set_lcd_icon_values(cpu: *mut super::CPU, pointer: u16, value: u16) {
    (*cpu).display.icon_buffer[pointer as usize] = value;
}

pub fn set_lcd(cpu: *mut super::CPU, pointer: u16, value: u16) {
    let mut x = 0;

    let seg = ((pointer & 0x7F) >> 1);
    let com0 = (((pointer & 0x80) >> 7) * 8 + (pointer & 0x1) * 4);

    for x in 0..4 {
        unsafe {
            set_lcd_values(cpu, seg, com0 + 1, (value >> x) & 0x1);
        }
    }
}

unsafe fn set_lcd_values(cpu: *mut super::CPU, segment: u16, component: u16, value: u16) {
    if SEGMENT_POSITION[segment as usize] < LCD_WIDTH as u8 {
        set_lcd_matrix_values(
            cpu,
            SEGMENT_POSITION[segment as usize] as u16,
            component,
            value,
        );
        return;
    }

    if (segment == 8) && (component < 4) {
        set_lcd_icon_values(cpu, component, value);
        return;
    }

    if (segment == 28) && (component >= 12) {
        set_lcd_icon_values(cpu, component - 8, value);
    }
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
