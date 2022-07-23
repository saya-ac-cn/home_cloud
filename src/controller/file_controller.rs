use actix_multipart::Multipart;
use actix_web::{web, get, post,delete, Responder, HttpRequest};
use crate::entity::dto::picture_base64::Base64PictureDTO;
use crate::entity::dto::pictures::PicturesPageDTO;
use crate::entity::vo::{RespVO};
use crate::service::CONTEXT;

/// 上传Base64类型的图片
#[post("/picture/base64")]
pub async fn upload_base64_picture(req: HttpRequest,arg: web::Json<Base64PictureDTO>) -> impl Responder {
    let vo = CONTEXT.file_service.upload_base64_picture(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 上传文件类型的图片
#[post("/picture/file")]
pub async fn upload_file_picture(req: HttpRequest,payload: Multipart) -> impl Responder {
    let vo = CONTEXT.file_service.upload_file_picture(&req,payload).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取图片分页列表
#[get("/picture/page")]
pub async fn page_picture(req: HttpRequest,arg: web::Json<PicturesPageDTO>) -> impl Responder {
    log::info!("news:{:?}", arg.0);
    let vo = CONTEXT.file_service.pictures_page(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 删除图片或壁纸
#[delete("/picture/{id}")]
pub async fn picture_delete(req: HttpRequest,path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let vo = CONTEXT.file_service.picture_delete(&req,id).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 上传文件
#[post("/files/file")]
pub async fn upload_file(req: HttpRequest,payload: Multipart) -> impl Responder {
    let vo = CONTEXT.file_service.upload_file(&req,payload).await;
    return RespVO::from_result(&vo).resp_json();
}