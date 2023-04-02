use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct News{
    pub id:Option<u64>,
    pub topic:Option<String>,
    pub label:Option<String>,
    pub abstracts:Option<String>,
    pub content:Option<String>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub create_time:Option<String>,
    pub update_time:Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pictures{
    pub id:Option<u64>,
    pub category:Option<u32>,
    pub file_name:Option<String>,
    pub descript:Option<String>,
    pub file_url:Option<String>,
    pub web_url:Option<String>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub create_time:Option<String>,
    pub update_time:Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Files{
    pub id:Option<u64>,
    pub uid:Option<String>,
    pub file_name:Option<String>,
    pub file_url:Option<String>,
    pub file_type:Option<String>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub status:Option<u32>,
    pub create_time:Option<String>,
    pub update_time:Option<String>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Memo{
    pub id:Option<u64>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub title:Option<String>,
    pub content:Option<String>,
    pub create_time:Option<String>,
    pub update_time:Option<String>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoteBook{
    pub id:Option<u64>,
    pub name:Option<String>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub status:Option<u32>,
    pub descript:Option<String>
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notes{
    pub id:Option<u64>,
    pub notebook_id:Option<u64>,
    pub label:Option<String>,
    pub topic:Option<String>,
    pub abstracts:Option<String>,
    pub content:Option<String>,
    pub source:Option<String>,
    pub create_time:Option<String>,
    pub update_time:Option<String>,
}

