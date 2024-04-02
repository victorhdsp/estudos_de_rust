use std::error;
use std::fmt;

#[derive(Debug)]
struct GenericError {
    message: String
}

impl GenericError {
    fn new(message: &str) -> GenericError {
        GenericError {
            message: message.to_string()
        }
    }
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for GenericError {}

fn create_error (message: &str) -> Result<(), Box<dyn error::Error>> {
    let err = GenericError::new(message);
    Err(Box::new(err))
}

pub fn throw_error(message: &str) {
    if let Err(err) = create_error(message) {
        println!("Ocorreu um erro: {}", err);
    }
}