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
pub mod plan;
pub mod plan_archive;
pub mod db_dump_log;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmptyDTO {}

/// IdDTO
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IdDTO {
    pub id: Option<String>,
}
