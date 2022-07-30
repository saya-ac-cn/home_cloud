use actix_web::{web, post,get,put,delete, Responder, HttpRequest};
use crate::entity::dto::memo::{MemoDTO, MemoPageDTO};
use crate::entity::dto::news::{NewsDTO, NewsPageDTO};
use crate::entity::dto::notebook::NoteBookDTO;
use crate::entity::dto::notes::{NotesDTO, NotesPageDTO};
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

/// 添加便笺
#[post("/memo")]
pub async fn add_memo(req: HttpRequest,arg: web::Json<MemoDTO>) -> impl Responder {
    log::info!("add_memo:{:?}", arg.0);
    let vo = CONTEXT.content_service.add_memo(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 修改便笺
#[put("/memo")]
pub async fn edit_memo(req: HttpRequest,arg: web::Json<MemoDTO>) -> impl Responder {
    log::info!("edit_memo:{:?}", arg.0);
    let vo = CONTEXT.content_service.edit_memo(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}


/// 删除便笺
#[delete("/memo/{id}")]
pub async fn delete_memo(req: HttpRequest,path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let vo = CONTEXT.content_service.delete_memo(&req,&id).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取指定id的便笺
#[get("/memo/{id}")]
pub async fn get_memo(path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let vo = CONTEXT.content_service.get_memo_detail(&id).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取便笺分页列表
#[get("/memo")]
pub async fn page_memo(req: HttpRequest,arg: web::Json<MemoPageDTO>) -> impl Responder {
    let vo = CONTEXT.content_service.page_memo(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 添加笔记簿
#[post("/notebook")]
pub async fn add_notebook(req: HttpRequest,arg: web::Json<NoteBookDTO>) -> impl Responder {
    log::info!("add_notebook:{:?}", arg.0);
    let vo = CONTEXT.content_service.add_notebook(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 修改笔记簿
#[put("/notebook")]
pub async fn edit_notebook(req: HttpRequest,arg: web::Json<NoteBookDTO>) -> impl Responder {
    log::info!("edit_notebook:{:?}", arg.0);
    let vo = CONTEXT.content_service.edit_notebook(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}


/// 删除笔记簿
#[delete("/notebook/{id}")]
pub async fn delete_notebook(req: HttpRequest,path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let vo = CONTEXT.content_service.delete_notebook(&req,&id).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取笔记簿列表
#[get("/notebook")]
pub async fn notebook_list(req: HttpRequest,arg: web::Json<NoteBookDTO>) -> impl Responder {
    let vo = CONTEXT.content_service.list_notebook(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 添加笔记
#[post("/notes")]
pub async fn add_notes(req: HttpRequest,arg: web::Json<NotesDTO>) -> impl Responder {
    log::info!("add_notes:{:?}", arg.0);
    let vo = CONTEXT.content_service.add_notes(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 修改笔记
#[put("/notes")]
pub async fn edit_notes(req: HttpRequest,arg: web::Json<NotesDTO>) -> impl Responder {
    log::info!("edit_notes:{:?}", arg.0);
    let vo = CONTEXT.content_service.edit_notes(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}


/// 删除笔记
#[delete("/notes/{id}")]
pub async fn delete_notes(req: HttpRequest,path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let vo = CONTEXT.content_service.delete_notes(&req,&id).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取指定id的笔记
#[get("/notes/{id}")]
pub async fn get_notes(path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let vo = CONTEXT.content_service.get_notes_detail(&id).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取笔记分页列表
#[get("/notes")]
pub async fn page_notes(req: HttpRequest,arg: web::Json<NotesPageDTO>) -> impl Responder {
    let vo = CONTEXT.content_service.page_notes(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}