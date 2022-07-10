pub mod user;
pub mod resource;
pub mod page;
pub mod log;
pub mod log_type;
pub mod sign_in;


pub use user::*;
pub use page::*;
pub use resource::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmptyDTO {}
