use rbatis::{Error};
use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use crate::domain::table::LinkLabel;
crud!(LinkLabel {});

pub struct LinkLabelMapper {}

impl LinkLabelMapper{
    /// 查询指定文章的label
    #[html_sql("./src/domain/mapper/link_label_mapper.html")]
    pub async fn select_content(rb: &mut dyn Executor,category:&str,organize:u64,content_id:u64) -> Result<Vec<LinkLabel>,Error> { impled!() }
}