use rbatis::rbatis::RBatis;
use lazy_static::lazy_static;
use tokio::sync::Mutex;
use crate::config::redis_client::RedisClient;
use crate::config::config::ApplicationConfig;
use crate::service::content_service::ContentService;
use crate::service::financial_service::FinancialService;
use crate::service::oss_service::OssService;
use crate::service::system_service::SystemService;
use delay_timer::prelude::{DelayTimer, DelayTimerBuilder};

// 第一种初始化方法
// /// CONTEXT is all of the service struct
// pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

// 在lazy_static! { //your code} 中的代码并不会在编译时初始化静态量，它会在首次调用时，执行代码，来初始化。也就是所谓的延迟计算。
lazy_static! {
    // CONTEXT is all of the service struct
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
    // SCHEDULER is only SCHEDULER VARIABLE
    pub static ref SCHEDULER: Mutex<DelayTimer> = Mutex::new(DelayTimerBuilder::default().build());
}

#[macro_export]
macro_rules! primary_rbatis_pool {
    () => {
        &mut $crate::config::CONTEXT.primary_rbatis.clone()
    };
}

#[macro_export]
macro_rules! business_rbatis_pool {
    () => {
        &mut $crate::config::CONTEXT.business_rbatis.clone()
    };
}

#[macro_export]
macro_rules! financial_rbatis_pool {
    () => {
        &mut $crate::config::CONTEXT.financial_rbatis.clone()
    };
}

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub primary_rbatis: RBatis,
    pub business_rbatis: RBatis,
    pub financial_rbatis: RBatis,
    pub system_service: SystemService,
    pub oss_service: OssService,
    pub content_service: ContentService,
    pub financial_service: FinancialService,
    pub redis_client: RedisClient,
}

impl ServiceContext {
    /// init database pool
    pub async fn init_pool(&self) {
        // futures::executor::block_on(async {
        //     self.init_datasource(&self.primary_rbatis,&self.config.primary_database_url,"primary_pool").await
        // });
        self.init_datasource(
            &self.primary_rbatis,
            &self.config.primary_database_url,
            "primary_pool",
        )
            .await;
        self.init_datasource(
            &self.business_rbatis,
            &self.config.business_database_url,
            "business_pool",
        )
            .await;
        self.init_datasource(
            &self.financial_rbatis,
            &self.config.financial_database_url,
            "financial_pool",
        )
            .await;
        log::info!(
            " - Local:   http://{}",
            self.config.server_url.replace("0.0.0.0", "127.0.0.1")
        );
    }

    pub async fn init_datasource(&self, rbatis: &RBatis, url: &str, name: &str) {
        log::info!("[home_cloud] rbatis {} init ({})...", name, url);
        let driver = rbdc_mysql::driver::MysqlDriver {};
        let driver_name = format!("{:?}", driver);
        rbatis
            .init(driver, url)
            .expect(&format!("[home_cloud] rbatis {} init fail!", name));
        rbatis.acquire().await.expect(&format!(
            "rbatis connect database(driver={},url={}) fail",
            driver_name, url
        ));
        log::info!(
            "[home_cloud] rbatis {} init success! pool state = {:?}",
            name,
            rbatis.get_pool().expect("pool not init!").status()
        );
    }
}

impl Default for ServiceContext {
    /// 初始化操作，由全局的静态方法触发
    fn default() -> Self {
        let config = ApplicationConfig::default();
        ServiceContext {
            primary_rbatis: crate::dao::init_rbatis(&config),
            business_rbatis: crate::dao::init_rbatis(&config),
            financial_rbatis: crate::dao::init_rbatis(&config),
            system_service: SystemService {},
            oss_service: OssService {},
            content_service: ContentService {},
            financial_service: FinancialService {},
            redis_client: RedisClient::new(&config.redis_url),
            config,
        }
    }
}
