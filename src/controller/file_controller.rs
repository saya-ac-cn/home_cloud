use actix_web::{web, get,post, Responder, HttpRequest};
use crate::entity::dto::picture_base64::Base64PictureDTO;
use crate::entity::vo::{RespVO};
use crate::service::CONTEXT;

/// 获取日志类别列表
#[post("/picture/base64")]
pub async fn upload_base64_picture(req: HttpRequest,arg: web::Json<Base64PictureDTO>) -> impl Responder {
    let vo = CONTEXT.file_service.upload_base64_picture(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}