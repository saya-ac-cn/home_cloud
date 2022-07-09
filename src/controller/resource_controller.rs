use actix_web::{web, HttpRequest, Responder};

use crate::entity::dto::{ResourceDTO};
use crate::entity::vo::{JWTToken, RespVO};
use crate::service::CONTEXT;

/// 用户添加
pub async fn add(arg: web::Json<ResourceDTO>) -> impl Responder {
    let vo = CONTEXT.resource_service.add(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}
