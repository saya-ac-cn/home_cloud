use actix_web::{web,get, Responder};
use crate::entity::dto::log::LogPageDTO;
use crate::entity::vo::{JWTToken, RespVO};
use crate::service::CONTEXT;
use crate::util::{NOT_PARAMETER};

/// 获取日志类别列表
#[get("/type")]
pub async fn query_log_type() -> impl Responder {
    let vo = CONTEXT.log_service.query_log_type().await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取用户分页列表
#[get("/page")]
pub async fn page(arg: web::Json<LogPageDTO>) -> impl Responder {
    let vo = CONTEXT.log_service.page(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}