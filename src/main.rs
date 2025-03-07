use std::error::Error;
use std::thread;

use crate::daemons::{control_output, monitor_readers};
use crate::gpio::relay;

pub mod actions;
mod daemons;
mod gpio;

pub const LOGIC_OUTPUT_PINS: [u8; 4] = [5, 6, 13, 16]; //, 19, 20, 21, 26];
pub const LOGIC_INPUT_PINS: [u8; 4] = [2, 3, 4, 17]; //, 27, 22, 10, 9];
pub const RESET_BUTTON_PIN: u8 = 18;
pub const TIMER_DURATION: u32 = 900;

pub static mut TIMER: [u32; 4] = [0, 0, 0, 0];
pub static mut STATE: [bool; 4] = [false, false, false, false];

fn hw_init() {
    for lane in LOGIC_OUTPUT_PINS.iter() {
        let lane_usize = *lane as usize;
        let mut relay_guard = relay.lock().unwrap();
        relay_guard[lane_usize].set_high();
    }
}

fn start_threads() {
    for lane in LOGIC_OUTPUT_PINS.iter() {
        let lane_usize = *lane as usize;
        thread::spawn(move || {
            control_output(lane_usize);
        });
    }

    thread::spawn(|| {
        monitor_readers();
    });
}

fn main() -> Result<(), Box<dyn Error>> {
    hw_init();
    start_threads();

    loop {
        thread::park();
    }
}
