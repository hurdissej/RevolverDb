use std::fmt;

pub struct CommandError; 

// Implement std::fmt::Display for CommandError
impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The input contained an a syntax error") // user-facing output
    }
}

// Implement std::fmt::Debug for CommandError
impl fmt::Debug for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}