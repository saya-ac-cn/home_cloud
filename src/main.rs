use home_cloud::controller::{system_controller, file_controller, content_controller};
use home_cloud::service::CONTEXT;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use log::info;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
            .route("/login", web::post().to(system_controller::login),)
            .route("/logout", web::post().to(system_controller::logout),)
            .service(fs::Files::new("/warehouse", "/Users/saya/warehouse"))
            .service(
                web::scope("/backend/system")
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
                    .service(system_controller::log_type)
            )
            .service(
                web::scope("/backend/oss")
                    .service(file_controller::upload_base64_picture)
                    .service(file_controller::upload_file_picture)
                    .service(file_controller::page_picture)
                    .service(file_controller::picture_delete)
                    .service(file_controller::upload_file)
            )
            .service(
                web::scope("/backend/content")
                    .service(content_controller::add_news)
                    .service(content_controller::edit_news)
                    .service(content_controller::delete_news)
                    .service(content_controller::get_news)
                    .service(content_controller::page_news)
            )
    })
    .bind(&CONTEXT.config.server_url)?
    .run()
    .await
}
