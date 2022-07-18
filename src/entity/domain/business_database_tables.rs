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