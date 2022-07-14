pub mod jwt;
pub mod sign_in;
pub mod user;
pub mod log;
pub mod log_type;


use crate::error::Error;
use crate::service::CONTEXT;
use actix_web::HttpResponse;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use crate::util::{CODE_SUCCESS,CODE_FAIL};


/// http接口返回模型结构，提供基础的 code，msg，data 等json数据结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespVO<T> {
    pub code: Option<i32>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T>
    where
        T: Serialize + DeserializeOwned + Clone,
{

    pub fn build_error(code:i32,message:&str) -> Self{
         RespVO {
            code: Some(code),
            msg: Some(message.to_string()),
            data: None,
        }
    }

    pub fn from_result(arg: &Result<T, Error>) -> Self {
        if arg.is_ok() {
            Self {
                code: Some(CODE_SUCCESS),
                msg: Some(String::from("操作成功")),
                data: arg.clone().ok(),
            }
        } else {
            Self {
                code: Some(CODE_FAIL),
                msg: Some(arg.clone().err().unwrap().to_string()),
                data: None,
            }
        }
    }

    pub fn from(arg: &T) -> Self {
        Self {
            code: Some(CODE_SUCCESS),
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
