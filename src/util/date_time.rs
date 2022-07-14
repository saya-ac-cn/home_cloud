pub trait DateTimeUtil{
    fn naive_date_time_to_str(&self,format:&str) -> Option<String>;
}

impl DateTimeUtil for Option<chrono::naive::NaiveDate>{
    fn naive_date_time_to_str(&self,format:&str) -> Option<String>{
        match self {
            None => None,
            Some(naive_date_time) => Some(naive_date_time.format(format).to_string()),
        }
    }
}

impl DateTimeUtil for Option<chrono::NaiveDateTime>{
    fn naive_date_time_to_str(&self,format:&str) -> Option<String>{
        match self {
            None => None,
            Some(naive_date_time) => Some(naive_date_time.format(format).to_string()),
        }
    }
}

impl DateTimeUtil for Option<rbatis::DateTimeNative>{
    fn naive_date_time_to_str(&self,format:&str) -> Option<String>{
        match self {
            None => None,
            Some(naive_date_time) => {
                let date = self.unwrap();
                DateTimeUtil::naive_date_time_to_str(&Some(date.inner),format)
            },
        }
    }
}
