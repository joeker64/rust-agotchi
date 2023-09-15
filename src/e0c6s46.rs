use std::fs;

pub mod instruction_set;
pub mod ram;

pub struct CPU {
    pub register_a: u16,
    pub register_b: u16,
    pub register_x: u16,
    pub register_y: u16,
    pub program_counter: u16,
    pub next_program_counter: u16,
    pub new_pointer: u16,
    pub flags: u16,
    pub memory: [u16;ram::RAM_TOTAL_SIZE as usize],
    pub stack_pointer: u16,
    //pub call_depth: u16, - Only used for debug, look into way of adding this only if compiled with debug 
}

pub struct test {
    pub operation: unsafe fn (*mut CPU, u16),
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_b: 0,
            register_x: 0,
            register_y: 0,
            program_counter: 0,
            next_program_counter: 0,
            new_pointer: 0,
            flags: 0,
            memory: [0; ram::RAM_TOTAL_SIZE as usize],
            stack_pointer: 0,
        }
    }
  
    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode {
                _ => todo!()
            }
        }
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
