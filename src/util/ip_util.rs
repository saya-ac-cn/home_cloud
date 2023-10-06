use crate::config::CONTEXT;
use log::error;
use serde_json::Value;
use std::collections::HashMap;

pub struct IpUtils {}

impl IpUtils {
    pub async fn city_location(ip: &String) -> String {
        let mut map: HashMap<&str, &String> = HashMap::new();
        map.insert("key", &CONTEXT.config.amap_key);
        map.insert("ip", ip);
        let client = reqwest::Client::builder().build().unwrap();

        let send_result = client
            .get(&CONTEXT.config.amap_url)
            .query(&map)
            .send()
            .await;
        if send_result.is_err() {
            error!("定位异常：{}", send_result.unwrap_err());
            return String::from("定位失败");
        }
        let read_result = send_result.unwrap().text().await;
        if read_result.is_err() {
            error!("解码定位异常：{}", read_result.unwrap_err());
            return String::from("定位失败");
        }
        let json_str = read_result.unwrap();
        let json = serde_json::from_str(json_str.as_str());
        if json.is_err() {
            error!("提取定位异常：{}", json.unwrap_err());
            return String::from("定位失败");
        }
        let location: Value = json.unwrap();
        return format!(
            "{}{}",
            location["province"].as_str().unwrap(),
            location["city"].as_str().unwrap()
        );
    }
}
