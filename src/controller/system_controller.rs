use actix_web::{web, get, post, put, delete, HttpRequest, Responder};
use crate::entity::dto::journal::JournalTotalDTO;
use crate::entity::dto::log::LogPageDTO;
use crate::entity::dto::picture_base64::Base64PictureDTO;
use crate::entity::dto::plan::{PlanDTO, PlanPageDTO};
use crate::entity::dto::plan_archive::PlanArchivePageDTO;
use crate::entity::dto::sign_in::SignInDTO;
use crate::entity::dto::user::{UserDTO, UserPageDTO};
use crate::entity::vo::{RespVO};
use crate::service::CONTEXT;

/// 用户登录
pub async fn login(req: HttpRequest,arg: web::Json<SignInDTO>) -> impl Responder {
    log::info!("login:{:?}", arg.0);
    let vo = CONTEXT.system_service.login(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 用户注销
pub async fn logout(req: HttpRequest) -> impl Responder {
    let vo = CONTEXT.system_service.logout(&req).await;
    return RespVO::from(&()).resp_json();
}

/// 获取当前用户信息
#[get("/user")]
pub async fn myself(req: HttpRequest) -> impl Responder {
    let user_data = CONTEXT.system_service.user_get_info_by_token(&req).await;
    return RespVO::from_result(&user_data).resp_json();
}

/// 添加用户
#[post("/user")]
pub async fn user_add(arg: web::Json<UserDTO>) -> impl Responder {
    log::info!("user_add:{:?}", arg.0);
    let vo = CONTEXT.system_service.user_add(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 修改用户
#[put("/user")]
pub async fn user_update(req: HttpRequest, arg: web::Json<UserDTO>) -> impl Responder {
    log::info!("user_update:{:?}", arg.0);
    let vo = CONTEXT.system_service.user_edit(&req, &arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 修改密码
#[put("/user/password")]
pub async fn user_update_password(req: HttpRequest, arg: web::Json<UserDTO>) -> impl Responder {
    log::info!("user_update_password:{:?}", arg.0);
    let vo = CONTEXT.system_service.user_update_password(&req, &arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}


/// 删除用户
#[delete("/user/{user}")]
pub async fn user_remove(path: web::Path<String>) -> impl Responder {
    let user = path.into_inner();
    let vo = CONTEXT.system_service.user_remove(&user).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取指定用户详情
#[get("/user/detail/{user}")]
pub async fn user_detail(path: web::Path<String>) -> impl Responder {
    let user = path.into_inner();
    let mut user_dto = UserDTO::empty();
    user_dto.account = Some(user);
    let vo = CONTEXT.system_service.user_detail(&user_dto).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取用户分页列表
#[get("/user/page")]
pub async fn user_page(arg: web::Json<UserPageDTO>) -> impl Responder {
    let vo = CONTEXT.system_service.user_page(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取当前用户所在组织的用户列表
#[get("/user/own/organize")]
pub async fn own_organize_user(req: HttpRequest) -> impl Responder {
    let user_data = CONTEXT.system_service.user_get_own_organize(&req).await;
    return RespVO::from_result(&user_data).resp_json();
}

/// 保存用户头像
#[post("/user/logo")]
pub async fn user_upload_logo(req: HttpRequest, arg:web::Json<Base64PictureDTO>) -> impl Responder {
    let vo = CONTEXT.oss_service.upload_logo(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取日志类别列表
#[get("/log/type")]
pub async fn log_type() -> impl Responder {
    let vo = CONTEXT.system_service.log_get_type() .await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取用户分页列表
#[get("/log/page")]
pub async fn log_page(req: HttpRequest, arg: web::Json<LogPageDTO>) -> impl Responder {
    let vo = CONTEXT.system_service.log_page(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 统计近6个月的活跃情况
#[get("/log/total/pre6")]
pub async fn compute_pre6_logs(req: HttpRequest,arg: web::Json<JournalTotalDTO>) -> impl Responder {
    let vo = CONTEXT.system_service.compute_pre6_logs(&req,&arg.archive_date.clone()).await;
    return RespVO::from_result(&vo).resp_json();
}


/// 创建提醒事项
#[post("/plan")]
pub async fn add_notes(req: HttpRequest,arg: web::Json<PlanDTO>) -> impl Responder {
    log::info!("add_plan:{:?}", arg.0);
    let vo = CONTEXT.system_service.add_plan(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 修改提醒事项
#[put("/plan")]
pub async fn edit_plan(req: HttpRequest,arg: web::Json<PlanDTO>) -> impl Responder {
    log::info!("edit_plan:{:?}", arg.0);
    let vo = CONTEXT.system_service.edit_plan(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 删除提醒事项
#[delete("/plan/{id}")]
pub async fn delete_plan(req: HttpRequest,path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let vo = CONTEXT.system_service.delete_plan(&req,&id).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 分页获取当前活跃的计划提醒
#[get("/plan/page")]
pub async fn plan_page(req: HttpRequest, arg: web::Json<PlanPageDTO>) -> impl Responder {
    let vo = CONTEXT.system_service.plan_page(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 提前完成提醒事项
#[put("/plan/finish/{id}")]
pub async fn finish_plan(req: HttpRequest,path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    let vo = CONTEXT.system_service.advance_finish_news(&req,&id).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 分页获取归档计划提醒数据
#[get("/plan/archive/page")]
pub async fn plan_archive_page(req: HttpRequest, arg: web::Json<PlanArchivePageDTO>) -> impl Responder {
    let vo = CONTEXT.system_service.plan_archive_page(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}