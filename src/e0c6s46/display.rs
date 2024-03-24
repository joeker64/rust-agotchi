extern crate nalgebra as na;
extern crate sdl2;

const ICON_NUMBER: u8 = 8;
const LCD_HEIGHT: usize = 16;
const LCD_WIDTH: usize = 32;
// const PIXEL_SIZE: u32 = 9;
const SEGMENT_POSITION: [u8; 40] = [
    0, 1, 2, 3, 4, 5, 6, 7, 32, 8, 9, 10, 11, 12, 13, 14, 15, 33, 34, 35, 31, 30, 29, 28, 27, 26,
    25, 24, 36, 23, 22, 21, 20, 19, 18, 17, 16, 37, 38, 39,
];

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
    let seg = ((pointer & 0x7F) >> 1);
    let com0 = (((pointer & 0x80) >> 7) * 8 + (pointer & 0x1) * 4);

    for x in 0..4 {
        unsafe {
            set_lcd_values(cpu, seg, com0 + x, (value >> x) & 0x1);
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
