use home_cloud::controller::{user_controller, log_controller, file_controller};
use home_cloud::service::CONTEXT;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use log::info;
///  命名规范
/// 1、蛇形命名法（Snake Case）
/// 文件名：例如：hello_world.rs、main_xxx.rs
/// 变量名：例如：zhangsan_name
/// 函数名：例如：func_name()
///
/// 2、大驼峰命名法（Camel Case）
/// 结构体：例如：struct ExampleStruct { name: String}
/// enum类型：例如：enum IpAddress {IPV4(u8,u8,u8,u8)}
///
/// 3、其它
/// 关联常量：全部大写，例如：NAME、AGE
/// 连接符：Cargo默认把连接符“-”转换成下划线“_”
/// 语句：跟C，Java语言等一样，每行语句结束都要添加;
/// PS：Rust也不建议以“-rs”或“_rs”为后缀来命名包名，如果以此来命名，会强制性的将此后缀去掉。

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
            .route("/login", web::post().to(user_controller::login),)
            .route("/logout", web::post().to(user_controller::logout),)
            .service(fs::Files::new("/warehouse", "/Users/saya/warehouse"))
            .service(
                web::scope("/backend/user")
                    .service(user_controller::myself)
                    .service(user_controller::add)
                    .service(user_controller::update)
                    .service(user_controller::detail)
                    .service(user_controller::remove)
                    .service(user_controller::page)
                    .service(user_controller::own_organize_user)
                    .service(user_controller::upload_logo)
                    .service(user_controller::update_password)
            )
            .service(
                web::scope("/backend/log")
                    .service(log_controller::page)
                    .service(log_controller::query_log_type)
            )
            .service(
                web::scope("/backend/oss")
                    .service(file_controller::upload_base64_picture)
            )
    })
    .bind(&CONTEXT.config.server_url)?
    .run()
    .await
}
