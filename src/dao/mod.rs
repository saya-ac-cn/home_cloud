pub mod abstracts_mapper;
pub mod db_dump_log_mapper;
pub mod files_mapper;
pub mod general_journal_mapper;
pub mod journal_mapper;
pub mod log_mapper;
pub mod log_type_mapper;
pub mod memo_mapper;
pub mod monetary_mapper;
pub mod news_mapper;
pub mod notebook_mapper;
pub mod notes_mapper;
pub mod payment_means_mapper;
pub mod pictures_mapper;
pub mod plan_archive_mapper;
pub mod plan_mapper;
pub mod user_mapper;

use crate::conf::ApplicationConfig;
use rbatis::RBatis;
pub fn init_rbatis(config: &ApplicationConfig) -> RBatis {
    let rbatis = RBatis::new();
    if rbatis.is_debug_mode() == false && config.debug.eq(&true) {
        panic!(
            r#"已使用release模式运行，但是仍使用debug模式！请修改 application.yml 中debug配置项为  debug: false"#
        );
    }
    return rbatis;
}
