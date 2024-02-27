use std::collections::HashMap;
use config::{Config, File};

/// Config
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    pub debug: bool,
    /// 当前服务地址
    pub server_url: String,
    /// 主数据库地址
    pub primary_database_url: String,
    /// 辅助业务数据库地址
    pub business_database_url: String,
    /// 财政相关数据库地址
    pub financial_database_url: String,
    /// redis地址
    pub redis_url: String,
    /// 日志目录 "target/logs/"
    pub log_dir: String,
    /// "100MB" 日志分割尺寸-单位KB,MB,GB
    pub log_temp_size: String,
    /// 日志打包格式可选“”（空-不压缩）“gzip”（gz压缩包）“zip”（zip压缩包）“lz4”（lz4压缩包（非常快））
    pub log_pack_compress: String,
    /// 日志滚动配置   保留全部:All,按时间保留:KeepTime(Duration),按版本保留:KeepNum(i64)
    pub log_rolling_type: String,
    /// 日志等级
    pub log_level: String,
    pub log_type: String,
    pub log_chan_len: Option<usize>,
    /// 白名单接口
    pub white_list_api: Vec<String>,
    /// 收件人
    pub to_mail: String,
    /// 高德地图ip定位地址
    pub amap_url: String,
    /// 高德地图ip定位密钥
    pub amap_key: String,
    /// 项目产生的数据目录
    pub data_dir: String,
    /// 数据库空闲目录
    pub mysql_dump: String,
    /// 文件类型映射字典
    pub file_type_map: HashMap<String, String>,
    /// 发送微信消息的接口
    pub wechat_api: String,
    /// 发送微信消息的模板(提醒)
    pub wechat_notice_template: String,
    /// 发送邮件的接口
    pub mail_api: String,
    /// 发送邮件的模板(提醒)
    pub mail_notice_template: String,
    /// 发送邮件的模板(备份)
    pub mail_dump_template: String,
}


impl Default for ApplicationConfig {
    fn default() -> Self {
        let mut config = Config::default();
        config.merge(File::with_name("application.yml")).unwrap();
        let result: ApplicationConfig = config.try_into().unwrap();
        if result.debug {
            println!("[home_cloud] load conf:{:?}", result);
            println!("[home_cloud] ///////////////////// Start On Debug Mode ////////////////////////////");
        } else {
            println!("[home_cloud] ///////////////////// Start On Release Mode ////////////////////////////");
        }
        result
    }
}
