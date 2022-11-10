use std::io::Write;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use crate::messages;

pub struct Radar {
    pub controller_address: String,
    pub sensor_address: String,
}

impl Radar {
    pub fn new(controller_address: String, sensor_address: String) -> Self {
        Self {
            controller_address,
            sensor_address,
        }
    }

    pub fn run(&self) {
        let controller_addr = self.controller_address.clone();
        let sensor_addr = self.sensor_address.clone();
        let control_handler = thread::spawn(move || Self::run_controller(controller_addr));
        let sensor_handler = thread::spawn(move || Self::run_sensor(sensor_addr));
        control_handler.join().unwrap();
        sensor_handler.join().unwrap();
    }

    pub fn run_controller(address: String) {
        let listener = TcpListener::bind(&address).unwrap();
        println!("Controller Server listening @ {}", &address);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection from: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || Self::broadcast_controller(stream));
                }
                Err(error) => println!("Controller Error - connection failed: {}", error),
            }
        }
        drop(listener);
    }

    pub fn broadcast_controller(mut stream: TcpStream) {
        let control_packet = messages::ControlPacket::new(128, true, 3.1415, 250.0, 50.0, 42, 25.0);
        let buffer = serde_json::to_string(&control_packet).unwrap();
        loop {
            match stream.write(buffer.as_bytes()) {
                // adding slight delay to not overwhelm python buffers
                Ok(_) => thread::sleep(Duration::from_millis(5)),
                Err(error) => {
                    println!("Error - terminating connection: {}", error);
                    stream.shutdown(Shutdown::Both).unwrap();
                }
            }
        }
    }

    pub fn run_sensor(address: String) {
        let listener = TcpListener::bind(&address).unwrap();
        println!("Sensor Server listening @ {}", &address);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection from: {}", stream.peer_addr().unwrap());
                    thread::spawn(move || Self::broadcast_sensor(stream));
                }
                Err(error) => println!("Sensor Error - connection failed: {}", error),
            }
        }
        drop(listener);
    }

    pub fn broadcast_sensor(mut stream: TcpStream) {
        let mut duration = 0;
        loop {
            let status_msg = messages::StatusPacket::new(
                123,
                1,
                "0852021.0.1.0.0".to_string(),
                22,
                31,
                (34.1753, -84.9192),
                (0.0, 0.0),
                327.0,
                2,
                vec![1, 2],
                messages::SensorHealth::GOOD,
                messages::RadarMode::OWLS,
                messages::CountermeasureState::STANDBY,
            );
            let emi_msg =
                messages::EmiPacket::new(22, 23, 32.123, 9.81, Duration::from_secs(duration));
            let sensor_packet = messages::SensorPacket::new(status_msg, emi_msg);
            let buffer = serde_json::to_string(&sensor_packet).unwrap();
            match stream.write(buffer.as_bytes()) {
                // adding slight delay to not overwhelm python buffers
                Ok(_) => thread::sleep(Duration::from_millis(5)),
                Err(error) => {
                    println!("Error - terminating connection: {}", error);
                    stream.shutdown(Shutdown::Both).unwrap();
                }
            }
            duration = duration + 1;
        }
    }
}
