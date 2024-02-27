use crate::conf::CONTEXT;
use crate::util;
use chrono::Local;
use log::error;
use rustflake::Snowflake;
use std::time::Duration;

/// token工具类
pub struct TokenUtils {}

impl TokenUtils {
    /// 在redis中为其生成一个token
    pub async fn create_token() -> String {
        let token = Snowflake::default().generate().to_string();
        CONTEXT
            .redis_client
            .set_string_ex(
                &format!("{:}:{:}", &util::REQUEST_TOKEN_PREFIX, &token),
                &Local::now().to_string(),
                Some(Duration::from_secs(86400)),
            )
            .await;
        token
    }

    /// 校验token是否有效，None -> 无效，Some(true) -> 有效
    pub async fn check_token(token: Option<String>) -> Option<bool> {
        // token的非空校验
        if token.is_none() {
            return None;
        }
        let token = token.unwrap();
        if token.is_empty() {
            return None;
        }
        // 尝试去删除，删除返回1表示之前有，且本次删除成功
        return match CONTEXT
            .redis_client
            .delete(&format!("{:}:{:}", &util::REQUEST_TOKEN_PREFIX, &token))
            .await
        {
            Ok(result) => match 1 == result {
                true => Some(true),
                false => None,
            },
            Err(e) => {
                error!("check redis user token cache data fail! token:{:?}", e);
                None
            }
        };
    }
}
