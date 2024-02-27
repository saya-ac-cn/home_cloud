use log::error;
use std::collections::HashMap;
use serde::Serialize;
use serde_json::Value;
use crate::conf::CONTEXT;

pub struct MessageUtil{}

impl MessageUtil {
    pub async fn send_wechat_message(openid: &str,template: &str,arg: &HashMap<&str,String>){
        let url = format!("{}/{}/{}",&CONTEXT.config.wechat_api, openid,template);
        let client = reqwest::Client::builder().build().unwrap();
        let send_result = client.post(&url).json(arg).send().await;
        if send_result.is_err() {
            error!("发送公众号消息异常：{}", send_result.unwrap_err());
            return;
        }
        let read_result = send_result.unwrap().text().await;
        if read_result.is_err() {
            error!("调用发送消息接口返回了异常的数据");
            return;
        }
        let json = serde_json::from_str(read_result.unwrap().as_str());
        if json.is_err() {
            error!("提取发送公众号消息接口回执数据异常：{}", json.unwrap_err());
            return;
        }
        let data: Value = json.unwrap();
        let status = data["data"].as_bool().unwrap();
        if !status {
            error!("发送公众号消息接口返回了异常的状态码：{}", status);
        }
    }

    pub async fn send_mail_message<T: Serialize + ?Sized>(mail: &str,template: &str,arg: &T){
        let url = format!("{}/{}/{}",&CONTEXT.config.mail_api,mail,template);
        let client = reqwest::Client::builder().build().unwrap();
        let send_result = client.post(&url).json(arg).send().await;
        if send_result.is_err() {
            error!("发送邮件异常：{}", send_result.unwrap_err());
            return;
        }
        let read_result = send_result.unwrap().text().await;
        if read_result.is_err() {
            error!("调用发送邮件接口返回了异常的数据");
            return;
        }
        let json = serde_json::from_str(read_result.unwrap().as_str());
        if json.is_err() {
            error!("提取发送邮件接口回执数据异常：{}", json.unwrap_err());
            return;
        }
        let data: Value = json.unwrap();
        let status = data["data"].as_bool().unwrap();
        if !status {
            error!("发送邮件接口返回了异常的状态码：{}", status);
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::util::message_util::MessageUtil;

    //#[test]
    // #[tokio::test]
    // async fn test_wechat() {
    //     let mut data: HashMap<&str, String> = HashMap::new();
    //     data.insert("user", String::from("lalala"));
    //     data.insert("title", String::from("挂号"));
    //     data.insert("content", String::from("做好一周后的挂号"));
    //     MessageUtil::send_wechat_message("o7Ubt6V3S_8DeGVh9D3VnTQMnHQw","zrxz2Ocx2ugFceMB6gQ1m_KU5SU69EI2p5nzp5NVQgY",&data).await;
    // }

    #[tokio::test]
    async fn test_mail() {
        let mut data: HashMap<&str, String> = HashMap::new();
        data.insert("archive_date", String::from("2024-02-01"));
        data.insert("start_date", String::from("2024-02-01 21:40:00"));
        data.insert("end_date", String::from("2024-02-01 21:40:00"));
        MessageUtil::send_mail_message("la-lalala@qq.com","dump",&data).await;
    }
}