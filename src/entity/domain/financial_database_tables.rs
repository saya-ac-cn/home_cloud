use rbatis::{DateNative, DateTimeNative, Decimal};

#[crud_table(table_name:journal)]
#[derive(Clone, Debug)]
pub struct Journal{
    pub id:Option<u64>,
    pub monetary_id:Option<u64>,
    pub income:Option<Decimal>,
    pub outlay:Option<Decimal>,
    pub means_id:Option<u64>,
    pub amount_id:Option<u64>,
    pub remarks:Option<String>,
    pub archive_date:Option<DateNative>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub create_time:Option<DateTimeNative>,
    pub update_time:Option<DateTimeNative>,
}
impl_field_name_method!(Journal{id,monetary_id,means_id,amount_id,organize,source});


#[crud_table(table_name:general_journal)]
#[derive(Clone, Debug)]
pub struct GeneralJournal{
    pub id:Option<u64>,
    pub journal_id:Option<u64>,
    pub flog:Option<u32>,
    pub amount:Option<Decimal>,
    pub remarks:Option<String>,
}
impl_field_name_method!(GeneralJournal{id,journal_id,flog});