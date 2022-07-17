pub mod user;
pub mod page;
pub mod log;
pub mod log_type;
pub mod sign_in;
pub mod picture_base64;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmptyDTO {}
