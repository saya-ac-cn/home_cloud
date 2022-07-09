pub trait DateTimeUtil{
    fn naive_date_time_to_str(&self) -> Option<String>;
}

impl DateTimeUtil for Option<chrono::NaiveDateTime>{
    fn naive_date_time_to_str(&self) -> Option<String>{
        let fmt = "%Y-%m-%d %H:%M:%S";
        match self {
            None => None,
            Some(naive_date_time) => Some(naive_date_time.format(fmt).to_string()),
        }
    }
}

impl DateTimeUtil for Option<rbatis::DateTimeNative>{
    fn naive_date_time_to_str(&self) -> Option<String>{
        let fmt = "%Y-%m-%d %H:%M:%S";
        match self {
            None => None,
            Some(naive_date_time) => {
                let date = self.unwrap();
                DateTimeUtil::naive_date_time_to_str(&Some(date.inner))
            },
        }
    }
}
