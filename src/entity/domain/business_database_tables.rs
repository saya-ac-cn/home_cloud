use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct News{
    pub id:Option<u64>,
    pub topic:Option<String>,
    pub label:Option<String>,
    pub content:Option<String>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub create_time:Option<String>,
    pub update_time:Option<String>,
}
impl_field_name_method!(News{id,topic,label,content,organize,source,create_time});

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
impl_field_name_method!(Pictures{id,category,file_name,organize,source,create_time});

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
impl_field_name_method!(Files{id,file_name,file_type,organize,source,status,create_time});

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
impl_field_name_method!(Memo{id,organize,source,title,content,create_time});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoteBook{
    pub id:Option<u64>,
    pub name:Option<String>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub status:Option<u32>,
    pub descript:Option<String>
}
impl_field_name_method!(NoteBook{id,name,organize,source,status});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notes{
    pub id:Option<u64>,
    pub notebook_id:Option<u64>,
    pub label:Option<String>,
    pub topic:Option<String>,
    pub content:Option<String>,
    pub source:Option<String>,
    pub create_time:Option<String>,
    pub update_time:Option<String>,
}
impl_field_name_method!(Notes{id,notebook_id,label,topic,content,create_time});
