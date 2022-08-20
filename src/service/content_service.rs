use std::ops::Div;
use actix_web::HttpRequest;
use chrono::Datelike;
use log::error;
use rbatis::crud::CRUD;
use rbatis::DateNative;
use rbson::Bson;
use rust_decimal::{Decimal, RoundingStrategy};
use rust_decimal::prelude::ToPrimitive;
use crate::dao::log_mapper::LogMapper;
use crate::dao::memo_mapper::MemoMapper;
use crate::dao::news_mapper::NewsMapper;
use crate::dao::notebook_mapper::NoteBookMapper;
use crate::dao::notes_mapper::NotesMapper;
use crate::entity::domain::business_database_tables::{Memo, News, NoteBook, Notes};
use crate::entity::dto::memo::{MemoDTO, MemoPageDTO};
use crate::entity::dto::news::{NewsDTO, NewsPageDTO};
use crate::entity::dto::notebook::NoteBookDTO;
use crate::entity::dto::notes::{NotesDTO, NotesPageDTO};
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::vo::jwt::JWTToken;
use crate::entity::vo::memo::MemoVO;
use crate::entity::vo::news::NewsVO;
use crate::entity::vo::notebook::NoteBookVO;
use crate::entity::vo::notes::NotesVO;
use crate::util::Page;
use crate::service::CONTEXT;
use crate::error::Error;
use crate::error::Result;
use crate::util;
use crate::util::date_time::DateUtils;

/// 文本（消息）服务
pub struct ContentService {}

impl ContentService {

    /// 发布消息动态
    pub async fn add_news(&self, req: &HttpRequest,arg: &NewsDTO) -> Result<u64> {
        let check_flag = arg.topic.is_none() || arg.topic.as_ref().unwrap().is_empty() || arg.content.is_none() || arg.content.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("动态标题和内容不能为空!",util::NOT_PARAMETER)));
        }
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
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
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX008")).await;
        return Ok(write_result?.rows_affected);
    }

    /// 修改消息动态
    pub async fn edit_news(&self, req: &HttpRequest,arg: &NewsDTO) -> Result<u64> {
        let check_flag = arg.id.is_none() || arg.topic.is_none() || arg.topic.as_ref().unwrap().is_empty() || arg.content.is_none() || arg.content.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("动态标题和内容不能为空!",util::NOT_PARAMETER)));
        }
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
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
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
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
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let mut arg= param.clone();
        // 用户只能看到自己组织下的数据
        arg.organize = Some(user_info.organize);

        let count_result = NewsMapper::select_count(&mut CONTEXT.business_rbatis.as_executor(), &arg,&extend).await;
        if count_result.is_err(){
            error!("在动态分页统计时，发生异常:{}",count_result.unwrap_err());
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

    /// 保存便笺
    pub async fn add_memo(&self, req: &HttpRequest,arg: &MemoDTO) -> Result<u64> {
        let check_flag = arg.title.is_none() || arg.title.as_ref().unwrap().is_empty() || arg.content.is_none() || arg.content.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("便笺标题和内容不能为空!",util::NOT_PARAMETER)));
        }
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let memo = Memo{
            id:None,
            organize: Some(user_info.organize),
            source:Some(user_info.account.clone()),
            title:arg.title.clone(),
            content:arg.content.clone(),
            create_time:Some(rbatis::DateTimeNative::now()),
            update_time:None,
        };
        let write_result = CONTEXT.business_rbatis.save(&memo, &[]).await;
        if  write_result.is_err(){
            error!("保存便笺时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("保存便笺失败!"));
        }
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX031")).await;
        return Ok(write_result?.rows_affected);
    }

    /// 修改便笺
    pub async fn edit_memo(&self, req: &HttpRequest,arg: &MemoDTO) -> Result<u64> {
        let check_flag = arg.id.is_none() || arg.title.is_none() || arg.title.as_ref().unwrap().is_empty() || arg.content.is_none() || arg.content.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("便笺标题和内容不能为空!",util::NOT_PARAMETER)));
        }
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let query_where = CONTEXT.business_rbatis.new_wrapper().eq(Memo::id(), &arg.id).and().eq(Memo::organize(),user_info.organize);
        let memo_option: Option<Memo> = CONTEXT.business_rbatis.fetch_by_wrapper(query_where).await?;
        let memo_exist = memo_option.ok_or_else(|| Error::from((format!("id={} 的便笺不存在!", &arg.id.clone().unwrap()), util::NOT_EXIST)))?;
        let memo = Memo{
            id:arg.id,
            organize: memo_exist.organize,
            source: Some(user_info.account.clone()),
            title: arg.title.clone(),
            content: arg.content.clone(),
            create_time: None,
            update_time: None
        };
        let result = MemoMapper::update_memo(&mut CONTEXT.business_rbatis.as_executor(),&memo).await;
        if result.is_err() {
            error!("在修改id={}的便笺时，发生异常:{}",arg.id.as_ref().unwrap(),result.unwrap_err());
            return Err(Error::from("便笺修改失败"));
        }
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX032")).await;
        return Ok(result?.rows_affected);
    }

    /// 删除便笺
    pub async fn delete_memo(&self, req: &HttpRequest,id: &u64) -> Result<u64> {
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        // 只能删除自己组织机构下的数据
        let delete_where = CONTEXT.business_rbatis.new_wrapper().eq(Memo::id(),id).and().eq(Memo::organize(),user_info.organize);
        let write_result = CONTEXT.business_rbatis.remove_by_wrapper::<Memo>(delete_where).await;
        if write_result.is_err(){
            error!("删除便笺时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("删除便笺失败!"));
        }
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX033")).await;
        return Ok(write_result?);
    }

    /// 获取便笺
    pub async fn get_memo_detail(&self,id: &u64) -> Result<MemoVO> {
        let memo_option: Option<Memo> = CONTEXT.business_rbatis.fetch_by_wrapper(CONTEXT.business_rbatis.new_wrapper().eq(Memo::id(), id)).await?;
        let memo_exist = memo_option.ok_or_else(|| Error::from((format!("id={} 的便笺不存在!", id),util::NOT_EXIST)))?;
        return Ok(MemoVO::from(memo_exist))
    }

    /// 便笺分页
    pub async fn page_memo(&self, req: &HttpRequest, param: &MemoPageDTO) -> Result<Page<MemoVO>>  {
        let mut extend = ExtendPageDTO{
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time:param.begin_time,
            end_time:param.end_time
        };
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let mut arg= param.clone();
        // 用户只能看到自己组织下的数据
        arg.organize = Some(user_info.organize);

        let count_result = MemoMapper::select_count(&mut CONTEXT.business_rbatis.as_executor(), &arg,&extend).await;
        if count_result.is_err(){
            error!("在便笺分页统计时，发生异常:{}",count_result.unwrap_err());
            return Err(Error::from("便笺分页查询异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from(("未查询到符合条件的数据",util::NOT_EXIST)));
        }
        let mut result = Page::<MemoVO>::page_query( total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no-1)*result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = MemoMapper::select_page(&mut CONTEXT.business_rbatis.as_executor(), &arg,&extend).await;
        if page_result.is_err() {
            error!("在便笺分页获取页面数据时，发生异常:{}",page_result.unwrap_err());
            return Err(Error::from("便笺分页查询异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

    /// 创建笔记簿
    pub async fn add_notebook(&self, req: &HttpRequest,arg: &NoteBookDTO) -> Result<u64> {
        let check_flag = arg.name.is_none() || arg.name.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("笔记簿名称不能为空!",util::NOT_PARAMETER)));
        }
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        // 重复校验，在同一组织下，不能重复
        let duplicate_where = CONTEXT.business_rbatis.new_wrapper().eq(NoteBook::organize(), user_info.organize).and().eq(NoteBook::name(),arg.name.clone());
        let notebook_option: Option<NoteBook> = CONTEXT.business_rbatis.fetch_by_wrapper(duplicate_where).await?;
        if notebook_option.is_some() {
            return Err(Error::from(format!(
                "该笔记簿名【{}】已存在!",
                arg.name.as_ref().unwrap()
            )));
        }
        let notebook = NoteBook{
            id:None,
            name:arg.name.clone(),
            organize: Some(user_info.organize),
            source:Some(user_info.account.clone()),
            descript:arg.descript.clone(),
            status:Some(2)
        };
        let write_result = CONTEXT.business_rbatis.save(&notebook, &[]).await;
        if  write_result.is_err(){
            error!("保存笔记簿时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("保存笔记簿失败!"));
        }
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX016")).await;
        return Ok(write_result?.rows_affected);
    }

    /// 修改笔记簿
    pub async fn edit_notebook(&self, req: &HttpRequest,arg: &NoteBookDTO) -> Result<u64> {
        let check_flag = arg.id.is_none() || arg.name.is_none() || arg.name.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("笔记簿名称不能为空!",util::NOT_PARAMETER)));
        }
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        // 重复校验，在同一组织下，不能重复
        let duplicate_where = CONTEXT.business_rbatis.new_wrapper().eq(NoteBook::organize(), user_info.organize).and().eq(NoteBook::name(),arg.name.clone()).and().ne(NoteBook::id(),arg.id);
        let notebook_option: Option<NoteBook> = CONTEXT.business_rbatis.fetch_by_wrapper(duplicate_where).await?;
        if notebook_option.is_some() {
            return Err(Error::from(format!(
                "该笔记簿名【{}】已存在!",
                arg.name.as_ref().unwrap()
            )));
        }
        let notebook = NoteBook{
            id:arg.id,
            name:arg.name.clone(),
            organize: None,
            source:Some(user_info.account.clone()),
            descript:arg.descript.clone(),
            status:arg.status
        };
        let result = NoteBookMapper::update_notebook(&mut CONTEXT.business_rbatis.as_executor(),&notebook).await;
        if result.is_err() {
            error!("在修改id={}的笔记簿时，发生异常:{}",arg.id.as_ref().unwrap(),result.unwrap_err());
            return Err(Error::from("笔记簿修改失败"));
        }
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX017")).await;
        return Ok(result?.rows_affected);
    }

    /// 删除笔记簿
    pub async fn delete_notebook(&self, req: &HttpRequest,id: &u64) -> Result<u64> {
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        // 只能删除自己组织机构下的数据
        let delete_where = CONTEXT.business_rbatis.new_wrapper().eq(NoteBook::id(),id).and().eq(NoteBook::organize(),user_info.organize);
        // 这里是级联删除，该笔记簿下的笔记也将级联删除
        let write_result = CONTEXT.business_rbatis.remove_by_wrapper::<NoteBook>(delete_where).await;
        if write_result.is_err(){
            error!("删除笔记簿时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("删除笔记簿失败!"));
        }
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX018")).await;
        return Ok(write_result?);
    }

    /// 获取笔记簿
    pub async fn list_notebook(&self, req: &HttpRequest,param: &NoteBookDTO) -> Result<Option<Vec<NoteBookVO>>>  {
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let mut arg= param.clone();
        // 用户只能看到自己组织下的数据
        arg.organize = Some(user_info.organize);
        let page_result = NoteBookMapper::select_list(&mut CONTEXT.business_rbatis.as_executor(), &arg).await;
        if page_result.is_err() {
            error!("在检索笔记簿数据时，发生异常:{}",page_result.unwrap_err());
            return Err(Error::from("检索笔记簿异常"));
        }
        let notebook_rows = page_result.unwrap();
        return Ok(notebook_rows);
    }

    /// 创建笔记
    pub async fn add_notes(&self, req: &HttpRequest,arg: &NotesDTO) -> Result<u64> {
        let check_flag = arg.notebook_id.is_none() || arg.topic.is_none() || arg.topic.as_ref().unwrap().is_empty() || arg.content.is_none() || arg.content.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("笔记标题和内容不能为空!",util::NOT_PARAMETER)));
        }
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let notes = Notes{
            id:None,
            notebook_id: arg.notebook_id,
            label:arg.label.clone(),
            topic: arg.topic.clone(),
            content:arg.content.clone(),
            source:Some(user_info.account.clone()),
            create_time:Some(rbatis::DateTimeNative::now()),
            update_time:None
        };

        let write_result = CONTEXT.business_rbatis.save(&notes, &[]).await;
        if  write_result.is_err(){
            error!("保存笔记时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("保存笔记失败!"));
        }
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX019")).await;
        return Ok(write_result?.rows_affected);
    }

    /// 修改笔记
    pub async fn edit_notes(&self, req: &HttpRequest,arg: &NotesDTO) -> Result<u64> {
        let check_flag = arg.id.is_none() || arg.notebook_id.is_none() || arg.topic.is_none() || arg.topic.as_ref().unwrap().is_empty() || arg.content.is_none() || arg.content.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("便笺标题和内容不能为空!",util::NOT_PARAMETER)));
        }
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let query_where = CONTEXT.business_rbatis.new_wrapper().eq(Notes::id(), &arg.id);
        let notes_option: Option<Notes> = CONTEXT.business_rbatis.fetch_by_wrapper(query_where).await?;
        let notes_exist = notes_option.ok_or_else(|| Error::from((format!("id={} 的笔记不存在!", arg.id.unwrap()), util::NOT_EXIST)))?;
        let notes = Notes{
            id:arg.id,
            notebook_id: arg.notebook_id,
            label:arg.label.clone(),
            topic: arg.topic.clone(),
            content:arg.content.clone(),
            source:Some(user_info.account.clone()),
            create_time:None,
            update_time:Some(rbatis::DateTimeNative::now())
        };
        let result = NotesMapper::update_notes(&mut CONTEXT.business_rbatis.as_executor(),&notes,&user_info.organize).await;
        if result.is_err() {
            error!("在修改id={}的笔记时，发生异常:{}",arg.id.as_ref().unwrap(),result.unwrap_err());
            return Err(Error::from("笔记修改失败"));
        }
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX020")).await;
        return Ok(result?.rows_affected);
    }

    /// 删除笔记
    pub async fn delete_notes(&self, req: &HttpRequest,id: &u64) -> Result<u64> {
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let result = NotesMapper::delete_notes(&mut CONTEXT.business_rbatis.as_executor(),id,&user_info.organize).await;
        if result.is_err() {
            error!("在删除id={}的笔记时，发生异常:{}",id,result.unwrap_err());
            return Err(Error::from("笔记删除失败"));
        }
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX021")).await;
        return Ok(result?.rows_affected);
    }

    /// 获取笔记详情
    pub async fn get_notes_detail(&self,id: &u64) -> Result<NotesVO> {
        let notes_option: Option<Notes> = CONTEXT.business_rbatis.fetch_by_wrapper(CONTEXT.business_rbatis.new_wrapper().eq(Notes::id(), id)).await?;
        let notes_exist = notes_option.ok_or_else(|| Error::from((format!("id={} 的动态不存在!", id),util::NOT_EXIST)))?;
        return Ok(NotesVO::from(notes_exist))
    }

    /// 笔记分页
    pub async fn page_notes(&self, req: &HttpRequest, param: &NotesPageDTO) -> Result<Page<NotesVO>>  {
        let mut extend = ExtendPageDTO{
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time:param.begin_time,
            end_time:param.end_time
        };
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let mut arg= param.clone();
        // 用户只能看到自己组织下的数据
        arg.organize = Some(user_info.organize);
        let count_result = NotesMapper::select_count(&mut CONTEXT.business_rbatis.as_executor(), &arg,&extend).await;
        if count_result.is_err(){
            error!("在笔记分页统计时，发生异常:{}",count_result.unwrap_err());
            return Err(Error::from("笔记分页查询异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from(("未查询到符合条件的数据",util::NOT_EXIST)));
        }
        let mut result = Page::<NotesVO>::page_query( total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no-1)*result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = NotesMapper::select_page(&mut CONTEXT.business_rbatis.as_executor(), &arg,&extend).await;
        if page_result.is_err() {
            error!("在便笺分页获取页面数据时，发生异常:{}",page_result.unwrap_err());
            return Err(Error::from("便笺分页查询异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

    /// 计算近6个月的动态发布情况
    pub async fn compute_pre6_news(&self, req: &HttpRequest,month:&String) ->Result<rbson::Document> {
        let user_month_wrap = DateNative::from_str(month.as_str());
        if user_month_wrap.is_err() {
            return Err(Error::from(("统计月份不能为空!",util::NOT_PARAMETER)));
        }
        let user_month = user_month_wrap.unwrap();
        // 判断是否为当前月
        let current_month = DateNative::now();
        let mut end= rbatis::DateTimeNative::now();

        // 总天数，计算日均用
        let days = if current_month.year() == user_month.year() && current_month.month() == user_month.month(){
            // 当前月只计算 已经过去的天数
            current_month.day()
        }else {
            // 当月所有的天数
            DateUtils::get_current_month_days(user_month.year(),user_month.month())
        };
        let start= rbatis::DateTimeNative::from_str((format!("{}-{:0>2}-01T00:00:00",user_month.year(),user_month.month())).as_str()).unwrap();
        let end= rbatis::DateTimeNative::from_str((format!("{}-{:0>2}-{:0>2}T00:00:00",user_month.year(),user_month.month(),days)).as_str()).unwrap();

        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let arg = NewsPageDTO{
            id: None,
            topic: None,
            label: None,
            content: None,
            source: None,
            page_no: None,
            page_size: None,
            begin_time: None,
            end_time: None,
            organize: Some(user_info.organize)
        };
        let mut extend = ExtendPageDTO{
            page_no: Some(1),
            page_size: Some(10),
            begin_time:Some(start),
            end_time:Some(end),
        };
        let count_result = NewsMapper::select_count(&mut CONTEXT.business_rbatis.as_executor(), &arg,&extend).await;
        if count_result.is_err(){
            error!("在统计指定日期范围的动态时，发生异常:{}",count_result.unwrap_err());
            return Err(Error::from("动态条数异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        let total = Decimal::from(total_row);
        let mut avg_total = total.div(Decimal::from(days));

        // 按月查询统计账单并排序
        let user_info = JWTToken::extract_user_by_request(req).unwrap();
        let query_sql = format!("call count_pre6_news({}, '{}')", &user_info.organize,month);
        let param:Vec<Bson> = Vec::new();
        let compute_result_warp = CONTEXT.business_rbatis.fetch(query_sql.as_str(), param).await;
        if compute_result_warp.is_err(){
            error!("在统计近6个月的动态发布时，发生异常:{}",compute_result_warp.unwrap_err());
            return Err(Error::from("统计近6个月的动态发布异常"));
        }
        let rows:rbson::Array = compute_result_warp.unwrap();
        let mut result = rbson::Document::new();
        result.insert("count",total.to_u64());
        result.insert("avg",avg_total.round_dp_with_strategy(2,RoundingStrategy::AwayFromZero).to_f64());
        result.insert("news6",rows);
        return Ok(result);
    }

}