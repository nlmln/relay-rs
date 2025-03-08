use std::error::Error;
use rppal::gpio::{Gpio, OutputPin, InputPin};
use lazy_static::lazy_static;
use std::sync::Mutex;

//if more lanes are added in here, remember to also change the LANES definition in main.rs and obviously to increase array size
const LOGIC_OUTPUT_PINS: [u8; 4] = [5, 6, 13, 16]; //, 19, 20, 21, 26];
const LOGIC_INPUT_PINS: [u8; 4] = [2, 3, 4, 17]; //, 27, 22, 10, 9];
const RESET_BUTTON_PIN: u8 = 18;

fn initialize_gpio() -> Result<([OutputPin; 4], [InputPin; 4], InputPin), Box<dyn Error>> {
    let local_relay = [
        Gpio::new()?.get(LOGIC_OUTPUT_PINS[0])?.into_output(),
        Gpio::new()?.get(LOGIC_OUTPUT_PINS[1])?.into_output(),
        Gpio::new()?.get(LOGIC_OUTPUT_PINS[2])?.into_output(),
        Gpio::new()?.get(LOGIC_OUTPUT_PINS[3])?.into_output(),
    ];
    
    let local_reader = [
        Gpio::new()?.get(LOGIC_INPUT_PINS[0])?.into_input_pullup(),
        Gpio::new()?.get(LOGIC_INPUT_PINS[1])?.into_input_pullup(),
        Gpio::new()?.get(LOGIC_INPUT_PINS[2])?.into_input_pullup(),
        Gpio::new()?.get(LOGIC_INPUT_PINS[3])?.into_input_pullup(),
    ];

    let local_reset_button = Gpio::new()?.get(RESET_BUTTON_PIN)?.into_input_pullup();

    Ok((local_relay, local_reader, local_reset_button))
}


lazy_static! {
    pub static ref relay: Mutex<[OutputPin; 4]> = {
        let (local_relay, _, _) = initialize_gpio().expect("Failed to initialize GPIO");
        Mutex::new(local_relay)
    };

    pub static ref reader: Mutex<[InputPin; 4]> = {
        let (_, local_reader, _) = initialize_gpio().expect("Failed to initialize GPIO");
        Mutex::new(local_reader)
    };

    pub static ref reset_button: InputPin = {
        let (_, _, local_reset_button) = initialize_gpio().expect("Failed to initialize GPIO");
        local_reset_button
    };
}
