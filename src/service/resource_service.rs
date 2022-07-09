use rbatis::crud::{CRUD, CRUDMut};
use rbatis::DateTimeNative;

use crate::entity::domain::Resource;
use crate::entity::dto::ResourceDTO;
use crate::error::Error;
use crate::error::Result;
use crate::service::CONTEXT;

///后台文件资源服务
pub struct ResourceService {}

impl ResourceService {
    ///上传文件资源
    pub async fn add(&self, arg: &ResourceDTO) -> Result<u64> {
        if arg.suffix.is_none()
            || arg.suffix.as_ref().unwrap().is_empty()
            || arg.name.is_none()
            || arg.name.as_ref().unwrap().is_empty()
            || arg.size.is_none()
        {
            return Err(Error::from("文件名和文件类型不能为空!"));
        }
        let user = Resource {
            id: Option::None,
            name: arg.name.clone(),
            suffix: arg.suffix.clone(),
            size: arg.size.clone(),
            del: 0.into(),
            create_user: Option::from("saya".to_string()),
            create_time: DateTimeNative::now().into(),
        };
        let mut tx = CONTEXT.primary_rbatis.acquire_begin().await.unwrap();
        let rows_affected = tx.save(&user, &[]).await?.rows_affected;
        tx.commit().await.unwrap();
        return Ok(rows_affected);
    }
}
