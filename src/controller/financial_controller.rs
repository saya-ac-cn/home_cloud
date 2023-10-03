use crate::entity::dto::general_journal::GeneralJournalDTO;
use crate::entity::dto::journal::{JournalDTO, JournalPageDTO, JournalTotalDTO};
use crate::entity::vo::RespVO;
use crate::service::CONTEXT;
use actix_web::{delete, get, post, put, web, HttpRequest, Responder};

/// 申报流水
#[post("/journal")]
pub async fn add_journal(req: HttpRequest, arg: web::Json<JournalDTO>) -> impl Responder {
    log::info!("add_journal:{:?}", arg.0);
    let vo = CONTEXT.financial_service.add_journal(&req, &arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 修改流水
#[put("/journal")]
pub async fn edit_journal(req: HttpRequest, arg: web::Json<JournalDTO>) -> impl Responder {
    log::info!("edit_journal:{:?}", arg.0);
    let vo = CONTEXT.financial_service.edit_journal(&req, &arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 删除流水
#[delete("/journal/{id}")]
pub async fn delete_journal(req: HttpRequest, path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let vo = CONTEXT.financial_service.delete_journal(&req, &id).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取流水分页列表
#[get("/journal")]
pub async fn page_journal(req: HttpRequest, arg: web::Query<JournalPageDTO>) -> impl Responder {
    log::info!("page_journal:{:?}", arg.clone().into_inner());
    let vo = CONTEXT
        .financial_service
        .journal_page(&req, &arg.into_inner())
        .await;
    return RespVO::from_result(&vo).resp_json();
}

/// 导出流水报表
#[get("/journal/excel")]
pub async fn excel_journal(req: HttpRequest, arg: web::Query<JournalPageDTO>) -> impl Responder {
    log::info!("excel_journal:{:?}", arg.clone().into_inner());
    let result = CONTEXT
        .financial_service
        .journal_excel(&req, &arg.into_inner())
        .await;
    return result;
}

/// 添加流水明细
#[post("/general/journal")]
pub async fn add_general_journal(
    req: HttpRequest,
    arg: web::Json<GeneralJournalDTO>,
) -> impl Responder {
    log::info!("add_general_journal:{:?}", arg.0);
    let vo = CONTEXT
        .financial_service
        .add_general_journal(&req, &arg.0)
        .await;
    return RespVO::from_result(&vo).resp_json();
}

/// 修改流水明细
#[put("/general/journal")]
pub async fn edit_general_journal(
    req: HttpRequest,
    arg: web::Json<GeneralJournalDTO>,
) -> impl Responder {
    log::info!("edit_general_journal:{:?}", arg.0);
    let vo = CONTEXT
        .financial_service
        .edit_general_journal(&req, &arg.0)
        .await;
    return RespVO::from_result(&vo).resp_json();
}

/// 删除流水明细
#[delete("/general/journal/{id}")]
pub async fn delete_general_journal(req: HttpRequest, path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let vo = CONTEXT
        .financial_service
        .delete_general_journal(&req, &id)
        .await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取流水列表
#[get("/general/journal")]
pub async fn detail_general_journal(
    req: HttpRequest,
    arg: web::Query<JournalPageDTO>,
) -> impl Responder {
    log::info!("detail_general_journal:{:?}", arg.clone().into_inner());
    let vo = CONTEXT
        .financial_service
        .general_journal_detail(&req, &arg.into_inner())
        .await;
    return RespVO::from_result(&vo).resp_json();
}

/// 导出流水明细报表
#[get("/general/journal/excel")]
pub async fn excel_general_journal(
    req: HttpRequest,
    arg: web::Query<JournalPageDTO>,
) -> impl Responder {
    log::info!("excel_general_journal:{:?}", arg.clone().into_inner());
    let result = CONTEXT
        .financial_service
        .general_journal_excel(&req, &arg.into_inner())
        .await;
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

/// 分页按天流水汇总数据
#[get("/journal/day")]
pub async fn page_journal_collect(
    req: HttpRequest,
    arg: web::Query<JournalPageDTO>,
) -> impl Responder {
    log::info!("page_journal_collect:{:?}", arg.clone().into_inner());
    let vo = CONTEXT
        .financial_service
        .journal_collect_page(&req, &arg.into_inner())
        .await;
    return RespVO::from_result(&vo).resp_json();
}

/// 导出流水明细汇总
#[get("/journal/collect/excel")]
pub async fn excel_journal_collect(
    req: HttpRequest,
    arg: web::Query<JournalPageDTO>,
) -> impl Responder {
    log::info!("excel_journal_collect:{:?}", arg.clone().into_inner());
    let result = CONTEXT
        .financial_service
        .journal_collect_excel(&req, &arg.into_inner())
        .await;
    return result;
}

/// 计算收支增长率
#[get("/journal/total/balance")]
pub async fn compute_account_growth_rate(
    req: HttpRequest,
    arg: web::Query<JournalTotalDTO>,
) -> impl Responder {
    log::info!("compute_account_growth_rate:{:?}", arg.clone().into_inner());
    let vo = CONTEXT
        .financial_service
        .compute_account_growth_rate(&req, &arg.archive_date.clone())
        .await;
    return RespVO::from_result(&vo).resp_json();
}

/// 计算指定月份的收入比重
#[get("/journal/total/income")]
pub async fn compute_income_percentage(
    req: HttpRequest,
    arg: web::Query<JournalTotalDTO>,
) -> impl Responder {
    log::info!("compute_income_percentage:{:?}", arg.clone().into_inner());
    let vo = CONTEXT
        .financial_service
        .compute_income_percentage(&req, &arg.archive_date.clone())
        .await;
    return RespVO::from_result(&vo).resp_json();
}

/// 统计指定月份中各摘要的排名
#[get("/journal/total/order")]
pub async fn order_month_journal(
    req: HttpRequest,
    arg: web::Query<JournalTotalDTO>,
) -> impl Responder {
    log::info!("order_month_journal:{:?}", arg.clone().into_inner());
    let vo = CONTEXT
        .financial_service
        .order_month_journal(&req, &arg.archive_date.clone())
        .await;
    return RespVO::from_result(&vo).resp_json();
}

/// 统算近6个月的财务流水
#[get("/journal/total/pre6")]
pub async fn compute_pre6_journal(
    req: HttpRequest,
    arg: web::Query<JournalTotalDTO>,
) -> impl Responder {
    log::info!("compute_pre6_journal:{:?}", arg.clone().into_inner());
    let vo = CONTEXT
        .financial_service
        .compute_pre6_journal(&req, &arg.archive_date.clone())
        .await;
    return RespVO::from_result(&vo).resp_json();
}
