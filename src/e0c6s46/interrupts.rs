#[path = "./ram.rs"]
mod ram;

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

pub struct interrupt{
    pub factor_flag_reg: u16,
    pub mask_reg: u16,
    pub triggered: bool,
    pub value: u16,
}

pub unsafe fn handle_interrupt(cpu: *mut super::CPU, index: usize, bit: u8){
    (*cpu).interrupts[index].factor_flag_reg = (*cpu).interrupts[index].factor_flag_reg | (0x1 << bit);

    if ((*cpu).interrupts[index].mask_reg & (0x1 << bit)) > 0{
        (*cpu).interrupts[index].triggered = true;
    }
}

pub unsafe fn set_input_state(cpu: *mut super::CPU, pin: u8, state_high: bool){
    (*cpu).input_port_state[(pin & 0x4) as usize] = ((*cpu).input_port_state[(pin & 0x4) as usize] & !(0x1 << (pin & 0x3))) | ((state_high as u16) << (pin & 0x3));

    if !(state_high){
        match ((pin & 0x4) >> 2){
            0 => handle_interrupt(cpu, 3, pin & 0x3),
            1 => handle_interrupt(cpu, 2, pin & 0x3),
            _ => (),
        }
    }
}

pub unsafe fn init_io_state(cpu: *mut super::CPU){
    set_input_state(cpu, 0, true);
    set_input_state(cpu, 1, true);
    set_input_state(cpu, 2, true);
}

pub fn init_interrupts() -> [interrupt;6]{
    return [
        interrupt{
            factor_flag_reg: 0,
            mask_reg: 0,
            triggered: false,
            value: 0xC
        },
        interrupt{
            factor_flag_reg: 0,
            mask_reg: 0,
            triggered: false,
            value: 0xA
        },
        interrupt{
            factor_flag_reg: 0,
            mask_reg: 0,
            triggered: false,
            value: 0x8
        },
        interrupt{
            factor_flag_reg: 0,
            mask_reg: 0,
            triggered: false,
            value: 0x6
        },
        interrupt{
            factor_flag_reg: 0,
            mask_reg: 0,
            triggered: false,
            value: 0x4
        },
        interrupt{
            factor_flag_reg: 0,
            mask_reg: 0,
            triggered: false,
            value: 0x2
        },
    ];
}

pub unsafe fn get_io(cpu: *mut super::CPU, pointer: u16) -> u16{
    match (pointer){
        REG_CLK_INT_FACTOR_FLAGS => {
            let temp = (*cpu).interrupts[5].factor_flag_reg;
            (*cpu).interrupts[5].factor_flag_reg = 0;
            return temp;
        }
        REG_SW_INT_FACTOR_FLAGS => {
            let temp = (*cpu).interrupts[4].factor_flag_reg;
            (*cpu).interrupts[4].factor_flag_reg = 0;
            return temp;
        }
        REG_PROG_INT_FACTOR_FLAGS => {
            let temp = (*cpu).interrupts[0].factor_flag_reg;
            (*cpu).interrupts[0].factor_flag_reg = 0;
            return temp;
        }
        REG_SERIAL_INT_FACTOR_FLAGS => {
            let temp = (*cpu).interrupts[1].factor_flag_reg;
            (*cpu).interrupts[1].factor_flag_reg = 0;
            return temp;
        }
        REG_K00_K03_INT_FACTOR_FLAGS => {
            let temp = (*cpu).interrupts[2].factor_flag_reg;
            (*cpu).interrupts[2].factor_flag_reg = 0;
            return temp;
        }
        REG_K10_K13_INT_FACTOR_FLAGS => {
            let temp = (*cpu).interrupts[3].factor_flag_reg;
            (*cpu).interrupts[3].factor_flag_reg = 0;
            return temp;
        }
        REG_CLOCK_INT_MASKS => {
            return (*cpu).interrupts[5].mask_reg;
        }
        REG_SW_INT_MASKS => {
            return (*cpu).interrupts[4].mask_reg;
        }
        REG_PROG_INT_MASKS => {
            return (*cpu).interrupts[0].mask_reg;
        }
        REG_SERIAL_INT_MASKS => {
            return (*cpu).interrupts[1].mask_reg;
        }
        REG_K00_K03_INT_MASKS => {
            return (*cpu).interrupts[2].mask_reg;
        }
        REG_K10_K13_INT_MASKS => {
            return (*cpu).interrupts[3].mask_reg;
        }
        REG_PROG_TIMER_DATA_L => {
            return (*cpu).program_timer_data & 0xF;
        }
        REG_PROG_TIMER_DATA_H => {
            return ((*cpu).program_timer_data >> 4) & 0xF;
        }
        REG_PROG_TIMER_RELOAD_DATA_L => {
            return (*cpu).program_timer_reload & 0xF;
        }
        REG_PROG_TIMER_RELOAD_DATA_H => {
            return ((*cpu).program_timer_reload >> 4) & 0xF;
        }
        REG_K00_K03_INPUT_PORT => {
            return (*cpu).input_port_state[0];
        }
        REG_K10_K13_INPUT_PORT => {
            return (*cpu).input_port_state[1];
        }
        REG_K40_K43_BZ_OUTPUT_PORT => {
            return (*cpu).memory[(pointer - ram::RAM_IO_ADDR + ram::RAM_SIZE + ram::RAM_DISPLAY_1_SIZE + ram::RAM_DISPLAY_2_SIZE) as usize];
            //return ram::get_memory(cpu, pointer);
        }
        REG_CPU_OSC3_CTRL => {
            return (*cpu).memory[(pointer - ram::RAM_IO_ADDR + ram::RAM_SIZE + ram::RAM_DISPLAY_1_SIZE + ram::RAM_DISPLAY_2_SIZE) as usize];
            //return ram::get_memory(cpu, pointer);
        }
        REG_LCD_CTRL => {
            return (*cpu).memory[(pointer - ram::RAM_IO_ADDR + ram::RAM_SIZE + ram::RAM_DISPLAY_1_SIZE + ram::RAM_DISPLAY_2_SIZE) as usize];
            //return ram::get_memory(cpu, pointer);
        }
        REG_LCD_CONTRAST => {

        }
        REG_SVD_CTRL => {
            return (*cpu).memory[(pointer - ram::RAM_IO_ADDR + ram::RAM_SIZE + ram::RAM_DISPLAY_1_SIZE + ram::RAM_DISPLAY_2_SIZE) as usize] & 0x7;
            //return ram::get_memory(cpu, pointer) & 0x7;
        }
        REG_BUZZER_CTRL1 => {
            return (*cpu).memory[(pointer - ram::RAM_IO_ADDR + ram::RAM_SIZE + ram::RAM_DISPLAY_1_SIZE + ram::RAM_DISPLAY_2_SIZE) as usize];
            //return ram::get_memory(cpu, pointer);
        }
        REG_BUZZER_CTRL2 => {
            return (*cpu).memory[(pointer - ram::RAM_IO_ADDR + ram::RAM_SIZE + ram::RAM_DISPLAY_1_SIZE + ram::RAM_DISPLAY_2_SIZE) as usize] & 0x3;
            //return ram::get_memory(cpu, pointer) & 0x3;
        }
        REG_CLK_WD_TIMER_CTRL => {

        }
        REG_SW_TIMER_CTRL => {

        }
        REG_PROG_TIMER_CTRL => {
            if (*cpu).program_timer_enabled{
                return 1;
            }
            else {
                return 0;
            }
        }
        REG_PROG_TIMER_CLK_SEL => {

        }
        _ => println!("ERROR")
    }
    return 0;
}

pub unsafe fn set_io(cpu: *mut super::CPU, pointer: u16, value: u16){
    match (pointer){
        REG_CLOCK_INT_MASKS => {
            (*cpu).interrupts[5].mask_reg = value;
        }
        REG_SW_INT_MASKS => {
            (*cpu).interrupts[4].mask_reg = value;
        }
        REG_PROG_INT_MASKS => {
            (*cpu).interrupts[0].mask_reg = value;
        }
        REG_SERIAL_INT_MASKS => {
            (*cpu).interrupts[1].mask_reg = value;
        }
        REG_K00_K03_INT_MASKS => {
            (*cpu).interrupts[2].mask_reg = value;
        }
        REG_K10_K13_INT_MASKS => {
            (*cpu).interrupts[3].mask_reg = value;
        }
        REG_PROG_TIMER_RELOAD_DATA_L => {
            (*cpu).program_timer_reload = value | ((*cpu).program_timer_reload & 0xF0);
        }
        REG_PROG_TIMER_RELOAD_DATA_H => {
            (*cpu).program_timer_reload = ((*cpu).program_timer_reload & 0xF0) | (value << 4);
        }
        REG_K00_K03_INPUT_PORT => {

        }
        REG_K40_K43_BZ_OUTPUT_PORT => {
            //TODO - Add support for buzzer
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
            //TODO - Add support for buzzer
        }
        REG_BUZZER_CTRL2 => {

        }
        REG_CLK_WD_TIMER_CTRL => {

        }
        REG_SW_TIMER_CTRL => {

        }
        REG_PROG_TIMER_CTRL => {
            if (value & 0x2) > 0{
                (*cpu).program_timer_data = (*cpu).program_timer_reload;
            }

            if ((value & 0x1) > 0) && !(*cpu).program_timer_enabled{
                (*cpu).prog_timer_timestamp = (*cpu).tick_counter;
            }

            if (value & 0x1) > 0 {
                (*cpu).program_timer_enabled = true;
            } else{
                (*cpu).program_timer_enabled = false
            }
        }
        REG_PROG_TIMER_CLK_SEL => {

        }
        _ => {

        }
    }
}