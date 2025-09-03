# relay-rs  

This repo contains software I've written to manage and control bowling lanes, translating pulses from our carding system to games of stackable length implementing monitoring, a web console for internal use (eg. booking a lane).  

The project is powered by a RaspberryPi using a relay board, Rust was chosen over Go for it's more solid GPIO library and WebSockets were implemented to improve monitoring especially across different devices, this software was originally written in python as a proof of concept which has since been rewritten in Rust after proving itself in it's original form.  
