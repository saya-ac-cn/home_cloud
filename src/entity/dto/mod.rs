pub mod user;
pub mod resource;
pub mod page;
pub mod log;

pub use user::*;
pub use page::*;
pub use resource::*;
pub mod sign_in;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmptyDTO {}
