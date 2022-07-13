use actix_multipart::Multipart;
use actix_web::{web, get, post, put, delete, HttpRequest, Responder};
use crate::entity::dto::sign_in::SignInDTO;
use crate::entity::dto::user::{UserDTO, UserPageDTO};
use crate::entity::vo::{RespVO};
use crate::service::CONTEXT;

/// 用户登录
pub async fn login(req: HttpRequest,arg: web::Json<SignInDTO>) -> impl Responder {
    log::info!("login:{:?}", arg.0);
    let vo = CONTEXT.user_service.login(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 用户注销
pub async fn logout(req: HttpRequest) -> impl Responder {
    let vo = CONTEXT.user_service.logout(&req).await;
    return RespVO::from(&()).resp_json();
}

/// 获取当前用户信息
#[get("/")]
pub async fn myself(req: HttpRequest) -> impl Responder {
    let user_data = CONTEXT.user_service.get_user_info_by_token(&req).await;
    return RespVO::from_result(&user_data).resp_json();
}

/// 添加用户
#[post("/")]
pub async fn add(arg: web::Json<UserDTO>) -> impl Responder {
    let vo = CONTEXT.user_service.add(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 修改用户
#[put("/")]
pub async fn update(req: HttpRequest,arg: web::Json<UserDTO>) -> impl Responder {
    let vo = CONTEXT.user_service.edit(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 修改密码
#[put("/password")]
pub async fn update_password(req: HttpRequest,arg: web::Json<UserDTO>) -> impl Responder {
    let vo = CONTEXT.user_service.update_password(&req,&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}


/// 删除用户
#[delete("/{user}")]
pub async fn remove(path: web::Path<String>) -> impl Responder {
    let user = path.into_inner();
    let vo = CONTEXT.user_service.remove(&user).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取指定用户详情
#[get("/detail/{user}")]
pub async fn detail(path: web::Path<String>) -> impl Responder {
    let user = path.into_inner();
    let mut user_dto = UserDTO::empty();
    user_dto.account = Some(user);
    let vo = CONTEXT.user_service.detail(&user_dto).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取用户分页列表
#[get("/page")]
pub async fn page(arg: web::Json<UserPageDTO>) -> impl Responder {
    let vo = CONTEXT.user_service.page(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取当前用户所在组织的用户列表
#[get("/own/organize")]
pub async fn own_organize_user(req: HttpRequest) -> impl Responder {
    let user_data = CONTEXT.user_service.get_own_organize_user(&req).await;
    return RespVO::from_result(&user_data).resp_json();
}

/// 上传logo
#[post("/logo/{user}")]
pub async fn upload_logo(path: web::Path<String>,payload: Multipart) -> impl Responder {
    let user = path.into_inner();
    println!("user:{}",user);
    let vo = CONTEXT.user_service.upload_logo(payload).await;
    let aaa:RespVO<String> = RespVO{code:Some(0), msg: None, data: None };
    return aaa.resp_json();
}

// #[post("/logo")]
// pub async fn upload_logo(form: web::Form<SignInDTO>) -> impl Responder {
//     println!("from data:{:?}",form);
//     let aaa:RespVO<String> = RespVO{code:Some(0), msg: None, data: None };
//     return aaa.resp_json();
// }