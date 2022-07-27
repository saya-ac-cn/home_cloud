use actix_web::{web, post,get,put,delete, Responder, HttpRequest};
use crate::entity::dto::news::{NewsDTO, NewsPageDTO};
use crate::entity::vo::{RespVO};
use crate::service::CONTEXT;

/// 添加动态
#[post("/news")]
pub async fn add_news(req: HttpRequest,arg: web::Json<NewsDTO>) -> impl Responder {
    log::info!("add_news:{:?}", arg.0);
    let vo = CONTEXT.content_service.add_news(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 修改动态
#[put("/news")]
pub async fn edit_news(req: HttpRequest,arg: web::Json<NewsDTO>) -> impl Responder {
    log::info!("edit_news:{:?}", arg.0);
    let vo = CONTEXT.content_service.edit_news(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}


/// 删除动态
#[delete("/news/{id}")]
pub async fn delete_news(req: HttpRequest,path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let vo = CONTEXT.content_service.delete_news(&req,&id).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取指定id的动态
#[get("/news/{id}")]
pub async fn get_news(path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let vo = CONTEXT.content_service.get_news_detail(&id).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取动态分页列表
#[get("/news")]
pub async fn page_news(req: HttpRequest,arg: web::Json<NewsPageDTO>) -> impl Responder {
    let vo = CONTEXT.content_service.news_page(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}