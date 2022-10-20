use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use rbatis::{Error};
use crate::entity::domain::business_database_tables::NoteBook;
use crate::entity::dto::notebook::NoteBookDTO;
use crate::entity::vo::notebook::NoteBookVO;

pub struct NoteBookMapper{}

impl NoteBookMapper {

    /// 修改笔记簿
    #[html_sql("./src/dao/notebook_mapper.html")]
    pub async fn update_notebook(rb: &mut dyn Executor,notebook:&NoteBook) -> rbatis::Result<ExecResult> { impled!() }

    /// 查询所有的笔记簿
    #[html_sql("./src/dao/notebook_mapper.html")]
    pub async fn select_list(rb: &mut dyn Executor,notebook:&NoteBookDTO) -> Result<Option<Vec<NoteBookVO>>,Error> { impled!() }
}