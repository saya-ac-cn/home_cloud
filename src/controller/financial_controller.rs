use actix_web::{web, post,get,put,delete, Responder, HttpRequest};
use crate::entity::dto::general_journal::GeneralJournalDTO;
use crate::entity::dto::journal::{JournalDTO};
use crate::entity::vo::{RespVO};
use crate::service::CONTEXT;

/// 申报流水
#[post("/journal")]
pub async fn add_journal(req: HttpRequest,arg: web::Json<JournalDTO>) -> impl Responder {
    log::info!("add_journal:{:?}", arg.0);
    let vo = CONTEXT.financial_service.add_journal(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 修改流水
#[put("/journal")]
pub async fn edit_journal(req: HttpRequest,arg: web::Json<JournalDTO>) -> impl Responder {
    log::info!("edit_journal:{:?}", arg.0);
    let vo = CONTEXT.financial_service.edit_journal(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 删除流水
#[delete("/journal/{id}")]
pub async fn delete_journal(req: HttpRequest,path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let vo = CONTEXT.financial_service.delete_journal(&req,&id).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 添加流水明细
#[post("/general/journal")]
pub async fn add_general_journal(req: HttpRequest,arg: web::Json<GeneralJournalDTO>) -> impl Responder {
    log::info!("add_general_journal:{:?}", arg.0);
    let vo = CONTEXT.financial_service.add_general_journal(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 修改流水明细
#[put("/general/journal")]
pub async fn edit_general_journal(req: HttpRequest,arg: web::Json<GeneralJournalDTO>) -> impl Responder {
    log::info!("edit_general_journal:{:?}", arg.0);
    let vo = CONTEXT.financial_service.edit_general_journal(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 删除流水明细
#[delete("/general/journal/{id}")]
pub async fn delete_general_journal(req: HttpRequest,path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let vo = CONTEXT.financial_service.delete_general_journal(&req,&id).await;
    return RespVO::from_result(&vo).resp_json();
}