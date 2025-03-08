use std::thread;
use std::time::Duration;
use std::sync::MutexGuard;
use rppal::gpio::{OutputPin, InputPin};

use crate::gpio::{reader, relay, reset_button};
use crate::actions::{add_time, reset_all};
use crate::{TIMER, STATE, LANES};

fn sleep(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

pub fn control_output(lane: usize) {
    loop {
        unsafe {
            if STATE[lane] {
                sleep(1000);
                if TIMER[lane] > 0 {
                    TIMER[lane] -= 1;
                    if TIMER[lane] == 0 {
                        let mut relay_guard: MutexGuard<[OutputPin; 4]> = relay.lock().unwrap();
                        relay_guard[lane].set_high();
                    }
                }
            }
            else {
                sleep(1000);
            }
        }
    }
}

pub fn monitor_readers() {
    loop {
        for lane in LANES {
            let reader_guard: MutexGuard<[InputPin; 4]> = reader.lock().unwrap();
            if reader_guard[lane].is_low() {
                add_time(lane);
                sleep(500);
            }
        }
        if reset_button.is_low() {
            reset_all();
            sleep(500);
        }
    }
}