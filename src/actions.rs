use rppal::gpio::OutputPin;
use std::sync::MutexGuard;

use crate::{TIMER, TIMER_DURATION, STATE, LANES};
use crate::gpio::relay;

pub fn add_time(lane: usize) {
    unsafe {
        TIMER[lane] += TIMER_DURATION;
        STATE[lane] = true;
        let mut relay_guard: MutexGuard<[OutputPin; 4]> = relay.lock().unwrap();
        relay_guard[lane].set_low();
    }
}

pub fn subtract_time(lane: usize) {
    unsafe {
        if TIMER[lane] != 0 {
            TIMER[lane] -= TIMER_DURATION;
        }
    }
}

pub fn reset_lane(lane: usize) {
    unsafe {
        TIMER[lane] = 0;
        STATE[lane] = false;
        let mut relay_guard: MutexGuard<[OutputPin; 4]> = relay.lock().unwrap();
        relay_guard[lane].set_high();
    }
}

pub fn reset_all() {
    for lane in LANES {
        unsafe {
            TIMER[lane] = 0;
            STATE[lane] = false;
            let mut relay_guard: MutexGuard<[OutputPin; 4]> = relay.lock().unwrap();
            relay_guard[lane].set_high();
        }
    }
}

pub fn status() -> String {
    let mut status = String::new();
    unsafe {
        for lane in LANES {
            let time = seconds_to_hhmmss(TIMER[lane]);
            status = format!("{}{}: {},", status, lane, time);
        }
        
    }
    format!("Status: {}", status)
}

fn seconds_to_hhmmss(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}