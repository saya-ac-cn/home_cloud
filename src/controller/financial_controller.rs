use actix_web::{web, post,get,put,delete, Responder, HttpRequest};
use crate::entity::dto::general_journal::GeneralJournalDTO;
use crate::entity::dto::journal::{JournalDTO, JournalPageDTO};
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

/// 获取流水分页列表
#[get("/journal")]
pub async fn page_journal(req: HttpRequest,arg: web::Json<JournalPageDTO>) -> impl Responder {
    let vo = CONTEXT.financial_service.journal_page(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 导出流水报表
#[get("/journal/excel")]
pub async fn excel_journal(req: HttpRequest,arg: web::Json<JournalPageDTO>) -> impl Responder {
    let result = CONTEXT.financial_service.journal_excel(&req,&arg.0).await;
    return result;
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

/// 获取流水分页列表
#[get("/general/journal")]
pub async fn detail_general_journal(req: HttpRequest,arg: web::Json<JournalPageDTO>) -> impl Responder {
    let vo = CONTEXT.financial_service.general_journal_detail(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 导出流水明细报表
#[get("/general/journal/excel")]
pub async fn excel_general_journal(req: HttpRequest,arg: web::Json<JournalPageDTO>) -> impl Responder {
    let result = CONTEXT.financial_service.general_journal_excel(&req,&arg.0).await;
    return result;
}

/// 货币列表
#[get("/dictionary/monetary")]
pub async fn get_monetary_list() -> impl Responder {
    let vo = CONTEXT.financial_service.get_monetary_list().await;
    return RespVO::from_result(&vo).resp_json();
}

/// 摘要列表
#[get("/dictionary/abstracts")]
pub async fn get_abstracts_list() -> impl Responder {
    let vo = CONTEXT.financial_service.get_abstracts_list().await;
    return RespVO::from_result(&vo).resp_json();
}

/// 收支方式列表
#[get("/dictionary/payment/means")]
pub async fn get_payment_means_list() -> impl Responder {
    let vo = CONTEXT.financial_service.get_payment_means_list().await;
    return RespVO::from_result(&vo).resp_json();
}