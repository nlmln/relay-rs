# NOTES  
- when adding more lanes make sure to change both `gpio.rs` and `main.rs` constants and to edit array size accordingly  
- `LANES` is a `usize` array as opposed to a `u8` array because that's what `rppal::gpio` expects us to use when addressing gpio pins  
- `let mut relay_guard = relay.lock().unwrap();` and `let reader_guard = reader.lock().unwrap();` is necessary because we are running multi threaded operations on the GPIO and mutex ensures only one code segment is accessing those pins at the same time  
- there's still plenty of memory unsafe operations due to multithreading  

# TODO  
- actualluy test the damn thing on real hardware (this is waiting on parts to arrive, I have a spare sd but no spare pi or relay board)  
- create a deployment playbook in ansible  
- try to avoid memory unsafe operations  
- improve code readability  
- add docs to github wiki  