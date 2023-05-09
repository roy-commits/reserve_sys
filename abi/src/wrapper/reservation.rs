use crate::{Error, Reservation};

pub trait Validator {
    fn validate(&self) -> Result<(), Error>;
}

impl Validator for Reservation {
    /// ensure the reservation properties are valid
    fn validate(&self) -> Result<(), Error> {
        if self.user_id.is_empty() {
            return Err(Error::InvalidUserId(&self.user_id));
        }

        if self.resource_id.is_empty() {
            return Err(Error::InvalidResourceId(&self.resource_id));
        }
        Ok(())
    }
}
