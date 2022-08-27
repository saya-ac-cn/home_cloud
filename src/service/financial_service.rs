use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use actix_http::StatusCode;
use actix_web::{HttpRequest, HttpResponse};
use chrono::{Datelike};
use log::error;
use rbatis::crud::{CRUD, CRUDMut};
use rbatis::DateNative;
use rust_decimal::{Decimal, RoundingStrategy};
use rust_decimal::prelude::{ToPrimitive, Zero};
use crate::dao::general_journal_mapper::GeneralJournalMapper;
use crate::dao::journal_mapper::JournalMapper;
use crate::dao::log_mapper::LogMapper;
use crate::entity::domain::financial_database_tables::{Abstracts, GeneralJournal, Journal, Monetary, PaymentMeans};
use crate::entity::dto::general_journal::GeneralJournalDTO;
use crate::entity::dto::journal::{JournalDTO, JournalPageDTO};
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::vo::abstracts::AbstractsVO;
use crate::entity::vo::general_journal::{GeneralJournalCollectVO, GeneralJournalVO};
use crate::entity::vo::journal::JournalVO;
use crate::entity::vo::jwt::JWTToken;
use crate::entity::vo::monetary::MonetaryVO;
use crate::entity::vo::payment_means::PaymentMeansVO;
use crate::error::Result;
use crate::service::CONTEXT;
use crate::error::Error;
use crate::util;
use crate::util::Page;
extern crate simple_excel_writer as excel;

use excel::*;
use rbson::Bson;
use crate::util::date_time::DateUtils;

/// 财政服务
pub struct FinancialService {}

impl FinancialService {

    /// 添加流水（主+子）
    pub async fn add_journal(&self, req: &HttpRequest,arg: &JournalDTO) -> Result<u64> {
        let check_flag = arg.monetary_id.is_none() || arg.means_id.is_none() || arg.abstract_id.is_none() || arg.details.is_none() || arg.archive_date.is_none();
        if check_flag{
            return Err(Error::from(("支付方式、摘要、交易货币、交易日期和流水明细不能为空!",util::NOT_PARAMETER)));
        }
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        // 总收入
        let mut income:Decimal = Decimal::from(0);
        // 总支出
        let mut outlay:Decimal = Decimal::from(0);
        // 从流水明细中，计算总收入 & 总支出
        let general_journal = arg.details.clone().ok_or_else(|| Error::from(("流水明细不能为空",util::NOT_CHECKING)))?;
        for info in general_journal {
            let flag = info.flag.unwrap();
            let amount = info.amount.unwrap();
            if "1" == flag{
                // 收入
                income = income.add(amount);
            } else if "2" == flag{
                // 支出
                outlay = outlay.add(amount);
            } else {
                return Err(Error::from(("未知的收支类型!",util::CODE_FAIL)));
            }
        }
        // 当日总收支（存入+支取）
        let mut total:Decimal = Decimal::from(0);
        total = total.add(income.clone());
        total = total.add(outlay.clone());
        let journal = Journal{
            id:None,
            monetary_id: arg.monetary_id,
            income: Some(income),
            outlay: Some(outlay),
            means_id: arg.means_id,
            abstract_id: arg.abstract_id,
            total: Some(total),
            remarks: arg.remarks.clone(),
            archive_date: arg.archive_date,
            organize: Some(user_info.organize),
            source:Some(user_info.account.clone()),
            create_time:Some(rbatis::DateTimeNative::now()),
            update_time: None
        };
        // 写入流水记录
        let mut tx = CONTEXT.financial_rbatis.acquire_begin().await.unwrap();
        let add_journal_result = tx.save(&journal, &[]).await;
        if add_journal_result.is_err() {
            error!("在保存流水时，发生异常:{}",add_journal_result.unwrap_err());
            tx.rollback();
            return Err(Error::from("保存流水失败"));
        }
        let journal_id = add_journal_result.unwrap().last_insert_id;
        // 构造流水明细
        let general_journal = arg.details.clone().unwrap();
        let mut details:Vec<GeneralJournal> = Vec::new();
        for info in general_journal {
            details.push(GeneralJournal {
                id:None,
                journal_id: journal_id.unwrap().to_u64(),
                flag: info.flag,
                amount: info.amount.clone(),
                remarks: info.remarks.clone()
            });
        }
        let add_general_journal_result = tx.save_batch(&details, &[]).await;
        if add_general_journal_result.is_err() {
            error!("在保存流水时，发生异常:{}",add_general_journal_result.unwrap_err());
            tx.rollback();
            return Err(Error::from("保存流水失败"));
        }
        // 所有的写入都成功，最后正式提交
        tx.commit().await;
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX025")).await;
        return Ok(add_general_journal_result?.rows_affected);
    }

    /// 修改流水（父记录）
    pub async fn edit_journal(&self, req: &HttpRequest,arg: &JournalDTO) -> Result<u64> {
        let check_flag = arg.id.is_none() || arg.monetary_id.is_none() || arg.means_id.is_none() || arg.abstract_id.is_none() || arg.archive_date.is_none();
        if check_flag{
            return Err(Error::from(("支付方式、摘要、交易货币、交易日期和流水明细不能为空!",util::NOT_PARAMETER)));
        }
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let query_where = CONTEXT.financial_rbatis.new_wrapper().eq(Journal::id(), &arg.id).and().eq(Journal::organize(),user_info.organize);
        let journal_option: Option<Journal> = CONTEXT.financial_rbatis.fetch_by_wrapper(query_where).await?;
        let journal_exist = journal_option.ok_or_else(|| Error::from((format!("id={} 的流水不存在!", &arg.id.clone().unwrap()),util::NOT_EXIST)))?;
        // 历史数据不允许操作
        let archive_date = journal_exist.archive_date.unwrap();
        let current = DateNative::now();
        if current.year() != archive_date.year() || current.month() != archive_date.month(){
            return Err(Error::from("只允许修改本月的流水，历史流水已归档，不允许操作"));
        }

        let journal = Journal{
            id:journal_exist.id,
            monetary_id: arg.monetary_id,
            income: None,
            outlay: None,
            means_id: arg.means_id,
            abstract_id: arg.abstract_id,
            total: None,
            remarks: arg.remarks.clone(),
            archive_date: arg.archive_date,
            organize: journal_exist.organize,
            source:Some(user_info.account.clone()),
            create_time: None,
            update_time: Some(rbatis::DateTimeNative::now())
        };
        let result = JournalMapper::update_journal(&mut CONTEXT.financial_rbatis.as_executor(),&journal).await;
        if result.is_err() {
            error!("在修改id={}的流水时，发生异常:{}",arg.id.as_ref().unwrap(),result.unwrap_err());
            return Err(Error::from("流水修改失败"));
        }
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX026")).await;
        return Ok(result?.rows_affected);
    }

    /// 级联删除流水（主+子）
    pub async fn delete_journal(&self, req: &HttpRequest,id: &u64) -> Result<u64> {
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        // 只能删除自己组织机构下的数据
        let delete_where = CONTEXT.financial_rbatis.new_wrapper().eq(Journal::id(),id).and().eq(Journal::organize(),user_info.organize);
        let journal_option: Option<Journal> = CONTEXT.financial_rbatis.fetch_by_wrapper(delete_where.clone()).await?;
        let journal = journal_option.ok_or_else(|| Error::from((format!("id={} 的流水不存在!", id),util::NOT_EXIST)))?;
        // 历史数据不允许操作
        let archive_date = journal.archive_date.unwrap();
        let current = DateNative::now();
        if current.year() != archive_date.year() || current.month() != archive_date.month(){
            return Err(Error::from("只允许删除本月的流水，历史流水已归档，不允许操作"));
        }
        let write_result = CONTEXT.financial_rbatis.remove_by_wrapper::<Journal>(delete_where).await;
        if write_result.is_err(){
            error!("删除流水时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("删除流水失败!"));
        }
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX026")).await;
        return Ok(write_result?);
    }

    /// 添加流水明细
    pub async fn add_general_journal(&self, req: &HttpRequest,arg: &GeneralJournalDTO) -> Result<u64>{
        let check_flag = arg.journal_id.is_none() || arg.flag.is_none() || arg.flag.as_ref().unwrap().is_empty() ||arg.amount.is_none() || arg.remarks.is_none() || arg.remarks.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("流水号、收支类型、金额和备注不能为空!",util::NOT_PARAMETER)));
        }
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        // 找到流水记录
        let query_where = CONTEXT.financial_rbatis.new_wrapper().eq(Journal::id(), &arg.journal_id).and().eq(Journal::organize(),user_info.organize);
        let journal_option: Option<Journal> = CONTEXT.financial_rbatis.fetch_by_wrapper(query_where).await?;
        let journal_exist = journal_option.ok_or_else(|| Error::from((format!("id={} 的流水不存在!", &arg.id.clone().unwrap()),util::NOT_EXIST)))?;
        // 历史数据不允许操作
        let archive_date = journal_exist.archive_date.unwrap();
        let current = DateNative::now();
        if current.year() != archive_date.year() || current.month() != archive_date.month(){
            return Err(Error::from("只允许修改本月的流水，历史流水已归档，不允许操作"));
        }
        // 重新计算新的总收入、总支出已经收支总额
        let flag = arg.flag.clone().unwrap();
        let amount = arg.amount.unwrap();
        let mut income = journal_exist.income.unwrap();
        let mut outlay = journal_exist.outlay.unwrap();
        if "1" == flag{
            // 收入
            income = income.add(amount);
        } else if "2" == flag{
            // 支出
            outlay = outlay.add(amount);
        } else {
            return Err(Error::from(("未知的收支类型!",util::CODE_FAIL)));
        }
        let mut total = Decimal::from(0);
        total = total.add(income.clone());
        total = total.add(outlay.clone());

        let journal = Journal{
            id:journal_exist.id,
            monetary_id: None,
            income: Some(income),
            outlay: Some(outlay),
            means_id: None,
            abstract_id: None,
            total: Some(total),
            remarks: None,
            archive_date: None,
            organize: journal_exist.organize,
            source:Some(user_info.account.clone()),
            create_time:None,
            update_time: Some(rbatis::DateTimeNative::now())
        };

        // 修改流水记录
        let mut tx = CONTEXT.financial_rbatis.acquire_begin().await.unwrap();
        let edit_journal_result = JournalMapper::update_journal(&mut tx.as_executor(), &journal).await;
        if edit_journal_result.is_err() {
            error!("在修改id={}的流水时，发生异常:{}",arg.id.as_ref().unwrap(),edit_journal_result.unwrap_err());
            tx.rollback();
            return Err(Error::from("流水修改失败"));
        }
        let general_journal = GeneralJournal{
            id:None,
            journal_id:journal_exist.id,
            flag:Some(flag),
            amount: arg.amount,
            remarks:arg.remarks.clone()
        };

        // 添加流水明细
        let add_general_journal_result =  tx.save(&general_journal, &[]).await;
        if add_general_journal_result.is_err() {
            error!("在保存流水时，发生异常:{}",add_general_journal_result.unwrap_err());
            tx.rollback();
            return Err(Error::from("保存流水失败"));
        }
        // 所有的写入都成功，最后正式提交
        tx.commit().await;
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX028")).await;
        return Ok(edit_journal_result?.rows_affected);
    }

    /// 修改流水明细
    pub async fn edit_general_journal(&self, req: &HttpRequest,arg: &GeneralJournalDTO) -> Result<u64>{
        let check_flag = arg.id.is_none() || arg.journal_id.is_none() || arg.flag.is_none() || arg.flag.as_ref().unwrap().is_empty() || arg.amount.is_none() || arg.remarks.is_none() || arg.remarks.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("流水号、收支类型、金额和备注不能为空!",util::NOT_PARAMETER)));
        }
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        // 找出流水记录
        let query_where = CONTEXT.financial_rbatis.new_wrapper().eq(Journal::id(), &arg.journal_id).and().eq(Journal::organize(),user_info.organize);
        let journal_option: Option<Journal> = CONTEXT.financial_rbatis.fetch_by_wrapper(query_where).await?;
        let journal_exist = journal_option.ok_or_else(|| Error::from((format!("id={} 的流水不存在!", &arg.id.clone().unwrap()),util::NOT_EXIST)))?;
        // 历史数据不允许操作
        let archive_date = journal_exist.archive_date.unwrap();
        let current = DateNative::now();
        if current.year() != archive_date.year() || current.month() != archive_date.month(){
            return Err(Error::from("只允许修改本月的流水，历史流水已归档，不允许操作"));
        }
        // 找到修改前的流水明细
        let query_where = CONTEXT.financial_rbatis.new_wrapper().eq(GeneralJournal::id(), &arg.id);
        let general_journal_option: Option<GeneralJournal> = CONTEXT.financial_rbatis.fetch_by_wrapper(query_where).await?;
        let general_journal_exist = general_journal_option.ok_or_else(|| Error::from((format!("id={} 的流水明细不存在!", &arg.id.clone().unwrap()),util::NOT_EXIST)))?;
        // 把原来的流水金额 减去 要修改的流水明细金额
        let mut income = journal_exist.income.unwrap();
        let mut outlay = journal_exist.outlay.unwrap();
        let last_version_flag = general_journal_exist.flag.unwrap();
        let last_version_amount = general_journal_exist.amount.unwrap();
        if "1" == last_version_flag{
            income = income.sub(last_version_amount);
        } else if "2" == last_version_flag{
            outlay = outlay.sub(last_version_amount);
        } else {
            return Err(Error::from(("未知的收支类型!",util::CODE_FAIL)));
        }
        // 把核减后的流水金额 加上 本次修改后的金额
        let current_version_flag = arg.flag.clone().unwrap();
        let current_version_amount = arg.amount.unwrap();
        if "1" == current_version_flag{
            income = income.add(current_version_amount);
        } else if "2" == current_version_flag{
            outlay = outlay.add(current_version_amount);
        } else {
            return Err(Error::from(("未知的收支类型!",util::CODE_FAIL)));
        }
        // 把最终的收支进行一次汇总
        let mut total = Decimal::from(0);
        total = total.add(income.clone());
        total = total.add(outlay.clone());
        let journal = Journal{
            id:journal_exist.id,
            monetary_id: None,
            income: Some(income),
            outlay: Some(outlay),
            means_id: None,
            abstract_id: None,
            total: Some(total),
            remarks: None,
            archive_date: None,
            organize: journal_exist.organize,
            source:Some(user_info.account.clone()),
            create_time:None,
            update_time: Some(rbatis::DateTimeNative::now())
        };

        // 修改流水记录
        let mut tx = CONTEXT.financial_rbatis.acquire_begin().await.unwrap();
        let edit_journal_result = JournalMapper::update_journal(&mut tx.as_executor(), &journal).await;
        if edit_journal_result.is_err() {
            error!("在修改id={}的流水时，发生异常:{}",arg.journal_id.as_ref().unwrap(),edit_journal_result.unwrap_err());
            tx.rollback();
            return Err(Error::from("流水修改失败"));
        }
        // 修改流水明细
        let general_journal = GeneralJournal{
            id:arg.id,
            journal_id:journal_exist.id,
            flag:Some(current_version_flag),
            amount: arg.amount,
            remarks:arg.remarks.clone()
        };
        let edit_general_journal_result = GeneralJournalMapper::update_general_journal(&mut tx.as_executor(), &general_journal).await;
        if edit_general_journal_result.is_err() {
            error!("在修改id={}的流水明细时，发生异常:{}",arg.id.as_ref().unwrap(),edit_general_journal_result.unwrap_err());
            tx.rollback();
            return Err(Error::from("流水明细修改失败"));
        }
        // 所有的写入都成功，最后正式提交
        tx.commit().await;
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX029")).await;
        return Ok(edit_general_journal_result?.rows_affected);
    }

    /// 删除流水明细
    pub async fn delete_general_journal(&self, req: &HttpRequest,id: &u64) -> Result<u64>{
        // 找出流水明细
        let general_journal_where = CONTEXT.financial_rbatis.new_wrapper().eq(GeneralJournal::id(), id);
        let general_journal_option: Option<GeneralJournal> = CONTEXT.financial_rbatis.fetch_by_wrapper(general_journal_where).await?;
        let general_journal_exist = general_journal_option.ok_or_else(|| Error::from((format!("id={} 的流水明细不存在!", id),util::NOT_EXIST)))?;
        // 找出关联的流水
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let journal_where = CONTEXT.financial_rbatis.new_wrapper().eq(Journal::id(), &general_journal_exist.journal_id).and().eq(Journal::organize(),user_info.organize);
        let journal_option: Option<Journal> = CONTEXT.financial_rbatis.fetch_by_wrapper(journal_where.clone()).await?;
        let journal_exist = journal_option.ok_or_else(|| Error::from((format!("id={} 的流水不存在!", &general_journal_exist.journal_id.unwrap()),util::NOT_EXIST)))?;
        let archive_date = journal_exist.archive_date.unwrap();
        // 历史数据不允许操作
        let current = DateNative::now();
        if current.year() != archive_date.year() || current.month() != archive_date.month(){
            return Err(Error::from("只允许修改本月的流水，历史流水已归档，不允许操作"));
        }
        // 判断要删除的这条流水明细关联的流水号下面，是否只有一条一条记录，如果只有一条，则直接级联删这个流水号，不允许空壳流水数据
        let general_journal_count = CONTEXT.financial_rbatis.fetch_count_by_wrapper::<GeneralJournal>(CONTEXT.financial_rbatis.new_wrapper().eq(GeneralJournal::journal_id(), journal_exist.id)).await?;
        if general_journal_count > 1  {
            // 把原来的流水金额 减去 要修改的流水明细金额
            let mut income = journal_exist.income.unwrap();
            let mut outlay = journal_exist.outlay.unwrap();
            let last_version_flag = general_journal_exist.flag.unwrap();
            let last_version_amount = general_journal_exist.amount.unwrap();
            if "1" == last_version_flag{
                income = income.sub(last_version_amount);
            } else if "2" == last_version_flag{
                outlay = outlay.sub(last_version_amount);
            } else {
                return Err(Error::from(("未知的收支类型!",util::CODE_FAIL)));
            }
            // 重新计算一下流水的总金额
            // 把最终的收支进行一次汇总
            let mut total = Decimal::from(0);
            total = total.add(income.clone());
            total = total.add(outlay.clone());

            // 删除流水明细
            let mut tx = CONTEXT.financial_rbatis.acquire_begin().await.unwrap();
            let delete_general_journal_result =  GeneralJournalMapper::delete_general_journal(&mut tx.as_executor(), id).await;
            if delete_general_journal_result.is_err() {
                error!("在删除流水明细时，发生异常:{}",delete_general_journal_result.unwrap_err());
                tx.rollback();
                return Err(Error::from("删除流水明细失败"));
            }
            // 修改流水
            let journal = Journal{
                id:journal_exist.id,
                monetary_id: None,
                income: Some(income),
                outlay: Some(outlay),
                means_id: None,
                abstract_id: None,
                total: Some(total),
                remarks: None,
                archive_date: None,
                organize: journal_exist.organize,
                source:Some(user_info.account.clone()),
                create_time:None,
                update_time: Some(rbatis::DateTimeNative::now())
            };

            // 修改流水记录
            let edit_journal_result = JournalMapper::update_journal(&mut tx.as_executor(), &journal).await;
            if edit_journal_result.is_err() {
                error!("在修改id={}的流水时，发生异常:{}",journal_exist.id.unwrap(),edit_journal_result.unwrap_err());
                tx.rollback();
                return Err(Error::from("流水修改失败"));
            }
            // 所有的写入都成功，最后正式提交
            tx.commit().await;
        }else{
            // 如果把这条流水明细删了后，该流水就是空壳了，所以应该干净直接的删流水
            let write_result = CONTEXT.financial_rbatis.remove_by_wrapper::<Journal>(journal_where).await;
            if write_result.is_err(){
                error!("删除流水时，发生异常:{}",write_result.unwrap_err());
                return Err(Error::from("删除流水失败!"));
            }
        }
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX030")).await;
        Ok(1)
    }

    /// 货币列表
    pub async fn get_monetary_list(&self) -> Result<Vec<MonetaryVO>> {
        let query_wrap_result = CONTEXT.financial_rbatis.fetch_list().await;
        if query_wrap_result.is_err() {
            error!("在查询货币列表时，发生异常:{}",query_wrap_result.unwrap_err());
            return Err(Error::from(("货币列表查询失败",util::CODE_FAIL)));
        }
        let query_result: Vec<Monetary> = query_wrap_result.unwrap();
        if query_result.is_empty() {
            return Err(Error::from(("货币数据不存在",util::NOT_EXIST)));
        }
        let mut result:Vec<MonetaryVO> = Vec::new();
        for item in query_result {
            result.push(MonetaryVO::from(item))
        }
        return Ok(result);
    }

    /// 摘要列表
    pub async fn get_abstracts_list(&self) -> Result<Vec<AbstractsVO>> {
        let query_wrap_result = CONTEXT.financial_rbatis.fetch_list().await;
        if query_wrap_result.is_err() {
            error!("在查询摘要时，发生异常:{}",query_wrap_result.unwrap_err());
            return Err(Error::from(("摘要查询失败",util::CODE_FAIL)));
        }
        let query_result: Vec<Abstracts> = query_wrap_result.unwrap();
        if query_result.is_empty() {
            return Err(Error::from(("摘要数据不存在",util::NOT_EXIST)));
        }
        let mut result:Vec<AbstractsVO> = Vec::new();
        for item in query_result {
            result.push(AbstractsVO::from(item))
        }
        return Ok(result);
    }

    /// 收支方式列表
    pub async fn get_payment_means_list(&self) -> Result<Vec<PaymentMeansVO>> {
        let query_wrap_result = CONTEXT.financial_rbatis.fetch_list().await;
        if query_wrap_result.is_err() {
            error!("在查询收支方式时，发生异常:{}",query_wrap_result.unwrap_err());
            return Err(Error::from(("收支方式查询失败",util::CODE_FAIL)));
        }
        let query_result : Vec<PaymentMeans> = query_wrap_result.unwrap();
        if query_result.is_empty() {
            return Err(Error::from(("摘要数据不存在",util::NOT_EXIST)));
        }
        let mut result:Vec<PaymentMeansVO> = Vec::new();
        for item in query_result {
            result.push(PaymentMeansVO::from(item))
        }
        return Ok(result);
    }

    /// 流水分页
    pub async fn journal_page(&self, req: &HttpRequest, param: &JournalPageDTO) -> Result<Page<JournalVO>>  {
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

        let count_result = JournalMapper::select_count(&mut CONTEXT.financial_rbatis.as_executor(), &arg,&extend).await;
        if count_result.is_err(){
            error!("在流水分页统计时，发生异常:{}",count_result.unwrap_err());
            return Err(Error::from("流水分页查询异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from(("未查询到符合条件的数据",util::NOT_EXIST)));
        }
        let mut result = Page::<JournalVO>::page_query( total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no-1)*result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = JournalMapper::select_page(&mut CONTEXT.financial_rbatis.as_executor(), &arg,&extend).await;
        if page_result.is_err() {
            error!("在流水分页获取页面数据时，发生异常:{}",page_result.unwrap_err());
            return Err(Error::from("流水分页查询异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

    /// 流水明细
    pub async fn general_journal_detail(&self, req: &HttpRequest, param: &JournalPageDTO)-> Result<Vec<GeneralJournalVO>>  {
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

        let wrap_result = GeneralJournalMapper::select_detail(&mut CONTEXT.financial_rbatis.as_executor(), &arg,&extend).await;
        if wrap_result.is_err(){
            error!("在查询流水明细时，发生异常:{}",wrap_result.unwrap_err());
            return Err(Error::from("流水明细查询异常"));
        }
        let result = wrap_result.unwrap().unwrap();
        return Ok(result)
    }

    /// 导出流水
    pub async fn journal_excel(&self, req: &HttpRequest, param: &JournalPageDTO) -> HttpResponse {
        let mut response = HttpResponse::Ok();
        let mut extend = ExtendPageDTO{
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time:param.begin_time,
            end_time:param.end_time
        };
        let user_info = JWTToken::extract_user_by_request(req).unwrap();
        let mut arg= param.clone();
        // 用户只能看到自己组织下的数据
        arg.organize = Some(user_info.organize);
        let count_result = JournalMapper::select_count(&mut CONTEXT.financial_rbatis.as_executor(), &arg,&extend).await;
        if count_result.is_err(){
            error!("在流水分页统计时，发生异常:{}",count_result.unwrap_err());
            response.status(StatusCode::INTERNAL_SERVER_ERROR);
            return response.finish()
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            response.status(StatusCode::NOT_FOUND);
            return response.finish()
        }
        let mut result = Page::<JournalVO>::page_query( total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no-1)*result.page_size);
        extend.page_size = Some(total_row);
        let page_result = JournalMapper::select_page(&mut CONTEXT.financial_rbatis.as_executor(), &arg,&extend).await;
        if page_result.is_err() {
            error!("在导出流水数据时，发生异常:{}",page_result.unwrap_err());
            response.status(StatusCode::INTERNAL_SERVER_ERROR);
            return response.finish()
        }
        let rows = page_result.unwrap().unwrap();
        let mut wb = Workbook::create_in_memory();
        let mut sheet = wb.create_sheet("流水报表");
        // 设置列宽
        sheet.add_column(Column { width: 12.0 });
        sheet.add_column(Column { width: 20.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;
            // 写入标题行
            sw.append_row(row!["流水号", "币种","收入","支出","总收支","收支方式","摘要","备注","归档日期","申报用户","申报时间", "修改时间"]);
            for item in rows {
                //item.id
                sw.append_row(row![item.id.unwrap().to_f64().unwrap(),item.monetary_name.unwrap(), item.income.unwrap().parse::<f64>().unwrap(), item.outlay.unwrap().parse::<f64>().unwrap(),item.total.unwrap().parse::<f64>().unwrap(),item.payment_means_name.unwrap(),item.abstracts_name.unwrap(),item.remarks.unwrap(),item.archive_date.unwrap(),item.source.unwrap(),item.create_time.unwrap(),item.update_time.unwrap()]);
            }
            Ok(())
        }).expect("write excel error!");
        // 这里面是直接返回流的
        let excel_stream = wb.close().expect("close excel error!");
        response.content_type("application/octet-stream;charset=UTF-8");
        response.insert_header((actix_web::http::header::CONTENT_DISPOSITION, "attachment;filename=journal.xlsx"));
        response.body(excel_stream.unwrap())
    }

    /// 导出流水明细
    pub async fn general_journal_excel(&self, req: &HttpRequest, param: &JournalPageDTO) -> HttpResponse {
        let mut response = HttpResponse::Ok();
        let mut extend = ExtendPageDTO{
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time:param.begin_time,
            end_time:param.end_time
        };
        let user_info = JWTToken::extract_user_by_request(req).unwrap();
        let mut arg= param.clone();
        // 用户只能看到自己组织下的数据
        arg.organize = Some(user_info.organize);
        let count_result = JournalMapper::select_count(&mut CONTEXT.financial_rbatis.as_executor(), &arg,&extend).await;
        if count_result.is_err(){
            error!("在流水分页统计时，发生异常:{}",count_result.unwrap_err());
            response.status(StatusCode::INTERNAL_SERVER_ERROR);
            return response.finish()
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            response.status(StatusCode::NOT_FOUND);
            return response.finish()
        }
        let mut result = Page::<JournalVO>::page_query( total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no-1)*result.page_size);
        extend.page_size = Some(total_row);
        let journal_result = JournalMapper::select_page(&mut CONTEXT.financial_rbatis.as_executor(), &arg,&extend).await;
        if journal_result.is_err() {
            error!("在导出流水数据时，发生异常:{}",journal_result.unwrap_err());
            response.status(StatusCode::INTERNAL_SERVER_ERROR);
            return response.finish()
        }
        let journal_rows = journal_result.unwrap().unwrap();
        // 联动查询明细
        let general_journal_result = GeneralJournalMapper::select_detail(&mut CONTEXT.financial_rbatis.as_executor(), &arg,&extend).await;
        if general_journal_result.is_err(){
            error!("在导出流水数据时，发生异常:{}",general_journal_result.unwrap_err());
            response.status(StatusCode::INTERNAL_SERVER_ERROR);
            return response.finish()
        }
        let general_journal_rows = general_journal_result.unwrap().unwrap();
        // 对数据按照流水号进行分组
        let mut map:HashMap<u64,Vec<GeneralJournalVO>> = HashMap::new();
        for item in general_journal_rows {
            let journal_id = item.journal_id.unwrap();
            if map.contains_key(&journal_id) {
                let mut list = map.get(&journal_id).unwrap().to_vec();
                list.push(item);
                map.insert(journal_id, list);
            }else {
                map.insert(journal_id, vec![item]);
            }
        }

        let mut wb = Workbook::create_in_memory();
        let mut sheet = wb.create_sheet("流水明细报表");
        // 设置列宽
        sheet.add_column(Column { width: 12.0 });
        sheet.add_column(Column { width: 20.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        // 第一行是表头
        let mut line_num:usize = 2;
        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;
            // 写入标题行
            sw.append_row(row!["流水号", "币种","收入","支出","总收支","收支方式","摘要","备注","归档日期","申报用户","申报时间", "修改时间","明细","收支类型","金额"]);
            for item in journal_rows {
                let journal_id = item.id.unwrap();
                if map.contains_key(&journal_id) {
                    let list = map.get(&journal_id).unwrap().to_vec();
                    let size = list.len();
                    if 1 != size {
                        // 不为1时，才发生合并
                        // 列,行；从1开始
                        let end_rows = line_num + size - 1;
                        for cell_num in 1..13 {
                            sw.merge_cells((cell_num,line_num),(cell_num,end_rows));
                        }
                    }
                    line_num = line_num + size;
                    // 写入数据
                    let mut general_journal_line = 1;
                    for general_journal in list {
                        let flag_name =  if "1" == general_journal.flag.unwrap(){"收入"}else { "支出" };
                        if 1 == general_journal_line {
                            sw.append_row(row![journal_id.to_f64().unwrap(),item.monetary_name.clone().unwrap(), item.income.clone().unwrap().parse::<f64>().unwrap(), item.outlay.clone().unwrap().parse::<f64>().unwrap(),item.total.clone().unwrap().parse::<f64>().unwrap(),item.payment_means_name.clone().unwrap(),item.abstracts_name.clone().unwrap(),item.remarks.clone().unwrap(),item.archive_date.clone().unwrap(),item.source.clone().unwrap(),item.create_time.clone().unwrap(),item.update_time.clone().unwrap(),general_journal.remarks.unwrap(),flag_name,general_journal.amount.unwrap().parse::<f64>().unwrap()]);
                        }else {
                            sw.append_row(row![(),(),(),(),(),(),(),(),(),(),(),(),general_journal.remarks.unwrap(),flag_name,general_journal.amount.unwrap().parse::<f64>().unwrap()]);
                        }
                        general_journal_line = general_journal_line + 1;
                    }
                }else{
                    // 没有明细，直接判断为脏数据，因为业务不可能有空壳流水
                }
            }
            Ok(())
        }).expect("write excel error!");
        // 这里面是直接返回流的
        let excel_stream = wb.close().expect("close excel error!");
        response.content_type("application/octet-stream;charset=UTF-8");
        response.insert_header((actix_web::http::header::CONTENT_DISPOSITION, "attachment;filename=general_journal.xlsx"));
        response.body(excel_stream.unwrap())
    }

    /// 流水汇总分页
    pub async fn journal_collect_page(&self, req: &HttpRequest, param: &JournalPageDTO) -> Result<Page<JournalVO>>  {
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

        let count_result = JournalMapper::select_day_count(&mut CONTEXT.financial_rbatis.as_executor(), &arg,&extend).await;
        if count_result.is_err(){
            error!("在流水汇总分页统计时，发生异常:{}",count_result.unwrap_err());
            return Err(Error::from("流水分页查询异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from(("未查询到符合条件的数据",util::NOT_EXIST)));
        }
        let mut result = Page::<JournalVO>::page_query( total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no-1)*result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = JournalMapper::select_day_page(&mut CONTEXT.financial_rbatis.as_executor(), &arg,&extend).await;
        if page_result.is_err() {
            error!("在流水汇总分页获取页面数据时，发生异常:{}",page_result.unwrap_err());
            return Err(Error::from("流水汇总分页查询异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

    /// 导出流水明细汇总
    pub async fn journal_collect_excel(&self, req: &HttpRequest, param: &JournalPageDTO) -> HttpResponse {
        let mut response = HttpResponse::Ok();
        let mut extend = ExtendPageDTO{
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time:param.begin_time,
            end_time:param.end_time
        };
        let user_info = JWTToken::extract_user_by_request(req).unwrap();
        let mut arg= param.clone();
        // 用户只能看到自己组织下的数据
        arg.organize = Some(user_info.organize);
        let count_result = JournalMapper::select_day_count(&mut CONTEXT.financial_rbatis.as_executor(), &arg,&extend).await;
        if count_result.is_err(){
            error!("在流水汇总分页统计时，发生异常:{}",count_result.unwrap_err());
            response.status(StatusCode::INTERNAL_SERVER_ERROR);
            return response.finish()
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            response.status(StatusCode::NOT_FOUND);
            return response.finish()
        }
        let mut result = Page::<JournalVO>::page_query( total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no-1)*result.page_size);
        extend.page_size = Some(total_row);
        let journal_result = JournalMapper::select_day_page(&mut CONTEXT.financial_rbatis.as_executor(), &arg,&extend).await;
        if journal_result.is_err() {
            error!("在导出流水汇总数据时，发生异常:{}",journal_result.unwrap_err());
            response.status(StatusCode::INTERNAL_SERVER_ERROR);
            return response.finish()
        }
        let journal_rows = journal_result.unwrap().unwrap();
        // 联动查询明细
        let general_journal_result = GeneralJournalMapper::select_day_collect(&mut CONTEXT.financial_rbatis.as_executor(), &arg,&extend).await;
        if general_journal_result.is_err(){
            error!("在导出流水汇总数据时，发生异常:{}",general_journal_result.unwrap_err());
            response.status(StatusCode::INTERNAL_SERVER_ERROR);
            return response.finish()
        }
        let general_journal_rows = general_journal_result.unwrap().unwrap();
        // 对数据按照流水号进行分组
        let mut map:HashMap<String,Vec<GeneralJournalCollectVO>> = HashMap::new();
        for item in general_journal_rows {
            let archive_date = item.archive_date.clone().unwrap();
            if map.contains_key(&archive_date) {
                let mut list = map.get(&archive_date).unwrap().to_vec();
                list.push(item);
                map.insert(archive_date, list);
            }else {
                map.insert(archive_date, vec![item]);
            }
        }

        let mut wb = Workbook::create_in_memory();
        let mut sheet = wb.create_sheet("流水汇总报表");
        // 设置列宽
        sheet.add_column(Column { width: 12.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        // 第一行是表头
        let mut line_num:usize = 2;
        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;
            // 写入标题行
            sw.append_row(row!["归档日期","支出","收入","总收支","明细","收支类型","金额"]);
            for item in journal_rows {
                let archive_date = item.archive_date.unwrap();
                //let archive_date = archive_date_str.as_str();
                if map.contains_key(&archive_date) {
                    let list = map.get(&archive_date).unwrap().to_vec();
                    let size = list.len();
                    if 1 != size {
                        // 不为1时，才发生合并
                        // 列,行；从1开始
                        let end_rows = line_num + size - 1;
                        for cell_num in 1..5 {
                            sw.merge_cells((cell_num,line_num),(cell_num,end_rows));
                        }
                    }
                    line_num = line_num + size;
                    // 写入数据
                    let mut general_journal_line = 1;
                    for general_journal in list {
                        let flag_name =  if "1" == general_journal.flag.unwrap(){"收入"}else { "支出" };
                        if 1 == general_journal_line {
                            sw.append_row(row![general_journal.archive_date.unwrap(), item.income.clone().unwrap().parse::<f64>().unwrap(), item.outlay.clone().unwrap().parse::<f64>().unwrap(),item.total.clone().unwrap().parse::<f64>().unwrap(),general_journal.remarks.unwrap(),flag_name,general_journal.amount.unwrap().parse::<f64>().unwrap()]);
                        }else {
                            sw.append_row(row![(),(),(),(),general_journal.remarks.unwrap(),flag_name,general_journal.amount.unwrap().parse::<f64>().unwrap()]);
                        }
                        general_journal_line = general_journal_line + 1;
                    }
                }else{
                    // 没有明细，直接判断为脏数据，因为业务不可能有空壳流水
                }
            }
            Ok(())
        }).expect("write excel error!");
        // 这里面是直接返回流的
        let excel_stream = wb.close().expect("close excel error!");
        response.content_type("application/octet-stream;charset=UTF-8");
        response.insert_header((actix_web::http::header::CONTENT_DISPOSITION, "attachment;filename=journal_collect.xlsx"));
        response.body(excel_stream.unwrap())
    }

    /// 计算收支增长率
    pub async fn compute_account_growth_rate(&self, req: &HttpRequest,month:&String) ->Result<HashMap<&str,Decimal>> {
        let user_month_wrap = DateNative::from_str((format!("{}-01",month.as_str())).as_str());
        if user_month_wrap.is_err() {
            return Err(Error::from(("统计月份不能为空!",util::NOT_PARAMETER)));
        }
        let user_month = user_month_wrap.unwrap();
        // 判断是否为当前月
        let current_month = DateNative::now();
        // 总天数，计算日均用
        let days = if current_month.year() == user_month.year() && current_month.month() == user_month.month(){
            // 当前月只计算 已经过去的天数
            current_month.day()
        }else {
            // 当月所有的天数
            DateUtils::get_current_month_days(user_month.year(),user_month.month())
        };
        // 得到上一个月的时间
        let last_month = DateUtils::month_compute(&user_month,-1);
        // 得到去年同期这个月的时间
        let last_year = DateUtils::month_compute(&user_month,-12);
        let user_info = JWTToken::extract_user_by_request(req).unwrap();
        // 本月（用户请求的月份）的统计
        let _current_month_wrap = JournalMapper::total_balance(&mut CONTEXT.financial_rbatis.as_executor(), &user_info.organize,&user_month).await;
        if _current_month_wrap.is_err(){
            error!("在统计指定月份的收支数据时，发生异常:{}",_current_month_wrap.unwrap_err());
            return Err(Error::from("收支增长率计算异常"));
        }
        let _current_month_row = _current_month_wrap.unwrap().unwrap();
        let mut _current_month_total = Decimal::from(0);
        if _current_month_row.total.is_some() {
            _current_month_total = _current_month_total.add(_current_month_row.total.unwrap());
        }
        // 上月的统计
        let _last_month_wrap = JournalMapper::total_balance(&mut CONTEXT.financial_rbatis.as_executor(), &user_info.organize,&last_month).await;
        if _last_month_wrap.is_err(){
            error!("在统计指定月份的收支数据时，发生异常:{}",_last_month_wrap.unwrap_err());
            return Err(Error::from("收支增长率计算异常"));
        }
        let _last_month_row = _last_month_wrap.unwrap().unwrap();
        let mut _last_month_total = Decimal::from(0);
        if _last_month_row.total.is_some() {
            _last_month_total = _last_month_total.add(_last_month_row.total.unwrap());
        }
        // 去年同期
        let _last_year_wrap = JournalMapper::total_balance(&mut CONTEXT.financial_rbatis.as_executor(), &user_info.organize,&last_year).await;
        if _last_year_wrap.is_err(){
            error!("在统计指定月份的收支数据时，发生异常:{}",_last_year_wrap.unwrap_err());
            return Err(Error::from("收支增长率计算异常"));
        }
        let _last_year_row = _last_year_wrap.unwrap().unwrap();
        let mut _last_year_total = Decimal::from(0);
        if _last_year_row.total.is_some() {
            _last_year_total = _last_year_total.add(_last_year_row.total.unwrap());
        }
        // 计算本月日均
        let mut current_avg_total = _current_month_total.div(Decimal::from(days));
        // 计算环比 （本月的值-上月的值）÷上月的值(如果上月值为空，不计算)
        let mut m2m = Decimal::from(0);
        if !_last_month_total.is_zero() {
            m2m = (_current_month_total.sub(_last_month_total)).div(_last_month_total);
            m2m = m2m.mul(Decimal::from(100));
        }
        // 计算同比 （本年的值-去年同期的值）÷去年同期的值(如果同期值为空，不计算)
        let mut y2y = Decimal::from(0);
        if !_last_year_total.is_zero() {
            y2y = (_current_month_total.sub(_last_year_total)).div(_last_year_total);
            y2y = y2y.mul(Decimal::from(100));
        }
        let mut result:HashMap<&str,Decimal> = HashMap::new();
        result.insert("account",_current_month_total.round_dp_with_strategy(2,RoundingStrategy::AwayFromZero));
        result.insert("avg",current_avg_total.round_dp_with_strategy(2,RoundingStrategy::AwayFromZero));
        result.insert("m2m",m2m.round_dp_with_strategy(2,RoundingStrategy::AwayFromZero));
        result.insert("y2y",y2y.round_dp_with_strategy(2,RoundingStrategy::AwayFromZero));
        Ok(result)
    }

    /// 计算指定月份的收入比重
    pub async fn compute_income_percentage(&self, req: &HttpRequest,month:&String) ->Result<HashMap<&str,Decimal>> {
        let user_info = JWTToken::extract_user_by_request(req).unwrap();
        let user_month_wrap = DateNative::from_str(month.as_str());
        if user_month_wrap.is_err() {
            return Err(Error::from(("统计月份不能为空!", util::NOT_PARAMETER)));
        }
        let user_month = user_month_wrap.unwrap();
        // 本月（用户请求的月份）的统计
        let _current_month_wrap = JournalMapper::total_balance(&mut CONTEXT.financial_rbatis.as_executor(), &user_info.organize,&user_month).await;
        if _current_month_wrap.is_err(){
            error!("在统计指定月份的收支数据时，发生异常:{}",_current_month_wrap.unwrap_err());
            return Err(Error::from("收入比重计算异常"));
        }
        let _current_month_row = _current_month_wrap.unwrap().unwrap();

        let income = _current_month_row.income.unwrap_or(Decimal::zero());
        let total = _current_month_row.total.unwrap_or(Decimal::zero());
        let mut result:HashMap<&str,Decimal> = HashMap::new();
        if total.is_zero() {
            result.insert("percentage",total);
        }else {
            result.insert("percentage",income.div(total).round_dp_with_strategy(2,RoundingStrategy::AwayFromZero));
        }
        result.insert("account",total.round_dp_with_strategy(2,RoundingStrategy::AwayFromZero));
        Ok(result)
    }

    /// 计算指定月份中各摘要的排名
    pub async fn order_month_journal(&self, req: &HttpRequest,month:&String) ->Result<Vec<JournalVO>> {
        let user_month_wrap = DateNative::from_str(month.as_str());
        if user_month_wrap.is_err() {
            return Err(Error::from(("统计月份不能为空!", util::NOT_PARAMETER)));
        }
        let user_month = user_month_wrap.unwrap();
        // 按月查询统计账单并排序
        let user_info = JWTToken::extract_user_by_request(req).unwrap();
        let bill_wrap = JournalMapper::bill_rank(&mut CONTEXT.financial_rbatis.as_executor(), &user_info.organize,&user_month).await;
        if bill_wrap.is_err(){
            error!("在统计指定月份中各摘要的排名时，发生异常:{}",bill_wrap.unwrap_err());
            return Err(Error::from("计算指定月份中各摘要的排名异常"));
        }
        let rows = bill_wrap.unwrap();
        return Ok(rows.unwrap());
    }

    /// 计算近6个月的财务流水
    pub async fn compute_pre6_journal(&self, req: &HttpRequest,month:&String) ->Result<rbson::Array> {
        // 按月查询统计账单并排序
        let user_info = JWTToken::extract_user_by_request(req).unwrap();
        let query_sql = format!("call count_pre6_journal({}, '{}')", &user_info.organize,month);
        let param:Vec<Bson> = Vec::new();
        let compute_result_warp = CONTEXT.financial_rbatis.fetch(query_sql.as_str(), param).await;
        if compute_result_warp.is_err(){
            error!("在统计近6个月的财务流水时，发生异常:{}",compute_result_warp.unwrap_err());
            return Err(Error::from("统计近6个月的财务流水异常"));
        }
        let rows:rbson::Array = compute_result_warp.unwrap();
        return Ok(rows);
    }

}