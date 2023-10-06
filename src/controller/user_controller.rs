use actix_web::{get, web, Responder, post, put, HttpRequest, delete};
use crate::domain::dto::user::{UserDTO, UserPageDTO};
use crate::config::CONTEXT;
use crate::domain::vo::RespVO;

/// 获取用户分页列表
#[get("/user/page")]
pub async fn user_page(arg: web::Json<UserPageDTO>) -> impl Responder {
    let vo = CONTEXT.user_service.user_page(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 添加用户
#[post("/user")]
pub async fn user_add(arg: web::Json<UserDTO>) -> impl Responder {
    log::info!("user_add:{:?}", arg.0);
    let vo = CONTEXT.user_service.user_add(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 修改用户
#[put("/user")]
pub async fn user_update(req: HttpRequest, arg: web::Json<UserDTO>) -> impl Responder {
    log::info!("user_update:{:?}", arg.0);
    let vo = CONTEXT.user_service.user_edit(&req, &arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 删除用户
#[delete("/user/{user}")]
pub async fn user_remove(path: web::Path<String>) -> impl Responder {
    let user = path.into_inner();
    let vo = CONTEXT.user_service.user_remove(&user).await;
    return RespVO::from_result(&vo).resp_json();
}


/// 获取指定用户详情
#[get("/user/detail/{user}")]
pub async fn user_detail(path: web::Path<String>) -> impl Responder {
    let user = path.into_inner();
    let mut user_dto = UserDTO::empty();
    user_dto.account = Some(user);
    let vo = CONTEXT.user_service.user_detail(&user_dto).await;
    return RespVO::from_result(&vo).resp_json();
}