mod connector;
mod errors;
mod scanner;

use clap::Parser;
use connector::{Action, Args};
use simple_error::SimpleError;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
enum Command {
    Scan,
    Connect,
    Disconnect,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Scan => write!(f, "scan"),
            Command::Connect => write!(f, "connect"),
            Command::Disconnect => write!(f, "disconnect"),
        }
    }
}

impl FromStr for Command {
    type Err = SimpleError;

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input.to_lowercase().as_str() {
            "scan" => Ok(Command::Scan),
            "connect" => Ok(Command::Connect),
            "disconnect" => Ok(Command::Disconnect),
            _ => Err(SimpleError::new(format!(
                "Invalid command given: {}",
                input.to_lowercase().as_str()
            ))),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.action {
        Action::Scan {} => {
            match scanner::scan() {
                Ok(networks) => {
                    for network in networks {
                        println!("{:?}", network)
                    }
                }
                Err(e) => println!("Failed to scan: {:?}", e),
            }
            Ok(())
        }
        Action::Connect {
            interface,
            network,
            password,
        } => {
            match connector::connect(interface, &network, password) {
                Ok(success) => {
                    if success {
                        println!("Connected to {}", network);
                    } else {
                        println!("Could not connect, but no error was reported")
                    }
                }
                Err(e) => println!("Could not connect: {:?}", e),
            }

            Ok(())
        }
        Action::Disconnect { interface } => {
            match connector::disconnect(interface) {
                Ok(success) => println!(
                    "{}",
                    if success {
                        "Disconnected from network"
                    } else {
                        "Could not disconnect, but no error was reported"
                    }
                ),
                Err(e) => println!("Could not disconnect: {:?}", e),
            }

            Ok(())
        }
    }
}
