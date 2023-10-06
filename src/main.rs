use actix_web::{App, HttpServer, web};
use home_cloud::controller::user_controller;
use home_cloud::config::CONTEXT;
use home_cloud::middleware::actix_interceptor::ActixInterceptor;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 日志初始化
    home_cloud::config::logger::init_log();
    // 数据库连接池初始化
    CONTEXT.init_pool().await;
    HttpServer::new(|| {
        App::new()
            .wrap(ActixInterceptor {})
            // 登录登出接口单独处理（因为都不在已有的分组中）
            //.route("/backend/login", web::post().to(system_controller::login))
            //.route("/backend/logout", web::post().to(system_controller::logout))
            // 映射静态资源目录
            //.service(fs::Files::new("/warehouse", &CONTEXT.config.data_dir))
            .service(
                web::scope("/backend/system")
                    .service(user_controller::user_add)
                    .service(user_controller::user_update)
                    .service(user_controller::user_detail)
                    .service(user_controller::user_remove)
                    .service(user_controller::user_page)
            )
    })
        .bind(&CONTEXT.config.server_url)?
        .run()
        .await
}