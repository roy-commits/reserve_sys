use crate::Status;
use std::fmt::{Display, Formatter};

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Unknown => write!(f, "unknown"),
            Status::Pending => write!(f, "pending"),
            Status::Confirmed => write!(f, "confirmed"),
            Status::Blocked => write!(f, "blocked"),
        }
    }
}
