use crate::{TIMER, TIMER_DURATION, STATE, LOGIC_OUTPUT_PINS};
use crate::gpio::relay;
use rppal::gpio::OutputPin;
use std::sync::MutexGuard;

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

pub fn reset_all() {
    for lane in LOGIC_OUTPUT_PINS.iter() {
        let lane_usize = *lane as usize;
        unsafe {
            TIMER[lane_usize] = 0;
            STATE[lane_usize] = false;
            let mut relay_guard: MutexGuard<[OutputPin; 4]> = relay.lock().unwrap();
            relay_guard[lane_usize].set_high();
        }
    }
}
