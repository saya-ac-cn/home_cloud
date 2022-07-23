use rbatis::DateTimeNative;

#[crud_table(table_name:news)]
#[derive(Clone, Debug)]
pub struct News{
    pub id:Option<u64>,
    pub topic:Option<String>,
    pub label:Option<String>,
    pub content:Option<String>,
    pub source:Option<String>,
    pub create_time:Option<DateTimeNative>,
    pub update_time:Option<DateTimeNative>,
}
impl_field_name_method!(News{id,topic,label,content,source,create_time,update_time});

#[crud_table(table_name:pictures)]
#[derive(Clone, Debug)]
pub struct Pictures{
    pub id:Option<u64>,
    pub category:Option<u32>,
    pub file_name:Option<String>,
    pub descript:Option<String>,
    pub file_url:Option<String>,
    pub web_url:Option<String>,
    pub source:Option<String>,
    pub date:Option<DateTimeNative>,
}
impl_field_name_method!(Pictures{id,category,file_name,descript,file_url,web_url,source,date});

#[crud_table(table_name:files)]
#[derive(Clone, Debug)]
pub struct Files{
    pub id:Option<u64>,
    pub uid:Option<String>,
    pub file_name:Option<String>,
    pub file_url:Option<String>,
    pub file_type:Option<String>,
    pub source:Option<String>,
    pub status:Option<String>,
    pub date:Option<DateTimeNative>,
}
impl_field_name_method!(Files{id,uid,file_name,file_url,file_type,source,status,date});