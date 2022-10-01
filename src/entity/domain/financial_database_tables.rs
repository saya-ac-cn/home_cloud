use rust_decimal::Decimal;

#[crud_table(table_name:journal)]
#[derive(Clone, Debug)]
pub struct Journal{
    pub id:Option<u64>,
    pub monetary_id:Option<u64>,
    pub income:Option<Decimal>,
    pub outlay:Option<Decimal>,
    pub means_id:Option<u64>,
    pub abstract_id:Option<u64>,
    pub total:Option<Decimal>,
    pub remarks:Option<String>,
    pub archive_date:Option<chrono::NaiveDate>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub create_time:Option<chrono::NaiveDateTime>,
    pub update_time:Option<chrono::NaiveDateTime>
}
impl_field_name_method!(Journal{id,monetary_id,means_id,abstract_id,organize,source});


#[crud_table(table_name:general_journal)]
#[derive(Clone, Debug)]
pub struct GeneralJournal{
    pub id:Option<u64>,
    pub journal_id:Option<u64>,
    pub flag:Option<String>,
    pub amount:Option<Decimal>,
    pub remarks:Option<String>
}
impl_field_name_method!(GeneralJournal{id,journal_id,flag});

#[crud_table(table_name:monetary)]
#[derive(Clone, Debug)]
pub struct Monetary{
    pub id:Option<u64>,
    pub name:Option<String>,
    pub abbreviate:Option<String>,
    pub symbol:Option<String>
}
impl_field_name_method!(Monetary{id,name,abbreviate});

#[crud_table(table_name:abstracts)]
#[derive(Clone, Debug)]
pub struct Abstracts{
    pub id:Option<u64>,
    pub flag:Option<String>,
    pub tag:Option<String>
}
impl_field_name_method!(Abstracts{id,flag});

#[crud_table(table_name:payment_means)]
#[derive(Clone, Debug)]
pub struct PaymentMeans{
    pub id:Option<u64>,
    pub name:Option<String>
}
impl_field_name_method!(PaymentMeans{id,name});