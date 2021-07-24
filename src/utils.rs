use std::fmt;


pub struct NegativeIntegerError;

impl fmt::Display for NegativeIntegerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A negative integer has been found.")
    }
}

impl fmt::Debug for NegativeIntegerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{file: {}, line: {}}}", file!(), line!())
    }
}
