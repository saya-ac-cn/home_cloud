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


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeneralJournal{
    pub id:Option<u64>,
    pub journal_id:Option<u64>,
    pub flag:Option<String>,
    pub amount:Option<Decimal>,
    pub remarks:Option<String>
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Monetary{
    pub id:Option<u64>,
    pub name:Option<String>,
    pub abbreviate:Option<String>,
    pub symbol:Option<String>
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Abstracts{
    pub id:Option<u64>,
    pub flag:Option<String>,
    pub tag:Option<String>
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentMeans{
    pub id:Option<u64>,
    pub name:Option<String>
}

