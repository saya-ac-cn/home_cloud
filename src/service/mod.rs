/// 服务层
///
/// 系统用户服务
mod user_service;
/// 日子相关服务
mod log_service;
/// 文件资源服务
mod file_service;

use rbatis::rbatis::Rbatis;
pub use crate::config::config::ApplicationConfig;
pub use user_service::*;
pub use log_service::*;
pub use file_service::*;
use crate::dao::DataSource;

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub primary_rbatis: Rbatis,
    pub financial_rbatis: Rbatis,
    pub user_service: UserService,
    pub log_service: LogService,
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        // 主数据源配置
        let primary_database_config = DataSource{
            database_url: config.primary_database_url.clone(),
            debug: config.debug,
            /// 逻辑删除字段
            logic_column: config.logic_column.clone(),
            logic_un_deleted: config.logic_un_deleted,
            logic_deleted: config.logic_deleted,
        };

        // 财政数据源配置
        let financial_database_config = DataSource{
            database_url: config.financial_database_url.clone(),
            debug: config.debug,
            logic_column: config.logic_column.clone(),
            logic_un_deleted: config.logic_un_deleted,
            logic_deleted: config.logic_deleted,
        };

        ServiceContext {
            primary_rbatis: async_std::task::block_on(async {
                crate::dao::init_rbatis(&primary_database_config).await
            }),
            financial_rbatis: async_std::task::block_on(async {
                crate::dao::init_rbatis(&financial_database_config).await
            }),
            user_service: UserService {},
            log_service: LogService {},
            config,
        }
    }
}

// 在lazy_static! { //your code} 中的代码并不会在编译时初始化静态量，它会在首次调用时，执行代码，来初始化。也就是所谓的延迟计算。
lazy_static! {
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}
