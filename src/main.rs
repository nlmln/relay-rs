use std::error::Error;
use std::thread;

use crate::daemons::{control_output, monitor_readers};
use crate::gpio::relay;

mod actions;
mod daemons;
mod gpio;

pub const LANES: [usize; 4] = [1, 2, 3, 4]; //, 5, 6, 7, 8];
pub const TIMER_DURATION: u32 = 900;

pub static mut TIMER: [u32; 4] = [0, 0, 0, 0];
pub static mut STATE: [bool; 4] = [false, false, false, false];

fn hw_init() {
    for lane in LANES {
        let mut relay_guard = relay.lock().unwrap();
        relay_guard[lane].set_high();
    }
}

fn start_threads() {
    for lane in LANES {
        thread::spawn(move || { control_output(lane); });
    }

    thread::spawn(|| { monitor_readers(); });
}

fn main() -> Result<(), Box<dyn Error>> {
    hw_init();
    start_threads();

    loop {
        thread::park();
    }
}
