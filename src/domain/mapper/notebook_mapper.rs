use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use rbatis::{Error};
use crate::domain::table::NoteBook;
use crate::domain::dto::notebook::NoteBookDTO;
use crate::domain::vo::notebook::NoteBookVO;

crud!(NoteBook {});
impl_select!(NoteBook {select_by_organize_name(name:&String,organize:&u64) => "`where name = #{name} and organize= #{organize}`"});
impl_select!(NoteBook {select_for_repeat(id:&u64,name:&String,organize:&u64) => "`where id != #{id} and name = #{name} and organize= #{organize}`"});
impl_delete!(NoteBook {delete_by_id_organize(id:&u64,organize:&u64) => "`where id = #{id} and organize= #{organize}`"});


pub struct NoteBookMapper{}

impl NoteBookMapper {

    /// 修改笔记簿
    #[html_sql("./src/domain/mapper/notebook_mapper.html")]
    pub async fn update_notebook(rb: &mut dyn Executor,notebook:&NoteBook) -> rbatis::Result<ExecResult> { impled!() }

    /// 查询所有的笔记簿
    #[html_sql("./src/domain/mapper/notebook_mapper.html")]
    pub async fn select_list(rb: &mut dyn Executor,notebook:&NoteBookDTO) -> Result<Option<Vec<NoteBookVO>>,Error> { impled!() }
}