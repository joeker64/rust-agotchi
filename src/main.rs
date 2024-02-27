mod e0c6s46;

use winit::event_loop::EventLoop;

use crate::e0c6s46::*;

fn main(){
    // env_logger::init();
    let event_loop = EventLoop::new();

    let mut screen: pixels::Pixels;
    match (e0c6s46::display::create_display(event_loop)){
        Ok(data) => {
            screen = data;
            unsafe{
                e0c6s46::run_cpu(screen);
            }
        }
        Err(err) => println!("Error: {}", err),
    };
    //thread::sleep(time::Duration::from_secs(2));
}