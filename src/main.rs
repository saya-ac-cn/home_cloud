use actix_web::{web, App, HttpServer};
use home_cloud::controller::system_controller;
use home_cloud::middleware::auth_actix::Auth;
use home_cloud::service::CONTEXT;

/// use tokio,because Rbatis specifies the runtime-tokio
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // log
    home_cloud::config::log::init_log();
    // database
    CONTEXT.init_pool().await;
    // router
    HttpServer::new(|| {
        App::new()
            .wrap(Auth {})
            // 登录登出接口单独处理（因为都不在已有的分组中）
            .route("/backend/login", web::post().to(system_controller::login))
            .route("/backend/logout", web::post().to(system_controller::logout))
            // TODO 映射静态资源目录
            // .service(fs::Files::new("/warehouse", &CONTEXT.config.data_dir))
            .service(
                web::scope("/backend/system")
                    .service(system_controller::token)
                    .service(system_controller::myself)
                    .service(system_controller::user_add)
                    .service(system_controller::user_update)
                    .service(system_controller::user_detail)
                    .service(system_controller::user_remove)
                    .service(system_controller::user_page)
                    .service(system_controller::user_update_password)
                    .service(system_controller::log_page)
                    .service(system_controller::log_excel)
                    .service(system_controller::log_type)
            )

    })
    .bind(&CONTEXT.config.server_url)?
    .run()
    .await
}
