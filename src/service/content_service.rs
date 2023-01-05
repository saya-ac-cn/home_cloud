use actix_web::HttpRequest;
use chrono::Datelike;
use log::error;
use rbs::to_value;
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
use crate::error::Error;
use crate::error::Result;
use crate::{business_rbatis_pool, primary_rbatis_pool, util};
use crate::entity::vo::total_news::TotalNewsVO;
use crate::entity::vo::total_pre_6_month::TotalPre6MonthVO;
use crate::util::date_time::{DateTimeUtil, DateUtils};
use crate::util::editor::Editor;

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
        // 生成一次简述
        let abstracts = Editor::get_content(arg.content.clone().unwrap().as_str());
        let news = News{
            id:None,
            topic:arg.topic.clone(),
            label:arg.label.clone(),
            abstracts:Some(abstracts),
            content:arg.content.clone(),
            organize: Some(user_info.organize),
            source:Some(user_info.account.clone()),
            create_time:DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()),&util::FORMAT_Y_M_D_H_M_S),
            update_time:None,
        };
        let write_result = News::insert(business_rbatis_pool!(),&news).await;
        if  write_result.is_err(){
            error!("保存消息动态时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("发布消息动态失败!"));
        }
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX008")).await;
        return Ok(write_result?.rows_affected);
    }

    /// 修改消息动态
    pub async fn edit_news(&self, req: &HttpRequest,arg: &NewsDTO) -> Result<u64> {
        let check_flag = arg.id.is_none() || arg.topic.is_none() || arg.topic.as_ref().unwrap().is_empty() || arg.content.is_none() || arg.content.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("动态标题和内容不能为空!",util::NOT_PARAMETER)));
        }
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let query_news_wrap = News::select_by_id_organize(business_rbatis_pool!(),  &arg.id.clone().unwrap(),&user_info.organize).await;
        if query_news_wrap.is_err() {
            error!("查询动态异常：{}",query_news_wrap.unwrap_err());
            return Err(Error::from("查询动态失败!"));
        }
        let news_option = query_news_wrap.unwrap().into_iter().next();
        let news_exist = news_option.ok_or_else(|| Error::from((format!("id={} 的动态不存在!", &arg.id.clone().unwrap()),util::NOT_EXIST)))?;
        // 生成一次简述
        let abstracts = Editor::get_content(arg.content.clone().unwrap().as_str());
        let news = News{
            id:arg.id,
            topic: arg.topic.clone(),
            label: arg.label.clone(),
            abstracts:Some(abstracts),
            content: arg.content.clone(),
            organize: news_exist.organize,
            source: Some(user_info.account.clone()),
            create_time: None,
            update_time: None
        };
        let result = NewsMapper::update_news(business_rbatis_pool!(),&news).await;
        if result.is_err() {
            error!("在修改id={}的动态时，发生异常:{}",arg.id.as_ref().unwrap(),result.unwrap_err());
            return Err(Error::from("动态修改失败"));
        }
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX009")).await;
        return Ok(result?.rows_affected);
    }

    /// 删除消息动态
    pub async fn delete_news(&self, req: &HttpRequest,id: &u64) -> Result<u64> {
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        // 只能删除自己组织机构下的数据
        let write_result = News::delete_by_id_organize(business_rbatis_pool!(),id,&user_info.organize).await;
        if write_result.is_err(){
            error!("删除消息动态时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("删除消息动态失败!"));
        }
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX010")).await;
        return Ok(write_result?.rows_affected);
    }

    /// 获取消息动态
    pub async fn get_news_detail(&self,id: &u64) -> Result<NewsVO> {
        let query_news_wrap = News::select_by_column(business_rbatis_pool!(),  News::id(), id).await;
        if query_news_wrap.is_err() {
            error!("查询动态异常：{}",query_news_wrap.unwrap_err());
            return Err(Error::from("查询动态失败!"));
        }
        let news_option = query_news_wrap.unwrap().into_iter().next();
        let news_exist = news_option.ok_or_else(|| Error::from((format!("id={} 的动态不存在!", id),util::NOT_EXIST)))?;
        return Ok(NewsVO::from(news_exist))
    }

    /// 获取消息动态详情[公众]
    pub async fn public_news_detail(&self,organize:&u64,id: &u64) -> Result<rbson::Document> {
        let query_sql = format!("call news_pre_and_next({}, {})",organize,id);
        let compute_result_warp = business_rbatis_pool!().fetch_decode::<Vec<NewsVO>>(query_sql.as_str(), vec![]).await;
        if compute_result_warp.is_err(){
            error!("在查询id={}附近的动态时，发生异常:{}",id,compute_result_warp.unwrap_err());
            return Err(Error::from("查询动态异常"));
        }
        let rows = compute_result_warp.unwrap();
        if rows.is_empty(){
            return Err(Error::from(("未查询到符合条件的数据",util::NOT_EXIST)));
        }
        if rows.len() == 1 && rows[0]. id.unwrap() != *id{
            return Err(Error::from(("未查询到符合条件的数据",util::NOT_EXIST)));
        }
        let mut result= rbson::Document::new();
        for news in rows {
            let news_id = news.id.unwrap();
            let mut _item = rbson::Document::new();
            _item.insert("id",news.id);
            _item.insert("topic",news.topic);
            _item.insert("label",news.label);
            _item.insert("content",news.content);
            _item.insert("source",news.source);
            _item.insert("create_time",news.create_time);
            if news_id == *id  {
                result.insert("now",_item);
            }else if news_id < *id {
                result.insert("pre",_item);
            }else {
                result.insert("next",_item);
            }
        }
        return Ok(result);
    }

    /// 动态分页
    pub async fn news_page(&self, req: &HttpRequest,organize: Option<u64>, param: &NewsPageDTO) -> Result<Page<NewsVO>>  {
        let mut extend = ExtendPageDTO{
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time:param.begin_time.clone(),
            end_time:param.end_time.clone()
        };
        let mut arg= param.clone();
        if organize.is_some() {
            arg.organize = organize;
        }else {
            let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
            // 用户只能看到自己组织下的数据
            arg.organize = Some(user_info.organize);
        }
        let count_result = NewsMapper::select_count(business_rbatis_pool!(), &arg,&extend).await;
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
        let page_result = NewsMapper::select_page(business_rbatis_pool!(), &arg,&extend).await;
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
            create_time:DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()),&util::FORMAT_Y_M_D_H_M_S),
            update_time:None,
        };
        let write_result = Memo::insert(business_rbatis_pool!(),&memo).await;
        if  write_result.is_err(){
            error!("保存便笺时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("保存便笺失败!"));
        }
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX031")).await;
        return Ok(write_result?.rows_affected);
    }

    /// 修改便笺
    pub async fn edit_memo(&self, req: &HttpRequest,arg: &MemoDTO) -> Result<u64> {
        let check_flag = arg.id.is_none() || arg.title.is_none() || arg.title.as_ref().unwrap().is_empty() || arg.content.is_none() || arg.content.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("便笺标题和内容不能为空!",util::NOT_PARAMETER)));
        }
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let query_memo_wrap = Memo::select_by_id_organize(business_rbatis_pool!(),  &arg.id.clone().unwrap(),&user_info.organize).await;
        if query_memo_wrap.is_err() {
            error!("查询便笺异常：{}",query_memo_wrap.unwrap_err());
            return Err(Error::from("查询便笺失败!"));
        }
        let memo_option = query_memo_wrap.unwrap().into_iter().next();
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
        let result = MemoMapper::update_memo(business_rbatis_pool!(),&memo).await;
        if result.is_err() {
            error!("在修改id={}的便笺时，发生异常:{}",arg.id.as_ref().unwrap(),result.unwrap_err());
            return Err(Error::from("便笺修改失败"));
        }
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX032")).await;
        return Ok(result?.rows_affected);
    }

    /// 删除便笺
    pub async fn delete_memo(&self, req: &HttpRequest,id: &u64) -> Result<u64> {
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        // 只能删除自己组织机构下的数据
        let write_result = Memo::delete_by_id_organize(business_rbatis_pool!(),id,&user_info.organize).await;
        if write_result.is_err(){
            error!("删除便笺时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("删除便笺失败!"));
        }
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX033")).await;
        return Ok(write_result?.rows_affected);
    }

    /// 获取便笺
    pub async fn get_memo_detail(&self,id: &u64) -> Result<MemoVO> {
        let query_memo_wrap = Memo::select_by_column(business_rbatis_pool!(),  Memo::id(), id).await;
        if query_memo_wrap.is_err() {
            error!("查询便笺异常：{}",query_memo_wrap.unwrap_err());
            return Err(Error::from("查询便笺失败!"));
        }
        let memo_option = query_memo_wrap.unwrap().into_iter().next();
        let memo_exist = memo_option.ok_or_else(|| Error::from((format!("id={} 的便笺不存在!", id),util::NOT_EXIST)))?;
        return Ok(MemoVO::from(memo_exist))
    }

    /// 便笺分页
    pub async fn page_memo(&self, req: &HttpRequest, param: &MemoPageDTO) -> Result<Page<MemoVO>>  {
        let mut extend = ExtendPageDTO{
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time:param.begin_time.clone(),
            end_time:param.end_time.clone()
        };
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let mut arg= param.clone();
        // 用户只能看到自己组织下的数据
        arg.organize = Some(user_info.organize);

        let count_result = MemoMapper::select_count(business_rbatis_pool!(), &arg,&extend).await;
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
        let page_result = MemoMapper::select_page(business_rbatis_pool!(), &arg,&extend).await;
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
        let query_notebook_wrap = NoteBook::select_by_organize_name(business_rbatis_pool!(),  &arg.name.clone().unwrap(),&user_info.organize).await;
        if query_notebook_wrap.is_err() {
            error!("查询笔记簿异常：{}",query_notebook_wrap.unwrap_err());
            return Err(Error::from("查询笔记簿失败!"));
        }
        let notebook_option = query_notebook_wrap.unwrap().into_iter().next();
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
        let write_result = NoteBook::insert(business_rbatis_pool!(),&notebook).await;
        if  write_result.is_err(){
            error!("保存笔记簿时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("保存笔记簿失败!"));
        }
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX016")).await;
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
        let query_notebook_wrap = NoteBook::select_for_repeat(business_rbatis_pool!(),  &arg.id.clone().unwrap(),&arg.name.clone().unwrap(),&user_info.organize).await;
        if query_notebook_wrap.is_err() {
            error!("查询笔记簿异常：{}",query_notebook_wrap.unwrap_err());
            return Err(Error::from("查询笔记簿失败!"));
        }
        let notebook_option = query_notebook_wrap.unwrap().into_iter().next();
        if notebook_option.is_some() {
            return Err(Error::from(format!(
                "该笔记簿名【{}】已存在!",
                arg.name.as_ref().unwrap()
            )));
        }
        let notebook = NoteBook{
            id:arg.id,
            name:arg.name.clone(),
            organize: Some(user_info.organize),
            source:Some(user_info.account.clone()),
            descript:arg.descript.clone(),
            status:arg.status
        };
        let result = NoteBookMapper::update_notebook(business_rbatis_pool!(),&notebook).await;
        if result.is_err() {
            error!("在修改id={}的笔记簿时，发生异常:{}",arg.id.as_ref().unwrap(),result.unwrap_err());
            return Err(Error::from("笔记簿修改失败"));
        }
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX017")).await;
        return Ok(result?.rows_affected);
    }

    /// 删除笔记簿
    pub async fn delete_notebook(&self, req: &HttpRequest,id: &u64) -> Result<u64> {
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        // 只能删除自己组织机构下的数据
        // 这里是级联删除，该笔记簿下的笔记也将级联删除
        let write_result = NoteBook::delete_by_id_organize(business_rbatis_pool!(),id,&user_info.organize).await;
        if write_result.is_err(){
            error!("删除笔记簿时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("删除笔记簿失败!"));
        }
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX018")).await;
        return Ok(write_result?.rows_affected);
    }

    /// 获取笔记簿
    pub async fn list_notebook(&self, req: &HttpRequest,organize:Option<u64>,param: &NoteBookDTO) -> Result<Option<Vec<NoteBookVO>>>  {
        let mut arg= param.clone();
        if organize.is_none() {
            // 用户只能看到自己组织下的数据
            let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
            arg.organize = Some(user_info.organize);
        }else {
            arg.organize = organize;
        }
        let page_result = NoteBookMapper::select_list(business_rbatis_pool!(), &arg).await;
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
        // 生成一次简述
        let abstracts = Editor::get_content(arg.content.clone().unwrap().as_str());
        let notes = Notes{
            id:None,
            notebook_id: arg.notebook_id,
            label:arg.label.clone(),
            topic: arg.topic.clone(),
            abstracts:Some(abstracts),
            content:arg.content.clone(),
            source:Some(user_info.account.clone()),
            create_time:DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()),&util::FORMAT_Y_M_D_H_M_S),
            update_time:None
        };

        let write_result = Notes::insert(business_rbatis_pool!(),&notes).await;
        if  write_result.is_err(){
            error!("保存笔记时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("保存笔记失败!"));
        }
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX019")).await;
        return Ok(write_result?.rows_affected);
    }

    /// 修改笔记
    pub async fn edit_notes(&self, req: &HttpRequest,arg: &NotesDTO) -> Result<u64> {
        let check_flag = arg.id.is_none() || arg.notebook_id.is_none() || arg.topic.is_none() || arg.topic.as_ref().unwrap().is_empty() || arg.content.is_none() || arg.content.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("便笺标题和内容不能为空!",util::NOT_PARAMETER)));
        }
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let query_notes_wrap = Notes::select_by_column(business_rbatis_pool!(),  Notes::id(), &arg.id).await;
        if query_notes_wrap.is_err() {
            error!("查询笔记异常：{}",query_notes_wrap.unwrap_err());
            return Err(Error::from("查询笔记失败!"));
        }
        let notes_option = query_notes_wrap.unwrap().into_iter().next();
        let notes_exist = notes_option.ok_or_else(|| Error::from((format!("id={} 的笔记不存在!", arg.id.unwrap()), util::NOT_EXIST)))?;
        // 生成一次简述
        let abstracts = Editor::get_content(arg.content.clone().unwrap().as_str());
        let notes = Notes{
            id:arg.id,
            notebook_id: arg.notebook_id,
            label:arg.label.clone(),
            topic: arg.topic.clone(),
            abstracts:Some(abstracts),
            content:arg.content.clone(),
            source:Some(user_info.account.clone()),
            create_time:None,
            update_time:None
        };
        let result = NotesMapper::update_notes(business_rbatis_pool!(),&notes,&user_info.organize).await;
        if result.is_err() {
            error!("在修改id={}的笔记时，发生异常:{}",arg.id.as_ref().unwrap(),result.unwrap_err());
            return Err(Error::from("笔记修改失败"));
        }
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX020")).await;
        return Ok(result?.rows_affected);
    }

    /// 删除笔记
    pub async fn delete_notes(&self, req: &HttpRequest,id: &u64) -> Result<u64> {
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let result = NotesMapper::delete_notes(business_rbatis_pool!(),id,&user_info.organize).await;
        if result.is_err() {
            error!("在删除id={}的笔记时，发生异常:{}",id,result.unwrap_err());
            return Err(Error::from("笔记删除失败"));
        }
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX021")).await;
        return Ok(result?.rows_affected);
    }

    /// 获取笔记详情
    pub async fn get_notes_detail(&self,id: &u64) -> Result<NotesVO> {
        let query_notes_wrap = Notes::select_by_column(business_rbatis_pool!(),  Notes::id(), id).await;
        if query_notes_wrap.is_err() {
            error!("查询笔记异常：{}",query_notes_wrap.unwrap_err());
            return Err(Error::from("查询笔记失败!"));
        }
        let notes_option = query_notes_wrap.unwrap().into_iter().next();
        let notes_exist = notes_option.ok_or_else(|| Error::from((format!("id={} 的动态不存在!", id),util::NOT_EXIST)))?;
        return Ok(NotesVO::from(notes_exist))
    }

    /// 获取笔记详情[公众]
    pub async fn public_notes_detail(&self,organize:&u64,id: &u64) -> Result<rbson::Document> {
        let query_sql = format!("call notes_pre_and_next({}, {})",organize,id);
        let compute_result_warp = business_rbatis_pool!().fetch_decode::<Vec<NotesVO>>(query_sql.as_str(), vec![]).await;
        if compute_result_warp.is_err(){
            error!("在查询id={}附近的笔记时，发生异常:{}",id,compute_result_warp.unwrap_err());
            return Err(Error::from("查询笔记异常"));
        }
        let rows = compute_result_warp.unwrap();
        if rows.is_empty(){
            return Err(Error::from(("未查询到符合条件的数据",util::NOT_EXIST)));
        }
        if rows.len() == 1 && rows[0]. id.unwrap() != *id{
            return Err(Error::from(("未查询到符合条件的数据",util::NOT_EXIST)));
        }
        let mut result= rbson::Document::new();
        for news in rows {
            let notes_id = news.id.unwrap();
            let mut _item = rbson::Document::new();
            _item.insert("id",news.id);
            _item.insert("topic",news.topic);
            _item.insert("content",news.content);
            _item.insert("label",news.label);
            _item.insert("source",news.source);
            _item.insert("create_time",news.create_time);
            if notes_id < *id {
                result.insert("pre",_item);
            }else if notes_id > *id{
                result.insert("next",_item);
            }else{
                result.insert("now",_item);
            }
        }
        return Ok(result);
    }

    /// 笔记分页
    pub async fn page_notes(&self, req: &HttpRequest, organize: Option<u64>,param: &NotesPageDTO) -> Result<Page<NotesVO>>  {
        let mut extend = ExtendPageDTO{
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time:param.begin_time.clone(),
            end_time:param.end_time.clone()
        };
        let mut arg= param.clone();
        if organize.is_none() {
            let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
            // 用户只能看到自己组织下的数据
            arg.organize = Some(user_info.organize);
        }else {
            arg.organize = organize;
        }
        let count_result = NotesMapper::select_count(business_rbatis_pool!(), &arg,&extend).await;
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
        let page_result = NotesMapper::select_page(business_rbatis_pool!(), &arg,&extend).await;
        if page_result.is_err() {
            error!("在便笺分页获取页面数据时，发生异常:{}",page_result.unwrap_err());
            return Err(Error::from("便笺分页查询异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

    /// 计算近6个月的动态发布情况
    pub async fn compute_pre6_news(&self, req: &HttpRequest,month:&String) ->Result<TotalNewsVO> {

        let user_info = JWTToken::extract_user_by_request(req).unwrap();
        let user_month_wrap = chrono::NaiveDate::parse_from_str(month.as_str(),&util::FORMAT_Y_M_D);
        if user_month_wrap.is_err() {
            return Err(Error::from(("统计月份不能为空!", util::NOT_PARAMETER)));
        }
        let user_month = user_month_wrap.unwrap();
        // 查询用户在制定月份发表的动态
        let query_current_month_news_sql = "select count(1) from `news` where `organize` = ? and  `create_time` like concat(date_format(?,'%Y-%m'),'%')";
        let current_month_news_result_warp = business_rbatis_pool!().fetch_decode::<u64>(query_current_month_news_sql, vec![to_value!(user_info.organize),to_value!(month.as_str())]).await;
        let mut current_month_news:u64 = 0;
        if current_month_news_result_warp.is_ok(){
            current_month_news = current_month_news_result_warp.unwrap()
        }else {
            error!("在查询指定月份的动态数据总数时，发生异常:{}",current_month_news_result_warp.unwrap_err());
        }
        let user_month = user_month_wrap.unwrap();
        // 判断是否为当前月
        let current_month = DateUtils::now().date();

        // 总天数，计算日均用
        let days = if current_month.year() == user_month.year() && current_month.month() == user_month.month(){
            // 当前月只计算 已经过去的天数
            current_month.day()
        }else {
            // 当月所有的天数
            DateUtils::get_current_month_days(user_month.year(),user_month.month())
        };
        let avg:u64 = current_month_news/(days as u64);
        let query_sql = format!("call count_pre6_news({}, '{}')", &user_info.organize,month);
        let compute_result_warp = business_rbatis_pool!().fetch_decode::<Vec<TotalPre6MonthVO>>(query_sql.as_str(), vec![]).await;
        if compute_result_warp.is_err(){
            error!("在统计近6个月的动态发布时，发生异常:{}",compute_result_warp.unwrap_err());
            return Err(Error::from("统计近6个月的动态发布异常"));
        }
        let rows = compute_result_warp.unwrap();
        let result = TotalNewsVO{
            avg:Some(avg),
            count:Some(current_month_news),
            news6:Some(rows)
        };
        return Ok(result);
    }

}