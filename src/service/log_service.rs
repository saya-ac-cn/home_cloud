use actix_web::HttpRequest;
use log::error;
use crate::dao::log_mapper::LogMapper;
use crate::dao::log_type_mapper::LogTypeMapper;
use crate::entity::dto::log::LogPageDTO;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::vo::jwt::JWTToken;
use crate::entity::vo::log::LogVO;
use crate::entity::vo::log_type::LogTypeVO;
use crate::util::Page;
use crate::service::CONTEXT;
use crate::error::Error;
use crate::error::Result;
use crate::util;

/// 用户服务
pub struct LogService {}

impl LogService {

    /// 日志类别列表
    pub async fn query_log_type(&self) -> Result<Vec<LogTypeVO>> {
        let query_result = LogTypeMapper::select_all(&mut CONTEXT.primary_rbatis.as_executor()).await;
        if query_result.is_err() {
            error!("在查询日志类型列表时，发生异常:{}",query_result.unwrap_err());
            return Err(Error::from("查询日志类型列表异常"));
        }
        return Ok(query_result.unwrap().unwrap());
    }

    /// 日志分页
    pub async fn page(&self, req: &HttpRequest,param: &LogPageDTO) -> Result<Page<LogVO>>  {
        let mut extend = ExtendPageDTO{
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time:param.begin_time,
            end_time:param.end_time
        };
        let mut arg= param.clone();
        if arg.organize.is_none() || arg.organize.as_ref().unwrap().len() == 0 {
            let token = req.headers().get("access_token");
            let extract_result = &JWTToken::extract_token_by_header(token);
            if extract_result.is_err() {
                log::error!("在获取用户信息时，发生异常:{}",extract_result.clone().unwrap_err().to_string());
                return Err(crate::error::Error::from(String::from("获取用户信息失败")));
            }
            let user_info = extract_result.clone().unwrap();
            arg.organize = Some(vec![user_info.account])
        }


        let count_result = LogMapper::select_count(&mut CONTEXT.primary_rbatis.as_executor(), &arg,&extend).await;
        if count_result.is_err(){
            error!("在用户分页统计时，发生异常:{}",count_result.unwrap_err());
            return Err(Error::from("用户分页查询异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from(("未查询到符合条件的数据",util::NOT_EXIST)));
        }
        let mut result = Page::<LogVO>::page_query( total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no-1)*result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = LogMapper::select_page(&mut CONTEXT.primary_rbatis.as_executor(), &arg,&extend).await;
        if page_result.is_err() {
            error!("在日志分页获取页面数据时，发生异常:{}",page_result.unwrap_err());
            return Err(Error::from("日志分页查询异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

}