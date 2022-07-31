pub mod user;
pub mod page;
pub mod log;
pub mod log_type;
pub mod sign_in;
pub mod picture_base64;
pub mod news;
pub mod pictures;
pub mod files;
pub mod memo;
pub mod notes;
pub mod notebook;
pub mod general_journal;
pub mod journal;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmptyDTO {}
