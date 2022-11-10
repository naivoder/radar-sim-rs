use args::Args;
use radar::Radar;

use std::env;
use std::process;

mod args;
mod messages;
mod radar;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = Args::new(&args).unwrap_or_else(|err| {
        if err.to_string().contains("HELP") {
            process::exit(0);
        } else {
            eprintln!("Error parsing arguments: {}", err);
            process::exit(0);
        }
    });

    let controller_address = format!("{}:{}", args.ipaddr, args.control_port);
    let sensor_address = format!("{}:{}", args.ipaddr, args.sensor_port);
    let radar = Radar::new(controller_address, sensor_address);
    radar.run();
}
