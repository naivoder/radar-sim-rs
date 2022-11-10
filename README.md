# RADAR PACKET SIMULATOR
This repo contains a multithreaded radar packet simulator written in Rust to aid GUI development.    
The simulator will broadcast sample controller and sensor TCP packets to separate ports (typically 4321 and 4322). 

# Install  
Install `rustup` on your system of choice to build the Rust language binary. 

# Development  
Use the Rust package manager, Cargo, to build and run the application in development mode:  
```
cd radar-sim-rs && cargo run 127.0.0.1 4321 4322
```

# Release  
The binary itself is saved to the `./target` dir. Build optimized version with `cargo build --release`. 
```
./radar-sim-rs <ip-address> <control-port> <sensor-port>
```
