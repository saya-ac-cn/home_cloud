pub mod user;
pub mod page;
pub mod log;
pub mod log_type;
pub mod sign_in;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmptyDTO {}
