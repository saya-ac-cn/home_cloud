use actix_multipart::Multipart;
use actix_web::{web, get, post,put,delete, Responder, HttpRequest};
use crate::entity::dto::files::{FilesDTO, FilesPageDTO};
use crate::entity::dto::picture_base64::Base64PictureDTO;
use crate::entity::dto::pictures::PicturesPageDTO;
use crate::entity::vo::{RespVO};
use crate::service::CONTEXT;

/// 上传Base64类型的图片
#[post("/picture/base64")]
pub async fn upload_base64_picture(req: HttpRequest,arg: web::Json<Base64PictureDTO>) -> impl Responder {
    let vo = CONTEXT.oss_service.upload_base64_picture(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 上传文件类型的图片
#[post("/picture/file")]
pub async fn upload_file_picture(req: HttpRequest,payload: Multipart) -> impl Responder {
    let vo = CONTEXT.oss_service.upload_file_picture(&req,payload).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取图片分页列表
#[get("/picture/page")]
pub async fn page_picture(req: HttpRequest,arg: web::Query<PicturesPageDTO>) -> impl Responder {
    log::info!("page_picture:{:?}", arg.clone().into_inner());
    let vo = CONTEXT.oss_service.pictures_page(&req,&arg.into_inner()).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 删除图片或壁纸
#[delete("/picture/{id}")]
pub async fn picture_delete(req: HttpRequest,path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let vo = CONTEXT.oss_service.picture_delete(&req,id).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 上传文件
#[post("/files/file")]
pub async fn upload_file(req: HttpRequest,payload: Multipart) -> impl Responder {
    let vo = CONTEXT.oss_service.upload_file(&req,payload).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取文件分页列表
#[get("/files/page")]
pub async fn page_files(req: HttpRequest,arg: web::Query<FilesPageDTO>) -> impl Responder {
    log::info!("page_files:{:?}", arg.clone().into_inner());
    let vo = CONTEXT.oss_service.files_page(&req,&arg.into_inner()).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 删除文件
#[delete("/files")]
pub async fn files_delete(req: HttpRequest,arg: web::Query<FilesDTO>) -> impl Responder {
    log::info!("files_delete:{:?}", arg.clone().into_inner());
    let vo = CONTEXT.oss_service.files_delete(&req,&arg.into_inner()).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 修改文件
#[put("/files/file")]
pub async fn files_edit(req: HttpRequest,arg:web::Json<FilesDTO>) -> impl Responder {
    log::info!("file_edit:{:?}", arg.0);
    let vo = CONTEXT.oss_service.files_update(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 下载文件
#[get("/files/download/{id}")]
pub async fn files_download(path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let vo = CONTEXT.oss_service.files_download(id).await;
    return vo;
}