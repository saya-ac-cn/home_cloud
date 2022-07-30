use rbatis::DateTimeNative;

#[crud_table(table_name:news)]
#[derive(Clone, Debug)]
pub struct News{
    pub id:Option<u64>,
    pub topic:Option<String>,
    pub label:Option<String>,
    pub content:Option<String>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub create_time:Option<DateTimeNative>,
    pub update_time:Option<DateTimeNative>,
}
impl_field_name_method!(News{id,topic,label,content,organize,source,create_time,update_time});

#[crud_table(table_name:pictures)]
#[derive(Clone, Debug)]
pub struct Pictures{
    pub id:Option<u64>,
    pub category:Option<u32>,
    pub file_name:Option<String>,
    pub descript:Option<String>,
    pub file_url:Option<String>,
    pub web_url:Option<String>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub create_time:Option<DateTimeNative>,
    pub update_time:Option<DateTimeNative>,
}
impl_field_name_method!(Pictures{id,category,file_name,descript,file_url,web_url,organize,source,create_time,update_time});

#[crud_table(table_name:files)]
#[derive(Clone, Debug)]
pub struct Files{
    pub id:Option<u64>,
    pub uid:Option<String>,
    pub file_name:Option<String>,
    pub file_url:Option<String>,
    pub file_type:Option<String>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub status:Option<u32>,
    pub create_time:Option<DateTimeNative>,
    pub update_time:Option<DateTimeNative>,
}
impl_field_name_method!(Files{id,uid,file_name,file_url,file_type,organize,source,status,create_time,update_time});

#[crud_table(table_name:memo)]
#[derive(Clone, Debug)]
pub struct Memo{
    pub id:Option<u64>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub title:Option<String>,
    pub content:Option<String>,
    pub create_time:Option<DateTimeNative>,
    pub update_time:Option<DateTimeNative>,
}
impl_field_name_method!(Memo{id,organize,source,title,content,create_time,update_time});

#[crud_table(table_name:notebook)]
#[derive(Clone, Debug)]
pub struct NoteBook{
    pub id:Option<u64>,
    pub name:Option<String>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub status:Option<u32>,
    pub descript:Option<String>
}
impl_field_name_method!(NoteBook{id,name,organize,source,status,descript});

#[crud_table(table_name:notes)]
#[derive(Clone, Debug)]
pub struct Notes{
    pub id:Option<u64>,
    pub notebook_id:Option<u64>,
    pub label:Option<String>,
    pub topic:Option<String>,
    pub content:Option<String>,
    pub source:Option<String>,
    pub create_time:Option<DateTimeNative>,
    pub update_time:Option<DateTimeNative>,
}
impl_field_name_method!(Notes{id,notebook_id,label,topic,content,create_time,update_time});