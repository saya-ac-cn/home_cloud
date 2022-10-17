use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Journal{
    pub id:Option<u64>,
    pub monetary_id:Option<u64>,
    pub income:Option<Decimal>,
    pub outlay:Option<Decimal>,
    pub means_id:Option<u64>,
    pub abstract_id:Option<u64>,
    pub total:Option<Decimal>,
    pub remarks:Option<String>,
    pub archive_date:Option<String>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub create_time:Option<String>,
    pub update_time:Option<String>
}
impl_field_name_method!(Journal{id,monetary_id,means_id,abstract_id,organize,source});


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeneralJournal{
    pub id:Option<u64>,
    pub journal_id:Option<u64>,
    pub flag:Option<String>,
    pub amount:Option<Decimal>,
    pub remarks:Option<String>
}
impl_field_name_method!(GeneralJournal{id,journal_id,flag});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Monetary{
    pub id:Option<u64>,
    pub name:Option<String>,
    pub abbreviate:Option<String>,
    pub symbol:Option<String>
}
crud!(Monetary {});
impl_field_name_method!(Monetary{id,name,abbreviate});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Abstracts{
    pub id:Option<u64>,
    pub flag:Option<String>,
    pub tag:Option<String>
}
crud!(Abstracts {});
impl_field_name_method!(Abstracts{id,flag});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentMeans{
    pub id:Option<u64>,
    pub name:Option<String>
}
crud!(PaymentMeans {});
impl_field_name_method!(PaymentMeans{id,name});
