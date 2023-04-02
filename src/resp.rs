use std::fmt::{Display, Formatter};
use crate::headers::Response;

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.error {
            true => write!(f, "query executed in {:?} with error. details: {:?}", self.time_taken, self.message),
            false => write!(f, "query executed in {:?} with success status", self.time_taken)
        }
    }
}