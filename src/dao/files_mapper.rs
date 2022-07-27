use rbatis::executor::{RbatisExecutor};
use rbatis::db::DBExecResult;
use rbatis::{Error};
use crate::entity::domain::business_database_tables::Files;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::vo::files::FilesVO;
use crate::entity::dto::files::{FilesDTO, FilesPageDTO};

pub struct FilesMapper{}

impl FilesMapper {

    /// 查询单个文件
    #[html_sql("./src/dao/files_mapper.html")]
    pub async fn select_one(rb: &mut RbatisExecutor<'_,'_>,files:&FilesDTO) -> Result<Option<FilesVO>,Error> { impled!() }

    /// 修改文件
    #[html_sql("./src/dao/files_mapper.html")]
    pub async fn update_files(rb: &mut RbatisExecutor<'_,'_>,files:&Files) -> rbatis::core::Result<DBExecResult> { impled!() }

    /// 分页查询文件
    #[html_sql("./src/dao/files_mapper.html")]
    pub async fn select_page(rb: &mut RbatisExecutor<'_,'_>,files:&FilesPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<FilesVO>>,Error> { impled!() }

    /// 查询文件总数
    #[html_sql("./src/dao/files_mapper.html")]
    pub async fn select_count(rb: &mut RbatisExecutor<'_,'_>,files:&FilesPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }
}