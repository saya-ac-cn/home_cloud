use rbatis::Rbatis;
use crate::service::ApplicationConfig;

///DDD分层架构，分为
///
/// * 领域层（domain）,该层存放数据库结构体模型（只在服务端流转，表结构）
pub mod domain;
/// * 数据传输层（dto，Data Transfer Object ）,存放接口传输的结构体（请求到服务端）
pub mod dto;
/// * 展示层（vo，View Object），存放展示的结构体（返回给无线端）
pub mod vo;

///实例化 rbatis orm 连接池
pub fn init_rbatis(config: &ApplicationConfig) -> Rbatis {
    let rbatis = Rbatis::new();
    if rbatis.is_debug_mode() == false && config.debug.eq(&true) {
        panic!(
            r#"已使用release模式运行，但是仍使用debug模式！请修改 application.yml 中debug配置项为  debug: false"#
        );
    }
    return rbatis;
}
