const RAM_SIZE: u16 = 0x280;
const RAM_DISPLAY_1_SIZE: u16 = 0x050;
const RAM_DISPLAY_1_ADDR: u16 = 0xE00;
const RAM_DISPLAY_2_SIZE: u16 = 0x050;
const RAM_DISPLAY_2_ADDR: u16 = 0xE80;
const RAM_IO_SIZE: u16 = 0x080;
const RAM_IO_ADDR: u16 = 0xF00;
pub const RAM_TOTAL_SIZE: u16 = RAM_SIZE + RAM_DISPLAY_1_SIZE + RAM_DISPLAY_2_SIZE + RAM_IO_SIZE;

const REG_CLK_INT_FACTOR_FLAGS: u16 = 0xF00;
const REG_SW_INT_FACTOR_FLAGS: u16 = 0xF01;
const REG_PROG_INT_FACTOR_FLAGS: u16 = 0xF02;
const REG_SERIAL_INT_FACTOR_FLAGS: u16 = 0xF03;
const REG_K00_K03_INT_FACTOR_FLAGS:	u16 = 0xF04;
const REG_K10_K13_INT_FACTOR_FLAGS:	u16 = 0xF05;
const REG_CLOCK_INT_MASKS:	u16 = 0xF10;
const REG_SW_INT_MASKS:	u16 = 0xF11;
const REG_PROG_INT_MASKS: u16 = 0xF12;
const REG_SERIAL_INT_MASKS:	u16 = 0xF13;
const REG_K00_K03_INT_MASKS: u16 = 0xF14;
const REG_K10_K13_INT_MASKS: u16 = 0xF15;
const REG_PROG_TIMER_DATA_L: u16 = 0xF24;
const REG_PROG_TIMER_DATA_H: u16 = 0xF25;
const REG_PROG_TIMER_RELOAD_DATA_L:	u16 = 0xF26;
const REG_PROG_TIMER_RELOAD_DATA_H:	u16 = 0xF27;
const REG_K00_K03_INPUT_PORT: u16 = 0xF40;
const REG_K10_K13_INPUT_PORT: u16 = 0xF42;
const REG_K40_K43_BZ_OUTPUT_PORT: u16 = 0xF54;
const REG_CPU_OSC3_CTRL: u16 = 0xF70;
const REG_LCD_CTRL:	u16 = 0xF71;
const REG_LCD_CONTRAST:	u16 = 0xF72;
const REG_SVD_CTRL:	u16 = 0xF73;
const REG_BUZZER_CTRL1:	u16 = 0xF74;
const REG_BUZZER_CTRL2:	u16 = 0xF75;
const REG_CLK_WD_TIMER_CTRL: u16 = 0xF76;
const REG_SW_TIMER_CTRL: u16 = 0xF77;
const REG_PROG_TIMER_CTRL: u16 = 0xF78;
const REG_PROG_TIMER_CLK_SEL: u16 = 0xF79;

pub fn set_memory(mut memory: [u16; RAM_TOTAL_SIZE as usize], pointer: u16, value: u16) -> [u16; RAM_TOTAL_SIZE as usize]{;
    if pointer < RAM_SIZE {
        memory[pointer as usize] = value;
        return memory;
    }
    else if ((RAM_DISPLAY_1_ADDR + RAM_DISPLAY_1_SIZE) > pointer) && (pointer >= RAM_DISPLAY_1_ADDR){
        memory[(pointer - RAM_DISPLAY_1_ADDR + RAM_SIZE) as usize] = value;
        return memory;
    }
    else if ((RAM_DISPLAY_2_ADDR + RAM_DISPLAY_2_SIZE) > pointer) && (pointer >= RAM_DISPLAY_2_ADDR){
        memory[(pointer - RAM_DISPLAY_2_ADDR + RAM_SIZE + RAM_DISPLAY_1_SIZE) as usize] = value;
        return memory;
    }
    else if ((RAM_IO_ADDR + RAM_IO_SIZE) > pointer) && (pointer >= RAM_IO_ADDR){
        memory[(pointer - RAM_IO_ADDR + RAM_SIZE + RAM_DISPLAY_1_SIZE + RAM_DISPLAY_2_SIZE) as usize] = value;
        return memory;
    }
    else{
        println!("ERROR");
        return memory;
    }

}

pub fn get_memory(memory: [u16; RAM_TOTAL_SIZE as usize], pointer: u16) -> u16 {
    if pointer < RAM_SIZE {
        return memory[pointer as usize];
    }
    else if ((RAM_DISPLAY_1_ADDR + RAM_DISPLAY_1_SIZE) < pointer) && (pointer <= RAM_DISPLAY_1_ADDR){
        return memory[(pointer - RAM_DISPLAY_1_ADDR + RAM_SIZE) as usize];
    }
    else if ((RAM_DISPLAY_2_ADDR + RAM_DISPLAY_2_SIZE) < pointer) && (pointer <= RAM_DISPLAY_2_ADDR){
        return memory[(pointer - RAM_DISPLAY_2_ADDR + RAM_SIZE + RAM_DISPLAY_1_SIZE) as usize];
    }
    else if ((RAM_IO_ADDR + RAM_IO_SIZE) < pointer) && (pointer <= RAM_IO_ADDR){
        return memory[(pointer - RAM_IO_ADDR + RAM_SIZE + RAM_DISPLAY_1_SIZE + RAM_DISPLAY_2_SIZE) as usize];
    }
    return 0;
}

fn get_io(memory: [u16; RAM_TOTAL_SIZE as usize], pointer: u16) -> u16{
    match (pointer){
        REG_CLK_INT_FACTOR_FLAGS => {

        }
        REG_SW_INT_FACTOR_FLAGS => {

        }
        REG_PROG_INT_FACTOR_FLAGS => {

        }
        REG_SERIAL_INT_FACTOR_FLAGS => {

        }
        REG_K00_K03_INT_FACTOR_FLAGS => {

        }
        REG_K10_K13_INT_FACTOR_FLAGS => {

        }
        REG_CLOCK_INT_MASKS => {

        }
        REG_SW_INT_MASKS => {

        }
        REG_PROG_INT_MASKS => {

        }
        REG_SERIAL_INT_MASKS => {

        }
        REG_K00_K03_INT_MASKS => {

        }
        REG_K10_K13_INT_MASKS => {

        }
        REG_PROG_TIMER_DATA_L => {

        }
        REG_PROG_TIMER_DATA_H => {

        }
        REG_PROG_TIMER_RELOAD_DATA_L => {

        }
        REG_PROG_TIMER_RELOAD_DATA_H => {

        }
        REG_K00_K03_INPUT_PORT => {

        }
        REG_K10_K13_INPUT_PORT => {

        }
        REG_K40_K43_BZ_OUTPUT_PORT => {

        }
        REG_CPU_OSC3_CTRL => {

        }
        REG_LCD_CTRL => {

        }
        REG_LCD_CONTRAST => {

        }
        REG_SVD_CTRL => {

        }
        REG_BUZZER_CTRL1 => {

        }
        REG_BUZZER_CTRL2 => {

        }
        REG_CLK_WD_TIMER_CTRL => {

        }
        REG_SW_TIMER_CTRL => {

        }
        REG_PROG_TIMER_CTRL => {

        }
        REG_PROG_TIMER_CLK_SEL => {

        }
        _ => println!("ERROR")
    }
    return 0;
}