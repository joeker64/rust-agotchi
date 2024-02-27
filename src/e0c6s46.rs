use std::fs;
use std::{thread, time};


mod instruction_set;
mod ram;
pub mod interrupts;
pub mod display;

const TIMER_1HZ_PERIOD: u64 = 32768;
const TIMER_256HZ_PERIOD: u64 = 128;
const TICK_FREQUENCY: u64 = 32768;

pub struct CPU {
    pub register_a: u16,
    pub register_b: u16,
    pub register_x: u16,
    pub register_y: u16,
    pub program_counter: u16,
    pub next_program_counter: u16,
    pub new_pointer: u16,
    pub flags: u16,
    pub memory: [u16; ram::RAM_TOTAL_SIZE as usize],
    pub stack_pointer: u16,
    pub interrupts: [interrupts::interrupt; 6],
    pub program_timer_data: u16,
    pub program_timer_reload: u16,
    pub input_port_state: [u16; 2],
    pub program_timer_enabled: bool,
    pub prog_timer_timestamp: u64,
    pub tick_counter: u64,
    pub frequency: u64,
    pub ref_ts: u64,
    pub previous_opcode_cycles: u8,
    pub display: display::display_values,
    pub clk_timer_timestamp: u64
    //pub call_depth: u16, - Only used for debug, look into way of adding this only if compiled with debug
}

pub struct test {
    pub operation: unsafe fn (*mut CPU, u16),
}

pub unsafe fn run_cpu(mut screen: pixels::Pixels){
    let mut line = String::new();
    let mut cpu: CPU = CPU {
        register_a: 0,
        register_b: 0,
        register_x: 0,
        register_y: 0,
        program_counter: 1 << 8,
        next_program_counter: 0,
        new_pointer: 1,
        flags: 0,
        memory: [0; ram::RAM_TOTAL_SIZE as usize],
        stack_pointer: 0,
        interrupts: interrupts::init_interrupts(),
        program_timer_data: 0,
        program_timer_reload: 0,
        input_port_state: [0,0],
        program_timer_enabled: false,
        prog_timer_timestamp: 0,
        tick_counter: 0,
        frequency: 1,//1000000,
        ref_ts: 0,
        previous_opcode_cycles: 0,
        display: display::init_display_values(),
        clk_timer_timestamp: 0,
    };
    let mut rom: Vec<u16> =  Vec::new();
    match read_rom("tama.b"){
        Ok(data) => rom = data,
        Err(err) => println!("Error: {}", err),
    }
    interrupts::init_io_state(&mut cpu);
    let mut x: u64 = 0;
    while x < 90000{
        let op: u16 = rom[cpu.program_counter as usize];

        cpu.next_program_counter = (cpu.program_counter + 1) & 0x1FFF;
        for opcode in instruction_set::ISA.iter(){
            if (op & opcode.mask == opcode.code) {
                // println!("{:#06x}: {} ({:#05x}) SP = {:#05x} NP = {:#05x} X = {:#05x} Y = {:#05x} A = {:#05x} B = {:#05x} FLAGS = {:#05x}",cpu.program_counter, opcode.name, op, cpu.stack_pointer, cpu.new_pointer, cpu.register_x, cpu.register_y, cpu.register_a, cpu.register_b, cpu.flags);

                cpu.ref_ts = wait_cycles(&mut cpu, cpu.ref_ts, cpu.previous_opcode_cycles);

                (opcode.operation)(&mut cpu, op);

                cpu.program_counter = cpu.next_program_counter;
                cpu.previous_opcode_cycles = opcode.cycles;

                if (opcode.name != "PSET"){
                    cpu.new_pointer = (cpu.program_counter >> 8) & 0x1F;
                }

                if (cpu.tick_counter - cpu.clk_timer_timestamp >= TIMER_1HZ_PERIOD){
                    while (cpu.tick_counter - cpu.clk_timer_timestamp >= TIMER_1HZ_PERIOD){
                        cpu.clk_timer_timestamp += TIMER_1HZ_PERIOD;
                    }

                    interrupts::handle_interrupt(&mut cpu, 5, 3);
                }

                if (cpu.program_timer_enabled && cpu.tick_counter - cpu.prog_timer_timestamp >= TIMER_256HZ_PERIOD){
                    while (cpu.tick_counter - cpu.prog_timer_timestamp >= TIMER_256HZ_PERIOD){
                        cpu.prog_timer_timestamp += TIMER_256HZ_PERIOD;
                        cpu.program_timer_data -= 1;

                        if (cpu.program_timer_data == 0){
                            cpu.program_timer_data = cpu.program_timer_reload;
                            interrupts::handle_interrupt(&mut cpu, 0, 0);
                        }
                    }
                }

                if ((cpu.flags & instruction_set::FLAG_I) > 0) && (opcode.name != "PSET"){
                    interrupts::process_interrupt(&mut cpu);
                }

                break;
            }
        }
        screen = display::update_display(screen, &mut cpu);
        x+=1;
        //println!("LCD MATRIX: \n {:?}", cpu.display.lcd_matrix);
        //let _ = std::io::stdin().read_line(&mut line);
    }
}

pub fn read_rom (path: &str) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
    let mut rom16 : Vec<u16> = vec![];
    let rom = fs::read(&path).unwrap();
    for n in 0..(rom.len() / 2){
        rom16.push(((rom[n*2] as u16) << 8) | rom[n*2+1] as u16);
    }
    Ok(rom16)
}

pub unsafe fn wait_cycles(cpu: *mut CPU,time: u64, cycles: u8 ) -> u64{
    //TODO: Needs fixing
    let mut deadline: u64 = 0;

    (*cpu).tick_counter += cycles as u64;

    deadline = time + (cycles as u64 * (*cpu).frequency as u64) / TICK_FREQUENCY;

    let ten_millis = time::Duration::from_millis(deadline);

    thread::sleep(ten_millis / 100000);

    return deadline;
}