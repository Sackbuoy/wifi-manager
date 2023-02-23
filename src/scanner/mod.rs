use simple_error::SimpleError;
use std::error::Error;
use wifiscanner::Wifi;

pub fn scan() -> Result<Vec<Wifi>, Box<dyn Error>> {
    match wifiscanner::scan() {
        Ok(results) => return Ok(results),
        Err(e) => match e {
            wifiscanner::Error::NoMatch => return Err(Box::new(SimpleError::new("known error"))),
            _ => return Err(Box::new(SimpleError::new("unknown error"))),
        },
    }
}
