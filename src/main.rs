use home_cloud::controller::{system_controller, oss_controller, content_controller, financial_controller};
use home_cloud::service::CONTEXT;
use actix_web::{web, App, HttpServer};
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
                    .service(oss_controller::upload_base64_picture)
                    .service(oss_controller::upload_file_picture)
                    .service(oss_controller::page_picture)
                    .service(oss_controller::picture_delete)
                    .service(oss_controller::upload_file)
                    .service(oss_controller::page_files)
                    .service(oss_controller::files_download)
                    .service(oss_controller::files_edit)
                    .service(oss_controller::files_delete)
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
            )
            .service(
                web::scope("/backend/financial")
                    .service(financial_controller::add_journal)
                    .service(financial_controller::edit_journal)
                    .service(financial_controller::delete_journal)
                    .service(financial_controller::page_journal)
                    .service(financial_controller::excel_journal)
                    .service(financial_controller::add_general_journal)
                    .service(financial_controller::edit_general_journal)
                    .service(financial_controller::delete_general_journal)
                    .service(financial_controller::detail_general_journal)
                    .service(financial_controller::excel_general_journal)
                    .service(financial_controller::get_monetary_list)
                    .service(financial_controller::get_abstracts_list)
                    .service(financial_controller::get_payment_means_list)
                    .service(financial_controller::page_journal_collect)
                    .service(financial_controller::excel_journal_collect)
                    .service(financial_controller::compute_account_growth_rate)
                    .service(financial_controller::compute_income_percentage)
                    .service(financial_controller::order_month_journal)
                    .service(financial_controller::compute_pre6_journal)
            )
    })
    .bind(&CONTEXT.config.server_url)?
    .run()
    .await
}
