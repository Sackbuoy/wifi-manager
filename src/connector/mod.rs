extern crate wifi_rs;

use clap::Parser;
use clap::Subcommand;
use simple_error::SimpleError;
use std::error::Error;
use wifi_rs::prelude::*;
use wifi_rs::WiFi;

use nix::ifaddrs::getifaddrs;

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Action {
    Scan {
        // I thought i could specify an interface but the
        // package im using doesnt support it
        // #[arg(short, long)]
        // interface: String,
    },
    Connect {
        #[arg(default_value_t = get_default_interface().unwrap(), short, long)]
        interface: String,

        #[arg(short, long)]
        network: String,

        #[arg(short, long)]
        password: String,
    },
    Disconnect {
        #[arg(default_value_t = get_default_interface().unwrap(), short, long)]
        interface: String,
    },
}

pub fn connect(
    interface: String,
    network: &String,
    password: String,
) -> Result<bool, WifiConnectionError> {
    let config = Some(Config {
        interface: Some(interface.as_str()),
    });

    let mut wifi = WiFi::new(config);

    wifi.connect(&network, &password)
}

pub fn disconnect(interface: String) -> Result<bool, WifiConnectionError> {
    let config = Some(Config {
        interface: Some(interface.as_str()),
    });

    let wifi = WiFi::new(config);

    wifi.disconnect()
}

fn get_default_interface() -> Result<String, Box<dyn Error>> {
    let addrs = getifaddrs()?;

    // I dont know a better way to find the default rn so this is what we doin
    let common_iface_names = vec!["eth0", "eth1", "wlan0", "enp3s0", "wlp2s0"];

    for ifaddr in addrs {
        if common_iface_names.contains(&ifaddr.interface_name.as_str()) {
            return Ok(ifaddr.interface_name);
        }
    }

    Err(Box::new(SimpleError::new(
        "Could not find network interface. Place specify using the -i flag",
    )))
}
