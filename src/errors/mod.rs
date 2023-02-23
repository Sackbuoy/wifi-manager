use simple_error::SimpleError;
use std::error::Error;

// literally just cause typing Err(Box::new(SimpleError::new("message"))) every goddamn time
pub fn err_box(err_string: &str) -> Box<dyn Error> {
    return Box::new(SimpleError::new(err_string));
}
