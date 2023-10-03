/// Config
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    pub debug: bool,
    /// 当前服务地址
    pub server_url: String,
    /// 主数据库地址
    pub primary_database_url: String,
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
    pub white_list_api: Vec<String>
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        let yml_data = include_str!("../../application.yml");
        //load config
        let result: ApplicationConfig =
            serde_yaml::from_str(yml_data).expect("load config file fail");
        if result.debug {
            println!("[home_cloud] load config:{:?}", result);
            println!("[home_cloud] ///////////////////// Start On Debug Mode ////////////////////////////");
        } else {
            println!("[home_cloud] ///////////////////// Start On Release Mode ////////////////////////////");
        }
        result
    }
}
