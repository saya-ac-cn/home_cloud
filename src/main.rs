use actix_files as fs;
use actix_web::{web, App, HttpServer};
use home_cloud::controller::{
    content_controller, financial_controller, oss_controller, system_controller,
};
use home_cloud::middleware::actix_interceptor::ActixInterceptor;
use home_cloud::conf::CONTEXT;
use home_cloud::conf::scheduler::Scheduler;

/// use tokio,because Rbatis specifies the runtime-tokio
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // log
    home_cloud::conf::logger::init_log();
    // database
    CONTEXT.init_pool().await;
    // scheduler
    //actix_web::rt::spawn(Scheduler::start()).await;
    Scheduler::init_system_scheduler().await;
    // router
    HttpServer::new(|| {
        App::new()
            .wrap(ActixInterceptor {})
            // 登录登出接口单独处理（因为都不在已有的分组中）
            .route("/backend/login", web::post().to(system_controller::login))
            .route("/backend/logout", web::post().to(system_controller::logout))
            // 映射静态资源目录
            .service(fs::Files::new("/warehouse", &CONTEXT.config.data_dir))
            .service(
                web::scope("/backend/system")
                    .service(system_controller::token)
                    .service(system_controller::myself)
                    .service(system_controller::user_add)
                    .service(system_controller::user_update)
                    .service(system_controller::user_detail)
                    .service(system_controller::user_remove)
                    .service(system_controller::user_page)
                    .service(system_controller::own_organize_user)
                    .service(system_controller::user_upload_logo)
                    .service(system_controller::check_password)
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
                    .service(system_controller::db_dump_log_page),
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
                    .service(content_controller::compute_pre6_news),
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
                    .service(financial_controller::compute_pre6_journal),
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
                    .service(oss_controller::files_delete),
            )
            .service(
                web::scope("/frontend")
                    .service(content_controller::public_page_news)
                    .service(content_controller::public_page_notes)
                    .service(oss_controller::public_page_files)
                    .service(oss_controller::files_download)
                    .service(content_controller::public_notebook_list)
                    .service(content_controller::public_news_detail)
                    .service(content_controller::public_notes_detail)
                    .service(system_controller::plan_grid),
            )
    })
    .bind(&CONTEXT.config.server_url)?
    .run()
    .await
}
