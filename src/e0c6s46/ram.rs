const RAM_SIZE: u16 = 0x280;
const RAM_DISPLAY_1_SIZE: u16 = 0x050;
const RAM_DISPLAY_1_ADDR: u16 = 0xE00;
const RAM_DISPLAY_2_SIZE: u16 = 0x050;
const RAM_DISPLAY_2_ADDR: u16 = 0xE80;
const RAM_IO_SIZE: u16 = 0x080;
const RAM_IO_ADDR: u16 = 0xF00;
pub const RAM_TOTAL_SIZE: u16 = RAM_SIZE + RAM_DISPLAY_1_SIZE + RAM_DISPLAY_2_SIZE + RAM_IO_SIZE;

pub fn set_memory(mut memory: [u16; RAM_TOTAL_SIZE as usize], pointer: u16, value: u16) -> [u16; RAM_TOTAL_SIZE as usize]{
    if pointer < RAM_SIZE {
        memory[pointer as usize] = value;
        return memory;
    }
    else if ((RAM_DISPLAY_1_ADDR + RAM_DISPLAY_1_SIZE) < pointer) && (pointer <= RAM_DISPLAY_1_ADDR){
        memory[(pointer - RAM_DISPLAY_1_ADDR) as usize] = value;
        return memory;
    }
    else if ((RAM_DISPLAY_2_ADDR + RAM_DISPLAY_2_SIZE) < pointer) && (pointer <= RAM_DISPLAY_2_ADDR){
        memory[(pointer - RAM_DISPLAY_2_ADDR) as usize] = value;
        return memory;
    }
    else if ((RAM_IO_ADDR + RAM_IO_SIZE) < pointer) && (pointer <= RAM_IO_ADDR){
        memory[(pointer - RAM_IO_ADDR) as usize] = value;
        return memory;
    }
    return memory;
}

pub fn get_memory(memory: [u16; RAM_TOTAL_SIZE as usize], pointer: u16) -> u16 {
    if pointer < RAM_SIZE {
        return memory[pointer as usize];
    }
    else if ((RAM_DISPLAY_1_ADDR + RAM_DISPLAY_1_SIZE) < pointer) && (pointer <= RAM_DISPLAY_1_ADDR){
        return memory[(pointer - RAM_DISPLAY_1_ADDR) as usize];
    }
    else if ((RAM_DISPLAY_2_ADDR + RAM_DISPLAY_2_SIZE) < pointer) && (pointer <= RAM_DISPLAY_2_ADDR){
        return memory[(pointer - RAM_DISPLAY_2_ADDR) as usize];
    }
    else if ((RAM_IO_ADDR + RAM_IO_SIZE) < pointer) && (pointer <= RAM_IO_ADDR){
        return memory[(pointer - RAM_IO_ADDR) as usize];
    }
    return 0;
}