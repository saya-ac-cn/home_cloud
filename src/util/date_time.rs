use std::ops::Add;
use chrono::{Datelike, Duration, NaiveDate, Timelike};

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

// 不建议使用rbatis自带的时间
// impl DateTimeUtil for Option<rbatis::DateNative>{
//     fn naive_date_time_to_str(&self,format:&str) -> Option<String>{
//         match self {
//             None => None,
//             Some(naive_date_time) => {
//                 let date = self.unwrap();
//                 Some(date.format(format).to_string())
//             },
//         }
//     }
// }

impl DateTimeUtil for Option<chrono::NaiveDateTime>{
    fn naive_date_time_to_str(&self,format:&str) -> Option<String>{
        match self {
            None => None,
            Some(naive_date_time) => Some(naive_date_time.format(format).to_string()),
        }
    }
}

impl DateTimeUtil for Option<chrono::DateTime<chrono::FixedOffset>>{
    fn naive_date_time_to_str(&self,format:&str) -> Option<String>{
        match self {
            None => None,
            Some(date_time) => Some(date_time.format(format).to_string()),
        }
    }
}

pub struct DateUtils{}

impl DateUtils {
    /// 获取指定月份的天数
    pub fn get_current_month_days(year: i32,month: u32) -> u32{
        match month {
            2 if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 => 29,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }

    // 月份加减
    pub fn month_compute(original_date:&NaiveDate,val:i32) -> NaiveDate{
        let year = original_date.year();
        let mut month=original_date.month();
        if val < 0 {
            // 是减
            let month:i32 = month as i32 + val;
            if month<=0 {
                // 年份要变
                let _year:i32 = year - (month.abs() / 12) as i32 - 1;
                let _month = month % 12 + 12;
                original_date.clone().with_year(_year).unwrap().with_month(_month as u32).unwrap()
            }else {
                // 年份不变
                original_date.clone().with_month(month as u32).unwrap()
            }
        }else {
            // 是加
            month = month+ (val.abs() as u32);
            if month <= 12  {
                // 年份不变
                original_date.clone().with_month(month).unwrap()
            }else{
                let _year = year + (month / 12) as i32;
                let _month = if month % 12 == 0 { 12 }else { month % 12 };
                original_date.clone().with_year(_year).unwrap().with_month(_month).unwrap()
            }
        }
    }

    /// 对计划的日期进行加运算
    pub fn plan_data_compute(original :&chrono::NaiveDateTime,cycle :u32,unit :u32) -> chrono::NaiveDateTime{
        // cycle= 1：一次性，2：天，3：周，4：月，5：年
        let param = unit as i64;
        let convert_one_result = if 2==cycle {
            original.add(Duration::days(param))
        } else if 3==cycle {
            original.add(Duration::weeks(param))
        } else if 4 == cycle {
            let original_date = original.date();
            let convert_date = DateUtils::month_compute(&original_date,unit as i32);
            original.clone().with_year(convert_date.year()).unwrap().with_month(convert_date.month()).unwrap()
        } else if 5 == cycle {
            let new_year = original.year() + (unit as i32);
            original.clone().with_year(new_year).unwrap()
        } else {
            original.clone()
        };
        return convert_one_result;
    }

    /// 根据日期时间生成cron表达式，其中秒一律按0处理，年份按照*，每年执行一次，因为cron_table框架不支持一次性的定时调度，特殊处理成每年执行一次，然后删除
    pub fn data_time_to_cron(data_time :&chrono::NaiveDateTime) -> String {
        let month = data_time.month();
        let day = data_time.day();
        let hour = data_time.hour();
        let minute = data_time.minute();
        format!(" 0 {} {} {} {} *",minute,hour,day,month)
    }

    /// 根据时区返回当前时间
    pub fn now() -> chrono::DateTime<chrono::FixedOffset>{
        // 世界时间
        let utc = chrono::Utc::now();
        // 东8区
        //let east8:chrono::FixedOffset = chrono::FixedOffset::east(8 * 3600);
        let east8:chrono::FixedOffset = chrono::FixedOffset::east_opt(8 * 3600).unwrap();
        utc.with_timezone(&east8)
    }

    /// 根据时区返回当前时间
    pub fn now_string() -> chrono::DateTime<chrono::FixedOffset>{
        // 世界时间
        let utc = chrono::Utc::now();
        // 东8区
        let east8:chrono::FixedOffset = chrono::FixedOffset::east_opt(8 * 3600).unwrap();
        utc.with_timezone(&east8)
    }
}
