use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::error::Error;
use crate::service::CONTEXT;
use crate::util;

pub mod jwt;
pub mod sign_in;
pub mod user;
pub mod log;
pub mod log_type;
pub mod news;
pub mod pictures;
pub mod files;
pub mod memo;
pub mod notebook;
pub mod notes;

/// http接口返回模型结构，提供基础的 code，msg，data 等json数据结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespVO<T> {
    pub code: Option<i32>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T> where T: Serialize + DeserializeOwned + Clone, {

    pub fn from_result(arg: &Result<T, Error>) -> Self {
        if arg.is_ok() {
            Self {
                code: Some(util::CODE_SUCCESS),
                msg: Some(String::from("操作成功")),
                data: arg.clone().ok(),
            }
        } else {
            // 不能直接使用clone，否则code将丢失，除非你重写，当然，在这里已经重写了！
            let error = arg.clone().err().unwrap();
            // 下面这一部分代码主要是把服务器内部的错误给予屏蔽，除非是用户明确想要暴露出来的
            if let Error::E(message, code) = error {
                if util::UNKNOWN_ERROR == code {
                    // 由服务器内部抛出的异常，异常的详情需要给予截留，敏感信息不对外暴露
                    Self {
                        code: Some(code),
                        msg: Some(String::from("处理失败，请稍后再试")),
                        data: None
                    }
                } else {
                    // 非敏感的信息，可以正常的对外展示
                    Self {
                        code: Some(code),
                        msg: Some(message),
                        data: None
                    }
                }
            } else {
                // 没有匹配到规定的错误异常消息，直接按默认打印即可
                Self {
                    code: Some(util::CODE_FAIL),
                    msg: Some(arg.clone().err().unwrap().to_string()),
                    data: None
                }
            }
        }
    }

    pub fn from(arg: &T) -> Self {
        Self {
            code: Some(util::CODE_SUCCESS),
            msg: Some(String::from("操作成功")),
            data: Some(arg.clone()),
        }
    }

    pub fn from_error(code: i32, arg: &Error) -> Self {
        Self {
            code: Some(code),
            msg: Some(arg.to_string()),
            data: None,
        }
    }

    pub fn from_error_info(code: i32, info: &str) -> Self {
        Self {
            code: Some(code),
            msg: Some(info.to_string()),
            data: None,
        }
    }

    pub fn resp_json(&self) -> HttpResponse {
        if CONTEXT.config.debug {
            println!("[home_cloud][debug] resp:{}", self.to_string());
        }
        return HttpResponse::Ok()
            .insert_header(("Access-Control-Allow-Origin", "*"))
            .insert_header(("Cache-Control", "no-cache"))
            .insert_header(("Content-Type", "text/json;charset=UTF-8"))
            .body(self.to_string());
    }
}

impl<T> ToString for RespVO<T>
    where
        T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}