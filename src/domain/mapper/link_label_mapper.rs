use rbatis::{Error};
use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use crate::domain::table::LinkLabel;
use crate::domain::vo::key_value::KeyValueVO;
crud!(LinkLabel {});

pub struct LinkLabelMapper {}

impl LinkLabelMapper{
    /// 查询指定文章的label
    #[html_sql("./src/domain/mapper/link_label_mapper.html")]
    pub async fn select_content(rb: &mut dyn Executor,category:&str,organize:u64,content_id:u64) -> Result<Vec<LinkLabel>,Error> { impled!() }

    /// 移除标签
    #[html_sql("./src/domain/mapper/link_label_mapper.html")]
    pub async fn delete_by_name(rb: &mut dyn Executor, content_id:u64, labels:&Vec<&u64>) -> rbatis::Result<ExecResult> { impled!() }

    /// 移除文章标签
    #[html_sql("./src/domain/mapper/link_label_mapper.html")]
    pub async fn delete_by_content(rb: &mut dyn Executor, category:&str,organize:u64,content_id:u64) -> rbatis::Result<ExecResult> { impled!() }

    /// 查询指定文章的label编号
    #[html_sql("./src/domain/mapper/link_label_mapper.html")]
    pub async fn select_link_by_content(rb: &mut dyn Executor,content:&u64) -> Result<String,Error> { impled!() }

    /// 查询指定集合文章的label编号
    #[html_sql("./src/domain/mapper/link_label_mapper.html")]
    pub async fn select_links_by_content(rb: &mut dyn Executor,content:&Vec<u64>) -> Result<Vec<KeyValueVO>,Error> { impled!() }

}