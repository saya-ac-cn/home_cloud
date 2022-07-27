use actix_web::HttpRequest;
use log::error;
use rbatis::crud::CRUD;
use crate::dao::log_mapper::LogMapper;
use crate::dao::news_mapper::NewsMapper;
use crate::entity::domain::business_database_tables::News;
use crate::entity::dto::news::{NewsDTO, NewsPageDTO};
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::vo::jwt::JWTToken;
use crate::entity::vo::news::NewsVO;
use crate::util::Page;
use crate::service::CONTEXT;
use crate::error::Error;
use crate::error::Result;
use crate::util;

/// 文本（消息）服务
pub struct ContentService {}

impl ContentService {

    /// 发布消息动态
    pub async fn add_news(&self, req: &HttpRequest,arg: &NewsDTO) -> Result<u64> {
        let check_flag = arg.topic.is_none() || arg.topic.as_ref().unwrap().is_empty() || arg.content.is_none() || arg.content.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("动态标题和内容不能为空!",util::NOT_PARAMETER)));
        }
        let token = req.headers().get("access_token");
        let extract_result = &JWTToken::extract_token_by_header(token);
        if extract_result.is_err() {
            error!("在获取用户信息时，发生异常:{}",extract_result.clone().unwrap_err().to_string());
            return Err(crate::error::Error::from(String::from("获取用户信息失败")));
        }
        let user_info = extract_result.clone().unwrap();
        let news = News{
            id:None,
            topic:arg.topic.clone(),
            label:arg.label.clone(),
            content:arg.content.clone(),
            organize: Some(user_info.organize),
            source:Some(user_info.account.clone()),
            create_time:Some(rbatis::DateTimeNative::now()),
            update_time:None,
        };
        let write_result = CONTEXT.business_rbatis.save(&news, &[]).await;
        if  write_result.is_err(){
            error!("保存消息动态时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("发布消息动态失败!"));
        }
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&extract_result.clone().unwrap(),String::from("OX008")).await;
        return Ok(write_result?.rows_affected);
    }

    /// 修改消息动态
    pub async fn edit_news(&self, req: &HttpRequest,arg: &NewsDTO) -> Result<u64> {
        let check_flag = arg.id.is_none() || arg.topic.is_none() || arg.topic.as_ref().unwrap().is_empty() || arg.content.is_none() || arg.content.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("动态标题和内容不能为空!",util::NOT_PARAMETER)));
        }
        let token = req.headers().get("access_token");
        let extract_result = &JWTToken::extract_token_by_header(token);
        if extract_result.is_err() {
            log::error!("在获取用户信息时，发生异常:{}",extract_result.clone().unwrap_err().to_string());
            return Err(crate::error::Error::from(String::from("获取用户信息失败")));
        }
        let user_info = extract_result.clone().unwrap();
        let query_where = CONTEXT.business_rbatis.new_wrapper().eq(News::id(), &arg.id).and().eq(News::organize(),user_info.organize);
        let news_option: Option<News> = CONTEXT.business_rbatis.fetch_by_wrapper(query_where).await?;
        let news_exist = news_option.ok_or_else(|| Error::from((format!("id={} 的动态不存在!", &arg.id.clone().unwrap()),util::NOT_EXIST)))?;
        let news = News{
            id:arg.id,
            topic: arg.topic.clone(),
            label: arg.label.clone(),
            content: arg.content.clone(),
            organize: news_exist.organize,
            source: Some(user_info.account.clone()),
            create_time: None,
            update_time: None
        };
        let result = NewsMapper::update_news(&mut CONTEXT.business_rbatis.as_executor(),&news).await;
        if result.is_err() {
            error!("在修改id={}的动态时，发生异常:{}",arg.id.as_ref().unwrap(),result.unwrap_err());
            return Err(Error::from("动态修改失败"));
        }
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX009")).await;
        return Ok(result?.rows_affected);
    }

    /// 删除消息动态
    pub async fn delete_news(&self, req: &HttpRequest,id: &u64) -> Result<u64> {
        let token = req.headers().get("access_token");
        let extract_result = &JWTToken::extract_token_by_header(token);
        if extract_result.is_err() {
            log::error!("在获取用户信息时，发生异常:{}",extract_result.clone().unwrap_err().to_string());
            return Err(crate::error::Error::from(String::from("获取用户信息失败")));
        }
        let user_info = extract_result.clone().unwrap();
        // 只能删除自己组织机构下的数据
        let delete_where = CONTEXT.business_rbatis.new_wrapper().eq(News::id(),id).and().eq(News::organize(),user_info.organize);
        let write_result = CONTEXT.business_rbatis.remove_by_wrapper::<News>(delete_where).await;
        if write_result.is_err(){
            error!("删除消息动态时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("删除消息动态失败!"));
        }
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX010")).await;
        return Ok(write_result?);
    }

    /// 获取消息动态
    pub async fn get_news_detail(&self,id: &u64) -> Result<NewsVO> {
        let news_option: Option<News> = CONTEXT.business_rbatis.fetch_by_wrapper(CONTEXT.business_rbatis.new_wrapper().eq(News::id(), id)).await?;
        let news_exist = news_option.ok_or_else(|| Error::from((format!("id={} 的动态不存在!", id),util::NOT_EXIST)))?;
        return Ok(NewsVO::from(news_exist))
    }

    /// 动态分页
    pub async fn news_page(&self, req: &HttpRequest, param: &NewsPageDTO) -> Result<Page<NewsVO>>  {
        let mut extend = ExtendPageDTO{
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time:param.begin_time,
            end_time:param.end_time
        };
        let token = req.headers().get("access_token");
        let extract_result = &JWTToken::extract_token_by_header(token);
        if extract_result.is_err() {
            log::error!("在获取用户信息时，发生异常:{}",extract_result.clone().unwrap_err().to_string());
            return Err(crate::error::Error::from(String::from("获取用户信息失败")));
        }
        let user_info = extract_result.clone().unwrap();
        let mut arg= param.clone();
        // 用户只能看到自己组织下的数据
        arg.organize = Some(user_info.organize);

        let count_result = NewsMapper::select_count(&mut CONTEXT.business_rbatis.as_executor(), &arg,&extend).await;
        if count_result.is_err(){
            error!("在用户分页统计时，发生异常:{}",count_result.unwrap_err());
            return Err(Error::from("动态分页查询异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from(("未查询到符合条件的数据",util::NOT_EXIST)));
        }
        let mut result = Page::<NewsVO>::page_query( total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no-1)*result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = NewsMapper::select_page(&mut CONTEXT.business_rbatis.as_executor(), &arg,&extend).await;
        if page_result.is_err() {
            error!("在动态分页获取页面数据时，发生异常:{}",page_result.unwrap_err());
            return Err(Error::from("动态分页查询异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

}