use crate::display::set_lcd;
use crate::e0c6s46::interrupts::get_io;
use crate::e0c6s46::interrupts::set_io;
use crate::e0c6s46::CPU;

pub const RAM_SIZE: u16 = 0x280;
pub const RAM_DISPLAY_1_SIZE: u16 = 0x050;
pub const RAM_DISPLAY_1_ADDR: u16 = 0xE00;
pub const RAM_DISPLAY_2_SIZE: u16 = 0x050;
pub const RAM_DISPLAY_2_ADDR: u16 = 0xE80;
pub const RAM_IO_SIZE: u16 = 0x080;
pub const RAM_IO_ADDR: u16 = 0xF00;
pub const RAM_TOTAL_SIZE: u16 = RAM_SIZE + RAM_DISPLAY_1_SIZE + RAM_DISPLAY_2_SIZE + RAM_IO_SIZE;

pub unsafe fn set_memory(cpu: *mut CPU, pointer: u16, value: u16) {
    // if pointer == 0x06E{
    //     println!("SET HERE! VALUE: {:x}", value);
    // }
    if pointer < RAM_SIZE {
        (*cpu).memory[pointer as usize] = value;
    } else if ((RAM_DISPLAY_1_ADDR + RAM_DISPLAY_1_SIZE) > pointer)
        && (pointer >= RAM_DISPLAY_1_ADDR)
    {
        (*cpu).memory[(pointer - RAM_DISPLAY_1_ADDR + RAM_SIZE) as usize] = value;
        set_lcd(cpu, pointer, value);
    } else if ((RAM_DISPLAY_2_ADDR + RAM_DISPLAY_2_SIZE) > pointer)
        && (pointer >= RAM_DISPLAY_2_ADDR)
    {
        (*cpu).memory[(pointer - RAM_DISPLAY_2_ADDR + RAM_SIZE + RAM_DISPLAY_1_SIZE) as usize] =
            value;
        set_lcd(cpu, pointer, value);
    } else if ((RAM_IO_ADDR + RAM_IO_SIZE) > pointer) && (pointer >= RAM_IO_ADDR) {
        (*cpu).memory[(pointer - RAM_IO_ADDR + RAM_SIZE + RAM_DISPLAY_1_SIZE + RAM_DISPLAY_2_SIZE)
            as usize] = value;
        set_io(cpu, pointer, value);
    } else {
        println!("ERROR SET MEM, VALUE: {}", pointer);
    }
}

pub unsafe fn get_memory(cpu: *mut CPU, pointer: u16) -> u16 {
    if pointer < RAM_SIZE {
        return (*cpu).memory[pointer as usize];
    } else if ((RAM_DISPLAY_1_ADDR + RAM_DISPLAY_1_SIZE) > pointer)
        && (pointer >= RAM_DISPLAY_1_ADDR)
    {
        return (*cpu).memory[(pointer - RAM_DISPLAY_1_ADDR + RAM_SIZE) as usize];
    } else if ((RAM_DISPLAY_2_ADDR + RAM_DISPLAY_2_SIZE) > pointer)
        && (pointer >= RAM_DISPLAY_2_ADDR)
    {
        return (*cpu).memory
            [(pointer - RAM_DISPLAY_2_ADDR + RAM_SIZE + RAM_DISPLAY_1_SIZE) as usize];
    } else if ((RAM_IO_ADDR + RAM_IO_SIZE) > pointer) && (pointer >= RAM_IO_ADDR) {
        return get_io(cpu, pointer);
    }
    println!("ERROR GET MEM");
    return 0;
}
