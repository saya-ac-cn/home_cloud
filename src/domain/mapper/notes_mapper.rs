use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use rbatis::{Error};
use crate::domain::table::Notes;
use crate::domain::dto::notes::NotesPageDTO;
use crate::domain::dto::page::ExtendPageDTO;
use crate::domain::vo::notes::NotesVO;

crud!(Notes {});

pub struct NotesMapper{}

impl NotesMapper {

    /// 修改笔记
    #[html_sql("./src/domain/mapper/notes_mapper.html")]
    pub async fn update_notes(rb: &mut dyn Executor,notes:&Notes,organize:&u64) -> rbatis::Result<ExecResult> { impled!() }

    /// 删除笔记
    #[html_sql("./src/domain/mapper/notes_mapper.html")]
    pub async fn delete_notes(rb: &mut dyn Executor,id:&u64,organize:&u64) -> rbatis::Result<ExecResult> { impled!() }

    /// 分页笔记动态
    #[html_sql("./src/domain/mapper/notes_mapper.html")]
    pub async fn select_page(rb: &mut dyn Executor,notes:&NotesPageDTO,extend:&ExtendPageDTO) -> Result<Vec<NotesVO>,Error> { impled!() }

    /// 查询笔记总数
    #[html_sql("./src/domain/mapper/notes_mapper.html")]
    pub async fn select_count(rb: &mut dyn Executor,notes:&NotesPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }
}