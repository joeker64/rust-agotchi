mod e0c6s46;

fn main() {

    unsafe{
        e0c6s46::run_cpu();
    }

    // match e0c6s46::read_rom("tama.b"){
    //     Ok(data) => println!("{:#06x}",data[256]),
    //     Err(err) => println!("Error: {}", err),
    // }
    // unsafe{
    //     let mut cp = e0c6s46::CPU{
    //         register_a: 0,
    //         register_b: 0,
    //         register_x: 0,
    //         register_y: 0,
    //         program_counter: 0,
    //         bank_pointer: 0,
    //         page_pointer: 0,
    //         flags: 0,
    //     };
    //     let tmp = e0c6s46::test{
    //         operation:test2
    //     };
    //     println!("ra: {}", cp.register_a);
    //     (tmp.operation)(&mut cp,2);
    //     println!("ra: {}", cp.register_a);
    // }
    
}

// unsafe fn test2(cp: *mut e0c6s46::CPU, op: u16){
//     (*cp).register_a = 10;
// }