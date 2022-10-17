use rbatis::executor::Executor;
use rbatis::{Error};
use rbatis::rbdc::db::ExecResult;
use crate::entity::domain::business_database_tables::Files;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::vo::files::FilesVO;
use crate::entity::dto::files::{FilesDTO, FilesPageDTO};
crud!(Files {});
impl_select!(Files{select_by_id_and_organize(id:&u64,organize:&u64) => "`where id = #{id} and organize= #{organize}`"});

pub struct FilesMapper{}

impl FilesMapper {

    /// 查询单个文件
    #[html_sql("./src/dao/files_mapper.html")]
    pub async fn select_one(rb: &mut dyn Executor,files:&FilesDTO) -> Result<Option<FilesVO>,Error> { impled!() }

    /// 修改文件
    #[html_sql("./src/dao/files_mapper.html")]
    pub async fn update_files(rb: &mut dyn Executor,files:&Files) -> rbatis::Result<ExecResult> { impled!() }

    /// 分页查询文件
    #[html_sql("./src/dao/files_mapper.html")]
    pub async fn select_page(rb: &mut dyn Executor,files:&FilesPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<FilesVO>>,Error> { impled!() }

    /// 查询文件总数
    #[html_sql("./src/dao/files_mapper.html")]
    pub async fn select_count(rb: &mut dyn Executor,files:&FilesPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }
}