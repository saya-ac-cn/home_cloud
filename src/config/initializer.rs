use lazy_static::lazy_static;
use rbatis::rbatis::RBatis;
use crate::config::redis_client::RedisClient;
use crate::service::user_service::UserService;
use crate::config::ApplicationConfig;


// 第一种初始化方法
// /// CONTEXT is all of the service struct
// pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

// 在lazy_static! { //your code} 中的代码并不会在编译时初始化静态量，它会在首次调用时，执行代码，来初始化。也就是所谓的延迟计算。
lazy_static! {
    // CONTEXT is all of the service struct
    pub static ref CONTEXT: ServiceContext = ServiceContext::default();
}

// 为方便使用，直接定义成宏
#[macro_export]
macro_rules! primary_rbatis_pool {
   () => {
       &mut $crate::config::CONTEXT.primary_rbatis.clone()
   };
}

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub redis_client: RedisClient,
    pub primary_rbatis: RBatis,
    pub user_service: UserService
}

impl ServiceContext {
    /// init database pool
    pub async fn init_pool(&self) {
        // futures::executor::block_on(async {
        //     self.init_datasource(&self.primary_rbatis,&self.config.primary_database_url,"primary_pool").await
        // });
        self.init_datasource(&self.primary_rbatis,&self.config.primary_database_url,"primary_pool").await;
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
            redis_client: RedisClient::new(&config.redis_url),
            user_service: UserService{},
            config,
        }
    }
}