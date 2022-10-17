use home_cloud::controller::{content_controller, system_controller};
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
            .route("/backend/logout", web::post().to(system_controller::logout),)
            .service(fs::Files::new("/warehouse", "/Users/saya/warehouse"))
            .service(
                web::scope("/backend/system")
                    .service(system_controller::token_refresh)
                    .service(system_controller::myself)
                    .service(system_controller::user_add)
                    .service(system_controller::user_update)
                    .service(system_controller::user_detail)
                    .service(system_controller::user_remove)
                    .service(system_controller::user_page)
                    .service(system_controller::own_organize_user)
                    .service(system_controller::user_upload_logo)
                    .service(system_controller::user_update_password)
                    .service(system_controller::log_page)
                    .service(system_controller::log_excel)
                    .service(system_controller::log_type)
                    .service(system_controller::compute_pre6_logs)
                    .service(system_controller::compute_object_rows)
                    .service(system_controller::add_notes)
                    .service(system_controller::edit_plan)
                    .service(system_controller::delete_plan)
                    .service(system_controller::plan_page)
                    .service(system_controller::finish_plan)
                    .service(system_controller::plan_archive_page)
                    .service(system_controller::edit_archive_plan)
                    .service(system_controller::delete_archive_plan)
                    .service(system_controller::db_dump_log_page)
            )
            .service(
                web::scope("/backend/content")
                    .service(content_controller::add_news)
                    .service(content_controller::edit_news)
                    .service(content_controller::delete_news)
                    .service(content_controller::get_news)
                    .service(content_controller::page_news)
                    .service(content_controller::add_memo)
                    .service(content_controller::edit_memo)
                    .service(content_controller::delete_memo)
                    .service(content_controller::get_memo)
                    .service(content_controller::page_memo)
                    .service(content_controller::add_notebook)
                    .service(content_controller::edit_notebook)
                    .service(content_controller::delete_notebook)
                    .service(content_controller::notebook_list)
                    .service(content_controller::add_notes)
                    .service(content_controller::edit_notes)
                    .service(content_controller::delete_notes)
                    .service(content_controller::get_notes)
                    .service(content_controller::page_notes)
                    .service(content_controller::compute_pre6_news)
            )
    })
    .bind(&CONTEXT.config.server_url)?
    .run()
    .await
}
