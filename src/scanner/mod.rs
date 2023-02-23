use super::errors::err_box;
use std::error::Error;
use wifiscanner::Wifi;

pub fn scan() -> Result<Vec<Wifi>, Box<dyn Error>> {
    match wifiscanner::scan() {
        Ok(results) => return Ok(results),
        Err(e) => match e {
            wifiscanner::Error::NoMatch => return Err(err_box("known error")),
            _ => return Err(err_box("unknown error")),
        },
    }
}
