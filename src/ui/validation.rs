use cursive::Cursive;

use super::dialogs::error_msgbox;

pub enum IntValidationError {
    ZeroValue,
    InvalidNumber,
}

pub fn validate_number(s: &mut Cursive, number: &str) -> Result<u32, IntValidationError> {
    match number.parse::<u32>() {
        Ok(val) if val > 0 => Ok(val),
        Ok(_) => {
            error_msgbox(s, "The number must be a positive integer");
            Err(IntValidationError::ZeroValue)
        }
        Err(_) => {
            error_msgbox(s, "Please provide a valid number of votes");
            Err(IntValidationError::InvalidNumber)
        }
    }
}
