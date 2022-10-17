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

use rbatis::rbatis::Rbatis;
pub use system_service::*;
pub use oss_service::*;
pub use content_service::*;
pub use financial_service::*;

pub use crate::config::config::ApplicationConfig;
use std::sync::Mutex;
use lazy_static::lazy_static;
use rbdc_mysql::driver::MysqlDriver;
use crate::util::scheduler::Scheduler;

// 第一种初始化方法
// /// CONTEXT is all of the service struct
// pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());


// 在lazy_static! { //your code} 中的代码并不会在编译时初始化静态量，它会在首次调用时，执行代码，来初始化。也就是所谓的延迟计算。
lazy_static! {
    // CONTEXT is all of the service struct
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
    // SCHEDULER is only SCHEDULER VARIABLE
    pub static ref SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::default());
}


#[macro_export]
macro_rules! primary_rbatis_pool {
    () => {
        &mut $crate::service::CONTEXT.primary_rbatis.clone()
    };
}

#[macro_export]
macro_rules! business_rbatis_pool {
    () => {
        &mut $crate::service::CONTEXT.business_rbatis.clone()
    };
}

#[macro_export]
macro_rules! financial_rbatis_pool {
    () => {
        &mut $crate::service::CONTEXT.financial_rbatis.clone()
    };
}

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub primary_rbatis: Rbatis,
    pub business_rbatis: Rbatis,
    pub financial_rbatis: Rbatis,
    pub system_service: SystemService,
    pub oss_service: OssService,
    pub content_service: ContentService,
    pub financial_service: FinancialService
}

impl ServiceContext {
    /// init database pool
    pub fn init_pool(&self) {
        async_std::task::block_on(async {
            self.init_datasource(&self.primary_rbatis,&self.config.primary_database_url).await
        });
        async_std::task::block_on(async {
            self.init_datasource(&self.business_rbatis,&self.config.business_database_url).await;
        });
        async_std::task::block_on(async {
            self.init_datasource(&self.financial_rbatis,&self.config.financial_database_url).await;
        });
    }

    pub async fn init_datasource(&self,rbatis:&Rbatis,url:&str){
        //连接数据库
        println!("[home_cloud] rbatis pool init ({})...",url);
        rbatis.init(MysqlDriver {}, url).expect("[home_cloud] rbatis pool init fail!");
        log::info!("[home_cloud] rbatis pool init success! pool state = {:?}",rbatis.get_pool().expect("pool not init!").status());
    }

}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        ServiceContext {
            primary_rbatis: crate::entity::init_rbatis(&config),
            business_rbatis: crate::entity::init_rbatis(&config),
            financial_rbatis: crate::entity::init_rbatis(&config),
            system_service: SystemService {},
            oss_service: OssService {},
            content_service: ContentService {},
            financial_service: FinancialService {},
            config,
        }
    }
}
