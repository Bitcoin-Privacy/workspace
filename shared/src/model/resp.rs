use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug)]
pub enum Status {
    Success,
    BadRequest,
}

#[cfg_attr(feature = "backend", derive(Serialize))]
#[cfg_attr(feature = "frontend", derive(Deserialize))]
#[derive(Debug)]
pub struct ApiResponse<T> {
    pub status: Status,
    pub message: Option<String>,
    pub data: Option<T>,
}
