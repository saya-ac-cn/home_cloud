use crate::config::config::ApplicationConfig;
use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
use rbatis::rbatis::Rbatis;

pub mod user_mapper;
pub mod log_mapper;

pub struct DataSource {
    /// 数据库地址
    pub database_url: String,
    pub debug: bool,
    /// 逻辑删除字段
    pub logic_column: String,
    /// 未删除时的状态
    pub logic_un_deleted: i64,
    /// 删除后的状态
    pub logic_deleted: i64,
}


///实例化 rbatis orm 连接池
pub async fn init_rbatis(config: &DataSource) -> Rbatis {
    let mut rbatis = Rbatis::new();
    /// logic plugin 设置逻辑删除插件(不定义逻辑删除，即为真删除)
    /// rbatis.logic_plugin = Some(Box::new(RbatisLogicDeletePlugin::new_opt(
    ///    &config.logic_column,
    ///    config.logic_deleted as i32,
    ///    config.logic_un_deleted as i32,
    /// )));
    if config.debug.eq(&false) && rbatis.is_debug_mode() {
        panic!(
            r#"已使用release模式，但是rbatis仍使用debug模式！请删除 Cargo.toml 中 rbatis的配置 features = ["debug_mode"]"#
        );
    }
    /// 连接数据库
    println!("[home_cloud] rbatis link database ({})...", config.database_url);
    rbatis
        .link(&config.database_url)
        .await
        .expect("[home_cloud] rbatis link database fail!");
    println!("[home_cloud] rbatis link database success!");
    return rbatis;
}
