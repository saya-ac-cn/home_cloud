use home_cloud::controller::{system_controller};
use home_cloud::service::{CONTEXT};
use actix_web::{web, App, HttpServer};
use actix_files as fs;
use log::info;
use home_cloud::util::scheduler::Scheduler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化数据库连接池
    CONTEXT.init_pool();
    // 初始化调度器
    actix_web::rt::spawn(Scheduler::start());
    //日志追加器
    home_cloud::config::log::init_log();
    info!(
        " - Local:   http://{}",
        CONTEXT.config.server_url.replace("0.0.0.0", "127.0.0.1")
    );
    //路由
    HttpServer::new(|| {
        App::new()
            .wrap(home_cloud::middleware::auth::Auth)
            .route("/backend/login", web::post().to(system_controller::login),)
    })
    .bind(&CONTEXT.config.server_url)?
    .run()
    .await
}
