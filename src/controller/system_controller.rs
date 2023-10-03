use crate::entity::dto::log::LogPageDTO;
use crate::entity::dto::sign_in::SignInDTO;
use crate::entity::dto::user::{UserDTO, UserPageDTO};
use crate::entity::vo::RespVO;
use crate::service::CONTEXT;
use actix_web::{delete, get, post, put, web, HttpRequest, Responder};

/// 生成token
#[get("/token")]
pub async fn token() -> impl Responder {
    let vo = CONTEXT.system_service.token().await;
    return RespVO::from_result(&vo).resp_json();
}

/// 用户登录
pub async fn login(req: HttpRequest, arg: web::Json<SignInDTO>) -> impl Responder {
    log::info!("login:{:?}", arg.0);
    let vo = CONTEXT.system_service.login(&req, &arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 用户注销
pub async fn logout(req: HttpRequest) -> impl Responder {
    let vo = CONTEXT.system_service.logout(&req).await;
    return RespVO::from_result(&vo).resp_json();
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
    let vo = CONTEXT
        .system_service
        .user_update_password(&req, &arg.0)
        .await;
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


/// 获取日志类别列表
#[get("/log/type")]
pub async fn log_type() -> impl Responder {
    let vo = CONTEXT.system_service.log_get_type().await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取日志分页列表
#[get("/log/page")]
pub async fn log_page(req: HttpRequest, arg: web::Query<LogPageDTO>) -> impl Responder {
    log::info!("log_page:{:?}", arg.clone().into_inner());
    let vo = CONTEXT
        .system_service
        .log_page(&req, &arg.into_inner())
        .await;
    return RespVO::from_result(&vo).resp_json();
}

/// 导出日志
#[get("/log/excel")]
pub async fn log_excel(req: HttpRequest, arg: web::Query<LogPageDTO>) -> impl Responder {
    log::info!("log_excel:{:?}", arg.clone().into_inner());
    let result = CONTEXT
        .system_service
        .log_excel(&req, &arg.into_inner())
        .await;
    return result;
}

