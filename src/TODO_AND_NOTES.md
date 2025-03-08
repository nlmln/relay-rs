# NOTES  
- when adding more lanes make sure to change both `gpio.rs` and `main.rs` constants and to edit array size accordingly  
- `LANES` is a `usize` array as opposed to a `u8` array because that's what `rppal::gpio` expects us to use when addressing gpio pins  
- `let mut relay_guard = relay.lock().unwrap();` and `let reader_guard = reader.lock().unwrap();` is necessary because we are running multi threaded operations on the GPIO and mutex ensures only one code segment is accessing those pins at the same time  
- there's still plenty of memory unsafe operations due to multithreading  

# TODO  
- add a API (like RESTful or WebSocket) to allow our webui to communicate  
- try to avoid memory unsafe operations  
- improve code readability  