use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::net::IpAddr;
use std::str::FromStr;

pub struct Args {
    pub ipaddr: IpAddr,
    pub control_port: u16,
    pub sensor_port: u16,
}

impl Args {
    pub fn new(args: &[String]) -> Result<Args, ArgError> {
        if (args[1].contains("-h") || args[1].contains("--help")) && args.len() == 2 {
            println!("Usage: ./radar_sim <ipaddr> <control-port> <sensor-port>");
            return Err(ArgError::Help);
        } else if args.len() < 3 {
            return Err(ArgError::NotEnoughArgs);
        } else if args.len() > 4 {
            return Err(ArgError::TooManyArgs);
        }
        if let Ok(ipaddr) = IpAddr::from_str(&args[1]) {
            if let Ok(control_port) = args[2].parse::<u16>() {
                if let Ok(sensor_port) = args[3].parse::<u16>() {
                    return Ok(Args {
                        ipaddr,
                        control_port,
                        sensor_port,
                    });
                }
            }
        }
        return Err(ArgError::InvalidSyntax);
    }
}

pub enum ArgError {
    NotEnoughArgs,
    TooManyArgs,
    InvalidSyntax,
    Help,
}

impl ArgError {
    fn message(&self) -> &str {
        match self {
            Self::NotEnoughArgs => "Not Enough Arguments",
            Self::TooManyArgs => "Too Many Arguments",
            Self::InvalidSyntax => "Invalid Syntax",
            Self::Help => "HELP",
        }
    }
}

impl Display for ArgError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ArgError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ArgError {}
