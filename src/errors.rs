use std::fmt;
use warp::reject::Reject;

#[derive(Debug)]
pub struct InternalServerError;

impl fmt::Display for InternalServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Internal Server Error")
    }
}

impl Reject for InternalServerError {}
