use actix_web::{web,get,post,put,delete, HttpRequest, Responder};
use crate::entity::dto::{UserDTO, UserPageDTO};
use crate::entity::dto::sign_in::SignInDTO;
use crate::entity::vo::{JWTToken, RespVO};
use crate::service::CONTEXT;
use crate::util::{NOT_PARAMETER};

/// 用户登录
pub async fn login(arg: web::Json<SignInDTO>) -> impl Responder {
    log::info!("login:{:?}", arg.0);
    let vo = CONTEXT.user_service.login(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 用户注销
pub async fn logout() -> impl Responder {
    let vo = CONTEXT.user_service.logout().await;
    return RespVO::from(&()).resp_json();
}

/// 获取当前用户信息
#[get("/")]
pub async fn myself(req: HttpRequest) -> impl Responder {
    let token = req.headers().get("access_token");
    match token {
        Some(token) => {
            let token = token.to_str().unwrap_or("");
            let token = JWTToken::verify(&CONTEXT.config.jwt_secret, token);
            if token.is_err() {
                return RespVO::from_result(&token).resp_json();
            }
            let user_data = CONTEXT
                .user_service
                .get_user_info_by_token(&token.unwrap())
                .await;
            return RespVO::from_result(&user_data).resp_json();
        }
        _ => {
            return RespVO::<String>::from_error_info(NOT_PARAMETER,"access_token is empty!").resp_json();
        }
    }
}

/// 添加用户
#[post("/")]
pub async fn add(arg: web::Json<UserDTO>) -> impl Responder {
    let vo = CONTEXT.user_service.add(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 修改用户
#[put("/")]
pub async fn update(arg: web::Json<UserDTO>) -> impl Responder {
    let vo = CONTEXT.user_service.edit(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}


/// 删除用户
#[delete("/{user}")]
pub async fn remove(path: web::Path<(String)>) -> impl Responder {
    let (user) = path.into_inner();
    let vo = CONTEXT.user_service.remove(&user).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取指定用户详情
#[get("/detail/{user}")]
pub async fn detail(path: web::Path<(String)>) -> impl Responder {
    let (user) = path.into_inner();
    let mut userDTO = UserDTO::empty();
    userDTO.account = Some(user);
    let vo = CONTEXT.user_service.detail(&userDTO).await;
    return RespVO::from_result(&vo).resp_json();
}

/// 获取用户分页列表
#[get("/page")]
pub async fn page(arg: web::Json<UserPageDTO>) -> impl Responder {
    let vo = CONTEXT.user_service.page(&arg.0).await;
    return RespVO::from_result(&vo).resp_json();
}