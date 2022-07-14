/// 定义的业务相关处理回执码
/// 处理成功
pub const CODE_SUCCESS: i32 = 0;
/// 处理失败（通用）
pub const CODE_FAIL: i32 = -1;
/// 记录不存在
pub const NOT_EXIST: i32 = -3;
/// 未登录
pub const NOT_CHECKING: i32 = -4;
/// 缺少参数
pub const NOT_PARAMETER: i32 = -5;
/// 文件类型错误
pub const FILE_IO_ERROR: i32 = -6;

/// 定义数据目录下的子级目录
/// 数据库目录
pub const DATABASE_PATH:&str = "database";
/// 文档目录
pub const DOCUMENT_PATH:&str = "document/file";
/// logo目录
pub const LOGO_PATH:&str = "picture/logo";
/// 插图目录
pub const ILLUSTRATED_PATH:&str = "picture/illustrated";
/// 墙纸&背景目录
pub const WALLPAPER_PATH:&str = "picture/wallpaper";

/// 定义日期相关的格式化format
pub const FORMAT_Y_M_D_H_M_S:&str = "%Y-%m-%d %H:%M:%S";
pub const FORMAT_Y_M_D:&str = "%Y-%m-%d";
pub const FORMAT_YMD:&str = "%Y%m%d";