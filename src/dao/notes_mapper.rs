use rbatis::executor::{RbatisExecutor};
use rbatis::db::DBExecResult;
use rbatis::{ Error};
use crate::entity::domain::business_database_tables::Notes;
use crate::entity::dto::notes::NotesPageDTO;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::vo::notes::NotesVO;

pub struct NotesMapper{}

impl NotesMapper {

    /// 修改笔记
    #[html_sql("./src/dao/notes_mapper.html")]
    pub async fn update_notes(rb: &mut RbatisExecutor<'_,'_>,notes:&Notes,organize:&u64) -> rbatis::core::Result<DBExecResult> { impled!() }

    /// 删除笔记
    #[html_sql("./src/dao/notes_mapper.html")]
    pub async fn delete_notes(rb: &mut RbatisExecutor<'_,'_>,id:&u64,organize:&u64) -> rbatis::core::Result<DBExecResult> { impled!() }

    /// 分页笔记动态
    #[html_sql("./src/dao/notes_mapper.html")]
    pub async fn select_page(rb: &mut RbatisExecutor<'_,'_>,notes:&NotesPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<NotesVO>>,Error> { impled!() }

    /// 查询笔记总数
    #[html_sql("./src/dao/notes_mapper.html")]
    pub async fn select_count(rb: &mut RbatisExecutor<'_,'_>,notes:&NotesPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }
}