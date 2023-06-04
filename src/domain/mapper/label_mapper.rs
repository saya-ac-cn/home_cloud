use rbatis::{Error};
use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use crate::domain::table::Label;
crud!(Label {});
impl_select!(Label {select_by_category_organize(category:&str,organize:u64) => "`where category = #{category} and organize= #{organize}`"});

pub struct LabelMapper {}

impl LabelMapper {
    /// 添加标签
    #[html_sql("./src/domain/mapper/label_mapper.html")]
    pub async fn insert_label(rb: &mut dyn Executor,labels:&Vec<Label>) -> rbatis::Result<ExecResult> { impled!() }

    /// 查询指定name下的id
    #[html_sql("./src/domain/mapper/label_mapper.html")]
    pub async fn select_id(rb: &mut dyn Executor,category:&str,organize:u64,names:&Vec<&String>) -> Result<Vec<Label>,Error> { impled!() }
}