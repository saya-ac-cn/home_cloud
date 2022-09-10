/// 服务层
///
/// 系统用户服务
mod system_service;
/// 文件资源服务
mod oss_service;
/// 文本（消息）服务
mod content_service;
/// 财政金融服务
mod financial_service;

use std::collections::HashMap;
use rbatis::rbatis::Rbatis;
pub use system_service::*;
pub use oss_service::*;
pub use content_service::*;
use crate::dao::DataSource;
pub use crate::config::config::ApplicationConfig;
use crate::service::financial_service::FinancialService;
use crate::util::scheduler::Scheduler;
use chrono::Utc;
use std::sync::Mutex;


pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub primary_rbatis: Rbatis,
    pub business_rbatis: Rbatis,
    pub financial_rbatis: Rbatis,
    pub system_service: SystemService,
    pub oss_service: OssService,
    pub content_service: ContentService,
    pub financial_service: FinancialService,
    //pub scheduler:Scheduler
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        // 主数据源配置
        let primary_database_config = DataSource {
            database_url: config.primary_database_url.clone(),
            debug: config.debug,
            /// 逻辑删除字段
            logic_column: config.logic_column.clone(),
            logic_un_deleted: config.logic_un_deleted,
            logic_deleted: config.logic_deleted,
        };

        // 业务数据源配置
        let business_database_config = DataSource {
            database_url: config.business_database_url.clone(),
            debug: config.debug,
            logic_column: config.logic_column.clone(),
            logic_un_deleted: config.logic_un_deleted,
            logic_deleted: config.logic_deleted,
        };

        // 财政数据源配置
        let financial_database_config = DataSource {
            database_url: config.financial_database_url.clone(),
            debug: config.debug,
            logic_column: config.logic_column.clone(),
            logic_un_deleted: config.logic_un_deleted,
            logic_deleted: config.logic_deleted,
        };
        let mut scheduler:Scheduler = Scheduler{
            scheduler:cron_tab::Cron::new(Utc),
            plan_pool: HashMap::new()
        };
        //actix_web::rt::spawn(scheduler.start());
        //scheduler.add(5210);
        ServiceContext {
            primary_rbatis: async_std::task::block_on(async {
                crate::dao::init_rbatis(&primary_database_config).await
            }),
            business_rbatis: async_std::task::block_on(async {
                crate::dao::init_rbatis(&business_database_config).await
            }),
            financial_rbatis: async_std::task::block_on(async {
                crate::dao::init_rbatis(&financial_database_config).await
            }),
            system_service: SystemService {},
            oss_service: OssService {},
            content_service: ContentService {},
            financial_service: FinancialService {},
            config,
            //scheduler
        }
    }
}

    // 在lazy_static! { //your code} 中的代码并不会在编译时初始化静态量，它会在首次调用时，执行代码，来初始化。也就是所谓的延迟计算。
    lazy_static! {
        pub static ref CONTEXT: ServiceContext = ServiceContext::default();
        // https://www.javaroad.cn/questions/71579
        pub static ref SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler{
            scheduler:cron_tab::Cron::new(Utc),
            plan_pool: HashMap::new()
        });
    }
