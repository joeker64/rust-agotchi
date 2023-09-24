use std::fs;

mod instruction_set;
mod ram;
pub mod interrupts;

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
    pub prog_timer_timestamp: u16,
    pub tick_counter: u16,
    //pub call_depth: u16, - Only used for debug, look into way of adding this only if compiled with debug
}

pub struct test {
    pub operation: unsafe fn (*mut CPU, u16),
}

pub unsafe fn run_cpu(){
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
    };
    let mut rom: Vec<u16> =  Vec::new();
    match read_rom("tama.b"){
        Ok(data) => rom = data,
        Err(err) => println!("Error: {}", err),
    }
    interrupts::init_io_state(&mut cpu);
    loop{
        let op: u16 = rom[cpu.program_counter as usize];

        cpu.next_program_counter = (cpu.program_counter + 1) & 0x1FFF;
        for opcode in instruction_set::ISA.iter(){
            if (op & opcode.mask == opcode.code) {
                println!("{:#06x}: {} ({:#05x}) SP = {:#05x} NP = {:#05x} X = {:#05x} Y = {:#05x} A = {:#05x} B = {:#05x} FLAGS = {:#05x}",cpu.program_counter, opcode.name, op, cpu.stack_pointer, cpu.new_pointer, cpu.register_x, cpu.register_y, cpu.register_a, cpu.register_b, cpu.flags);
                (opcode.operation)(&mut cpu, op);

                cpu.program_counter = cpu.next_program_counter;

                if opcode.name != "PSET"{
                    cpu.new_pointer = (cpu.program_counter >> 8) & 0x1F;
                }
                break;
            }
        }
        let _ = std::io::stdin().read_line(&mut line);
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
