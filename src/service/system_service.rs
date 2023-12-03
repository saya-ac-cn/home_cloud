use crate::entity::dto::db_dump_log::DbDumpLogPageDTO;
use crate::entity::dto::log::LogPageDTO;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::dto::plan::{PlanDTO, PlanPageDTO};
use crate::entity::dto::plan_archive::{PlanArchiveDTO, PlanArchivePageDTO};
use crate::entity::dto::sign_in::SignInDTO;
use crate::entity::dto::user::{UserDTO, UserPageDTO};
use crate::dao::log_mapper::LogMapper;
use crate::dao::user_mapper::UserMapper;
use crate::entity::table::{Pictures, Plan, PlanArchive, User};
use crate::entity::vo::db_dump_log::DbDumpLogVO;
use crate::entity::vo::log::LogVO;
use crate::entity::vo::log_type::LogTypeVO;
use crate::entity::vo::plan::PlanVO;
use crate::entity::vo::plan_archive::PlanArchiveVO;
use crate::entity::vo::sign_in::SignInVO;
use crate::entity::vo::user::{UserOwnOrganizeVO, UserVO};
use crate::config::CONTEXT;
use crate::util::date_time::{DateTimeUtil, DateUtils};
use crate::util::error::Error;
use crate::util::error::Result;
use crate::util::password_encoder_util::PasswordEncoder;
use crate::util::Page;
use actix_http::StatusCode;
use actix_web::{HttpRequest, HttpResponse};
use chrono::Datelike;
use log::error;
use std::collections::HashMap;
use std::time::Duration;
extern crate simple_excel_writer as excel;
use crate::dao::db_dump_log_mapper::DbDumpLogMapper;
use crate::dao::log_type_mapper::LogTypeMapper;
use crate::dao::plan_archive_mapper::PlanArchiveMapper;
use crate::dao::plan_mapper::PlanMapper;
use crate::entity::vo::total_pre_6_month::TotalPre6MonthVO;
use crate::entity::vo::total_table::TotalTable;
use crate::config::user_context::UserContext;
use crate::util::ip_util::IpUtils;
use crate::util::scheduler::Scheduler;
use crate::util::token_util::TokenUtils;
use crate::{business_rbatis_pool, primary_rbatis_pool, util};
use excel::*;
use rbs::to_value;
use serde_json::{json, Map, Value};

/// 系统服务
pub struct SystemService {}

impl SystemService {
    /// 颁发token
    pub async fn token(&self) -> Result<String> {
        let token = TokenUtils::create_token().await;
        return Ok(token);
    }

    /// 登录
    pub async fn login(&self, req: &HttpRequest, arg: &SignInDTO) -> Result<SignInVO> {
        if arg.account.is_none()
            || arg.account.as_ref().unwrap().is_empty()
            || arg.password.is_none()
            || arg.password.as_ref().unwrap().is_empty()
        {
            return Err(Error::from((
                "账号和密码不能为空!",
                util::NOT_PARAMETER_CODE,
            )));
        }
        let query_user_wrap =
            User::select_by_account(primary_rbatis_pool!(), &arg.account.clone().unwrap()).await;
        if query_user_wrap.is_err() {
            error!("查询用户异常：{}", query_user_wrap.unwrap_err());
            return Err(Error::from("查询用户失败!"));
        }
        let user_wrap = query_user_wrap.unwrap().into_iter().next();
        let user = user_wrap.ok_or_else(|| {
            Error::from((
                format!("账号:{} 不存在!", &arg.account.clone().unwrap()),
                util::NOT_EXIST_CODE,
            ))
        })?;
        // 判断用户是否被锁定
        if user.state.eq(&Some(0)) {
            return Err(Error::from("账户被禁用!"));
        }
        if !PasswordEncoder::verify(
            user.password.as_ref().ok_or_else(|| {
                Error::from(("错误的用户数据，密码为空!", util::NOT_PARAMETER_CODE))
            })?,
            &arg.password.clone().unwrap(),
        ) {
            return Err(Error::from("账户或密码不正确!"));
        }
        // 生成用户token并返回
        let sign_in_vo = self
            .generate_token(req, &user, &arg.platform.clone().unwrap())
            .await?;

        // 通过上面生成的token，完整记录日志
        let context = UserContext::extract_token(&sign_in_vo.access_token).await?;
        LogMapper::record_log_by_context(primary_rbatis_pool!(), &context, String::from("OX001"))
            .await?;
        return Ok(sign_in_vo);
    }

    /// 登录
    pub async fn check_password(&self, req: &HttpRequest, arg: &SignInDTO) -> Result<SignInVO> {
        return Err(Error::from("方法未实现!"));
    }

    ///  生成用户token并返回
    pub async fn generate_token(
        &self,
        req: &HttpRequest,
        user: &User,
        platform: &str,
    ) -> Result<SignInVO> {
        // 默认 browser browser端，会话有效期1h
        let mut leeway: u64 = util::BROWSER_PLATFORM_TTL;
        if String::from("desktop").eq(platform) {
            // desktop 非浏览器端，会话有效期7*24h
            leeway = util::DESKTOP_PLATFORM_TTL;
        }

        let mut user = user.clone();
        // 密码脱敏
        user.password = None;
        // 如果服务前面没有代理，应该可以从请求peer_addr()中检索到它。
        // 否则，您可以检索请求connection_info()，并从中检索realip_remote_addr()
        let ip = if req.connection_info().realip_remote_addr().is_some() {
            req.connection_info()
                .realip_remote_addr()
                .unwrap()
                .parse()
                .unwrap()
        } else {
            req.peer_addr().unwrap().ip().to_string()
        };
        let city = if ip.eq("127.0.0.1") || ip.eq("localhost") {
            String::from("局域网地址")
        } else {
            IpUtils::city_location(&ip).await
        };
        // 准备壁纸
        let mut user_vo = UserVO::from(user.clone());
        let query_picture_wrap =
            Pictures::select_by_column(business_rbatis_pool!(), "id", &user_vo.background).await;
        if query_picture_wrap.is_err() {
            error!("查询壁纸异常：{}", query_picture_wrap.unwrap_err());
        } else {
            let picture = query_picture_wrap.unwrap().into_iter().next();
            user_vo.background_url = if picture.is_some() {
                picture.unwrap().web_url
            } else {
                None
            }
        }
        let mut sign_vo = SignInVO {
            user: Some(user_vo),
            access_token: String::new(),
            plan: None,
            log: None,
        };
        // 查询准备今日计划安排
        let query_today_plan_sql = "select concat(`title`,'[',date_format(`archive_time`,'%Y-%m-%d'),']') as item from `plan_archive` where `status` != 3 and `user` = ? and `archive_time` <= date_format(now(),'%Y-%m-%d 23:59:59')\n
            union all\n
            select concat(b.`title`,'[',date_format(b.`standard_time`,'%Y-%m-%d'),']') as item from `plan` b where b.`user` = ? and b.`standard_time` <= date_format(now(),'%Y-%m-%d 23:59:59')";
        let today_plan_result_warp = primary_rbatis_pool!()
            .query_decode::<Vec<HashMap<String, String>>>(
                query_today_plan_sql,
                vec![
                    to_value!(user.account.clone()),
                    to_value!(user.account.clone()),
                ],
            )
            .await;
        if today_plan_result_warp.is_ok() {
            sign_vo.plan = Some(today_plan_result_warp.unwrap());
        }

        // 查询最近的一次操作日志
        let log_warp =
            LogMapper::select_recently(primary_rbatis_pool!(), &user.account.clone().unwrap())
                .await;
        if log_warp.is_ok() {
            sign_vo.log = log_warp.unwrap();
        }

        let token_wrap = UserContext::create_token(&user.account.clone().unwrap()).await;
        if token_wrap.is_err() {
            error!("生成token时，发生异常:{}", token_wrap.unwrap_err());
            return Err(Error::from("登录异常"));
        }

        let token = token_wrap.unwrap();
        let check: Result<bool> = self
            .check_duplicate_login(&user.account.clone().unwrap())
            .await;
        if check.is_err() {
            error!("检查用户是否重复登录时，发生异常:{}", check.unwrap_err());
            return Err(Error::from("登录异常"));
        }

        // 生成用户的会话信息
        let context: UserContext = UserContext {
            account: user.account.unwrap_or_default(),
            name: user.name.clone().unwrap_or_default(),
            organize: user.organize_id.unwrap(),
            ip,
            city,
            leeway,
        };
        let serialized_user = serde_json::to_string(&context).unwrap();
        sign_vo.access_token = token.clone();
        // 写入到redis
        CONTEXT
            .redis_client
            .set_string_ex(
                &format!("{:}:{:}", &util::USER_CACHE_PREFIX, token),
                serialized_user.as_str(),
                Some(Duration::from_secs(leeway)),
            )
            .await;
        return Ok(sign_vo);
    }

    /// 检查用户是否已经登录过，登录过需要下线处理
    pub async fn check_duplicate_login(&self, account: &str) -> Result<bool> {
        let check = CONTEXT
            .redis_client
            .scan(&format!("{:}:{:}", &util::USER_CACHE_PREFIX, account))
            .await;
        if check.is_err() {
            return Err(check.unwrap_err());
        }
        let keys: Vec<String> = check.unwrap();
        CONTEXT.redis_client.batch_delete(&keys).await;
        Ok(true)
    }

    /// 登出后台
    pub async fn logout(&self, req: &HttpRequest) -> Result<UserContext> {
        let user_info = UserContext::extract_user_by_request(req).await;
        LogMapper::record_log_by_context(
            primary_rbatis_pool!(),
            &user_info.clone().unwrap(),
            String::from("OX002"),
        )
        .await?;
        Ok(user_info.unwrap())
    }

    /// 用户分页
    pub async fn user_page(&self, arg: &UserPageDTO) -> Result<Page<UserVO>> {
        let mut extend = ExtendPageDTO {
            page_no: arg.page_no,
            page_size: arg.page_size,
            begin_time: arg.begin_time.clone(),
            end_time: arg.end_time.clone(),
        };
        let count_result = UserMapper::select_count(primary_rbatis_pool!(), &arg, &extend).await;
        if count_result.is_err() {
            error!("在用户分页统计时，发生异常:{}", count_result.unwrap_err());
            return Err(Error::from("用户分页查询异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from((
                "未查询到符合条件的数据",
                util::NOT_EXIST_CODE,
            )));
        }
        let mut result = Page::<UserVO>::page_query(total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no - 1) * result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = UserMapper::select_page(primary_rbatis_pool!(), &arg, &extend).await;
        if page_result.is_err() {
            error!(
                "在用户分页获取页面数据时，发生异常:{}",
                page_result.unwrap_err()
            );
            return Err(Error::from("用户分页查询异常"));
        }
        let page_rows = page_result.unwrap();
        let mut list = vec![];
        for item in page_rows.unwrap() {
            list.push(UserVO::from(item));
        }
        result.records = Some(list);
        return Ok(result);
    }

    ///创建账号
    pub async fn user_add(&self, arg: &UserDTO) -> Result<u64> {
        TokenUtils::check_token(arg.token.clone())
            .await
            .ok_or_else(|| Error::from(util::TOKEN_ERROR_CODE))?;
        let check_flag = arg.account.is_none()
            || arg.account.as_ref().unwrap().is_empty()
            || arg.name.is_none()
            || arg.name.as_ref().unwrap().is_empty()
            || arg.email.is_none()
            || arg.email.as_ref().unwrap().is_empty()
            || arg.phone.is_none()
            || arg.phone.as_ref().unwrap().is_empty()
            || arg.organize_id.is_none();
        if check_flag {
            return Err(Error::from((
                "账号、姓名、手机号、邮箱以及所属组织不能为空!",
                util::NOT_PARAMETER_CODE,
            )));
        }

        let query_user_wrap =
            User::select_by_account(primary_rbatis_pool!(), &arg.account.clone().unwrap()).await;
        if query_user_wrap.is_err() {
            error!("查询用户异常：{}", query_user_wrap.unwrap_err());
            return Err(Error::from("查询用户失败!"));
        }
        let old_user = query_user_wrap.unwrap().into_iter().next();
        if old_user.is_some() {
            return Err(Error::from(format!(
                "账户:{}已存在!",
                arg.account.as_ref().unwrap()
            )));
        }
        let mut password = arg.password.clone().unwrap_or_default();
        if password.is_empty() {
            //默认密码
            password = "123456".to_string();
        }
        let user = User {
            account: arg.account.clone(),
            name: arg.name.clone(),
            password: PasswordEncoder::encode(&password).into(),
            sex: arg.sex.clone(),
            qq: arg.qq.clone(),
            email: arg.email.clone(),
            phone: arg.phone.clone(),
            birthday: arg.birthday.clone(),
            hometown: arg.hometown.clone(),
            autograph: arg.autograph.clone(),
            logo: arg.logo.clone(),
            background: arg.background,
            organize_id: arg.organize_id,
            state: 1.into(),
            create_time: DateTimeUtil::naive_date_time_to_str(
                &Some(DateUtils::now()),
                &util::FORMAT_Y_M_D_H_M_S,
            ),
            update_time: None,
        };
        let write_result = User::insert(primary_rbatis_pool!(), &user).await;
        if write_result.is_err() {
            error!("创建账号时，发生异常:{}", write_result.unwrap_err());
            return Err(Error::from("创建账号时，发生异常!"));
        }
        // 当前不允许创建用户操作
        // LogMapper::record_log(&CONTEXT.primary_rbatis,String::from(""));
        return Ok(write_result?.rows_affected);
    }

    /// 通过token获取用户信息
    pub async fn user_get_info_by_token(&self, req: &HttpRequest) -> Result<UserVO> {
        let user_info = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(Error::from(util::NOT_AUTHORIZE_CODE)))?;
        let query_user_wrap =
            User::select_by_account(primary_rbatis_pool!(), &user_info.account).await;
        if query_user_wrap.is_err() {
            error!("查询用户异常：{}", query_user_wrap.unwrap_err());
            return Err(Error::from("查询用户失败!"));
        }
        let user_warp = query_user_wrap.unwrap().into_iter().next();
        let user = user_warp.ok_or_else(|| {
            Error::from((
                format!("账号:{} 不存在!", &user_info.account),
                util::NOT_EXIST_CODE,
            ))
        })?;
        return Ok(UserVO::from(user));
    }

    /// 修改用户信息
    pub async fn user_edit(&self, req: &HttpRequest, arg: &UserDTO) -> Result<u64> {
        TokenUtils::check_token(arg.token.clone())
            .await
            .ok_or_else(|| Error::from(util::TOKEN_ERROR_CODE))?;
        if arg.account.is_none() || arg.account.as_ref().unwrap().is_empty() {
            return Err(Error::from((
                "账号account不能为空!",
                util::NOT_PARAMETER_CODE,
            )));
        }
        // 首先判断要修改的用户是否存在
        let query_user_wrap =
            User::select_by_account(primary_rbatis_pool!(), &arg.account.clone().unwrap()).await;
        if query_user_wrap.is_err() {
            error!("查询用户异常：{}", query_user_wrap.unwrap_err());
            return Err(Error::from("查询用户失败!"));
        }
        let user_warp = query_user_wrap.unwrap().into_iter().next();
        let user_exist = user_warp.ok_or_else(|| {
            Error::from((
                format!("账号:{} 不存在!", &arg.account.clone().unwrap()),
                util::NOT_EXIST_CODE,
            ))
        })?;

        let user_edit = User {
            account: user_exist.account,
            name: arg.name.clone(),
            password: if arg.password.is_some() {
                Some(PasswordEncoder::encode(arg.password.as_ref().unwrap()))
            } else {
                user_exist.password
            },
            sex: arg.sex.clone(),
            qq: arg.qq.clone(),
            email: arg.email.clone(),
            phone: arg.phone.clone(),
            birthday: arg.birthday.clone(),
            hometown: arg.hometown.clone(),
            autograph: arg.autograph.clone(),
            logo: arg.logo.clone(),
            background: arg.background,
            organize_id: arg.organize_id,
            state: arg.state,
            create_time: user_exist.create_time,
            update_time: DateTimeUtil::naive_date_time_to_str(
                &Some(DateUtils::now()),
                &util::FORMAT_Y_M_D_H_M_S,
            ),
        };
        let result = UserMapper::update_user(primary_rbatis_pool!(), &user_edit).await; //CONTEXT.primary_rbatis.update_by_column(User::account(),&user_edit).await?;
        if result.is_err() {
            error!(
                "在修改用户{}的信息时，发生异常:{}",
                arg.account.as_ref().unwrap(),
                result.unwrap_err()
            );
            return Err(Error::from(format!(
                "修改账户[{}]信息失败!",
                arg.account.as_ref().unwrap()
            )));
        }
        let context = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
        LogMapper::record_log_by_context(primary_rbatis_pool!(), &context, String::from("OX003"))
            .await?;
        Ok(result.unwrap().rows_affected)
    }

    /// 删除用户
    pub async fn user_remove(&self, account: &str) -> Result<u64> {
        if account.is_empty() {
            return Err(Error::from((
                "account 不能为空！",
                util::NOT_PARAMETER_CODE,
            )));
        }
        let r = User::delete_by_account(primary_rbatis_pool!(), account.clone()).await?;
        return Ok(r.rows_affected);
    }

    /// 用户详情
    pub async fn user_detail(&self, arg: &UserDTO) -> Result<UserVO> {
        let account = arg.account.clone().unwrap_or_default();
        let query_user_wrap =
            User::select_by_account(primary_rbatis_pool!(), &account.clone()).await;
        if query_user_wrap.is_err() {
            error!("查询用户异常：{}", query_user_wrap.unwrap_err());
            return Err(Error::from("查询用户失败!"));
        }
        let user_warp = query_user_wrap.unwrap().into_iter().next();
        let user = user_warp.ok_or_else(|| {
            Error::from((
                format!("账号:{} 不存在!", &account.clone()),
                util::NOT_EXIST_CODE,
            ))
        })?;
        let user_vo = UserVO::from(user);
        return Ok(user_vo);
    }

    /// 获取当前用户所在组织的用户列表
    pub async fn user_get_own_organize(&self, req: &HttpRequest) -> Result<Vec<UserOwnOrganizeVO>> {
        let context = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
        let query_result =
            UserMapper::select_own_organize_user(primary_rbatis_pool!(), &context.account).await;
        if query_result.is_err() {
            error!(
                "在查询用户所属组织下的用户列表时，发生异常:{}",
                query_result.unwrap_err()
            );
            return Err(Error::from(format!("查询我所属组织的用户列表异常")));
        }
        return Ok(query_result.unwrap().unwrap());
    }

    /// 修改用户密码
    pub async fn user_update_password(&self, req: &HttpRequest, arg: &UserDTO) -> Result<u64> {
        TokenUtils::check_token(arg.token.clone())
            .await
            .ok_or_else(|| Error::from(util::TOKEN_ERROR_CODE))?;
        if arg.password.is_none() || arg.password.as_ref().unwrap().is_empty() {
            return Err(Error::from(("密码不能为空!", util::NOT_PARAMETER_CODE)));
        }
        // 首先判断要修改的用户是否存在

        let query_user_wrap =
            User::select_by_account(primary_rbatis_pool!(), &arg.account.clone().unwrap()).await;
        if query_user_wrap.is_err() {
            error!("查询用户异常：{}", query_user_wrap.unwrap_err());
            return Err(Error::from("查询用户失败!"));
        }
        let user_warp = query_user_wrap.unwrap().into_iter().next();
        let user_exist = user_warp.ok_or_else(|| {
            Error::from((
                format!("账号:{} 不存在!", &arg.account.clone().unwrap()),
                util::NOT_EXIST_CODE,
            ))
        })?;

        let user_edit = User {
            account: user_exist.account,
            name: None,
            password: Some(PasswordEncoder::encode(arg.password.as_ref().unwrap())),
            sex: None,
            qq: None,
            email: None,
            phone: None,
            birthday: None,
            hometown: None,
            autograph: None,
            logo: None,
            background: None,
            organize_id: None,
            state: None,
            create_time: None,
            update_time: DateTimeUtil::naive_date_time_to_str(
                &Some(DateUtils::now()),
                &util::FORMAT_Y_M_D_H_M_S,
            ),
        };
        let result = UserMapper::update_user(primary_rbatis_pool!(), &user_edit).await; //CONTEXT.primary_rbatis.update_by_column(User::account(),&user_edit).await?;
        if result.is_err() {
            error!(
                "在修改用户{}的密码时，发生异常:{}",
                arg.account.as_ref().unwrap(),
                result.unwrap_err()
            );
            return Err(Error::from(format!(
                "修改账户[{}]密码失败!",
                arg.account.as_ref().unwrap()
            )));
        }
        let context = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
        LogMapper::record_log_by_context(primary_rbatis_pool!(), &context, String::from("OX004"))
            .await?;
        Ok(result.unwrap().rows_affected)
    }

    /// 日志类别列表
    pub async fn log_get_type(&self) -> Result<Vec<LogTypeVO>> {
        let query_result = LogTypeMapper::select_all(primary_rbatis_pool!()).await;
        if query_result.is_err() {
            error!(
                "在查询日志类型列表时，发生异常:{}",
                query_result.unwrap_err()
            );
            return Err(Error::from("查询日志类型列表异常"));
        }
        return Ok(query_result.unwrap().unwrap());
    }

    /// 日志分页
    pub async fn log_page(&self, req: &HttpRequest, param: &LogPageDTO) -> Result<Page<LogVO>> {
        let mut extend = ExtendPageDTO {
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time: param.begin_time.clone(),
            end_time: param.end_time.clone(),
        };
        let user_info = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
        let mut arg = param.clone();
        arg.organize = Some(user_info.organize);

        let count_result = LogMapper::select_count(primary_rbatis_pool!(), &arg, &extend).await;
        if count_result.is_err() {
            error!("在日志分页统计时，发生异常:{}", count_result.unwrap_err());
            return Err(Error::from("日志分页查询异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from((
                "未查询到符合条件的数据",
                util::NOT_EXIST_CODE,
            )));
        }
        let mut result = Page::<LogVO>::page_query(total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no - 1) * result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = LogMapper::select_page(primary_rbatis_pool!(), &arg, &extend).await;
        if page_result.is_err() {
            error!(
                "在日志分页获取页面数据时，发生异常:{}",
                page_result.unwrap_err()
            );
            return Err(Error::from("日志分页查询异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

    /// 导出日志分页
    pub async fn log_excel(&self, req: &HttpRequest, param: &LogPageDTO) -> HttpResponse {
        let mut response = HttpResponse::Ok();
        let mut extend = ExtendPageDTO {
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time: param.begin_time.clone(),
            end_time: param.end_time.clone(),
        };
        let user_info = UserContext::extract_user_by_request(req).await.unwrap();
        let mut arg = param.clone();
        arg.organize = Some(user_info.organize);

        let count_result = LogMapper::select_count(primary_rbatis_pool!(), &arg, &extend).await;
        if count_result.is_err() {
            error!("在日志分页统计时，发生异常:{}", count_result.unwrap_err());
            response.status(StatusCode::INTERNAL_SERVER_ERROR);
            return response.finish();
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            response.status(StatusCode::NOT_FOUND);
            return response.finish();
        }
        let result = Page::<LogVO>::page_query(total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no - 1) * result.page_size);
        extend.page_size = Some(total_row);
        let page_result = LogMapper::select_page(primary_rbatis_pool!(), &arg, &extend).await;
        if page_result.is_err() {
            error!(
                "在日志分页获取页面数据时，发生异常:{}",
                page_result.unwrap_err()
            );
            response.status(StatusCode::INTERNAL_SERVER_ERROR);
            return response.finish();
        }
        let rows = page_result.unwrap().unwrap();
        let mut wb = Workbook::create_in_memory();
        let mut sheet = wb.create_sheet("操作日志");
        // 设置列宽
        sheet.add_column(Column { width: 12.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        sheet.add_column(Column { width: 25.0 });
        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;
            // 写入标题行
            sw.append_row(row!["用户", "操作详情", "IP", "城市", "日期"]);
            for item in rows {
                sw.append_row(row![
                    item.user.unwrap(),
                    item.detail.unwrap(),
                    item.ip.unwrap(),
                    item.city.unwrap(),
                    item.date.unwrap()
                ]);
            }
            Ok(())
        })
        .expect("write excel error!");
        // 这里面是直接返回流的
        let excel_stream = wb.close().expect("close excel error!");
        response.content_type("application/octet-stream;charset=UTF-8");
        response.insert_header((
            actix_web::http::header::CONTENT_DISPOSITION,
            "attachment;filename=log.xlsx",
        ));
        response.body(excel_stream.unwrap())
    }

    /// 计算近6个月的活跃情况
    pub async fn compute_pre6_logs(&self, req: &HttpRequest, month: &String) -> Result<Value> {
        let user_info = UserContext::extract_user_by_request(req).await.unwrap();
        let user_month_wrap =
            chrono::NaiveDate::parse_from_str(month.as_str(), &util::FORMAT_Y_M_D);
        if user_month_wrap.is_err() {
            return Err(Error::from(("统计月份不能为空!", util::NOT_PARAMETER_CODE)));
        }
        let user_month = user_month_wrap.unwrap();
        // 查询用户指定的月份日志数量
        let query_current_month_log_sql = "select count(1) from `log` where `organize` = ? and  `date` like concat(date_format(?,'%Y-%m'),'%')";
        let current_month_log_result_warp = primary_rbatis_pool!()
            .query_decode::<u64>(
                query_current_month_log_sql,
                vec![to_value!(user_info.organize), to_value!(month.as_str())],
            )
            .await;
        let mut current_month_log: u64 = 0;
        if current_month_log_result_warp.is_ok() {
            current_month_log = current_month_log_result_warp.unwrap()
        } else {
            error!(
                "在查询指定月份的日志数据总数时，发生异常:{}",
                current_month_log_result_warp.unwrap_err()
            );
        }
        // 判断是否为当前月
        let current_month = DateUtils::now().date_naive();
        // 总天数，计算日均用
        let days = if current_month.year() == user_month.year()
            && current_month.month() == user_month.month()
        {
            // 当前月只计算 已经过去的天数
            current_month.day()
        } else {
            // 当月所有的天数
            DateUtils::get_current_month_days(user_month.year(), user_month.month())
        };
        let avg: u64 = current_month_log / (days as u64);

        let query_sql = format!("call count_pre6_logs({}, '{}')", &user_info.organize, month);
        let compute_result_warp = primary_rbatis_pool!()
            .query_decode::<Vec<TotalPre6MonthVO>>(query_sql.as_str(), vec![])
            .await;
        if compute_result_warp.is_err() {
            error!(
                "在统计近6个月的活跃情况时，发生异常:{}",
                compute_result_warp.unwrap_err()
            );
            return Err(Error::from("统计近6个月的活跃情况异常"));
        }
        let rows: Vec<TotalPre6MonthVO> = compute_result_warp.unwrap();
        let mut result: Map<String, Value> = Map::new();
        result.insert(String::from("avg"), json!(avg));
        result.insert(String::from("count"), json!(current_month_log));
        result.insert(String::from("log6"), json!(rows));
        return Ok(json!(result));
    }

    /// 统计各个表的数据体量
    pub async fn compute_object_rows(&self, req: &HttpRequest) -> Result<Value> {
        // 最终结果集的容器
        let mut rose_data: Vec<Value> = Vec::new();
        let mut word_cloud: Vec<Value> = Vec::new();
        let mut result: Map<String, Value> = Map::new();
        let user_info = UserContext::extract_user_by_request(req).await.unwrap();

        let query_notebook_sql = "select a.`name`, count(b.`id`) as value from `note_book` a left join `notes` b on a.`id` = b.`notebook_id` where a.`organize` = ? group by a.`id`";
        let query_notebook_result_warp = business_rbatis_pool!()
            .query_decode(query_notebook_sql, vec![to_value!(user_info.organize)])
            .await;
        if query_notebook_result_warp.is_ok() {
            let business_rows: Vec<TotalTable> = query_notebook_result_warp.unwrap();
            for item in business_rows {
                let mut current_data: Map<String, Value> = Map::new();
                current_data.insert(String::from("name"), json!(item.name));
                current_data.insert(String::from("value"), json!(item.value));
                word_cloud.push(json!(current_data));
            }
        } else {
            error!(
                "在分类别查询笔记簿时，发生异常:{}",
                query_notebook_result_warp.unwrap_err()
            );
        }
        result.insert(String::from("word_cloud"), json!(word_cloud));

        let query_primary_count_sql = format!(
            "select '计划' as `name` , count(1) as `value` from `plan` where `organize` = {};",
            &user_info.organize
        );
        let primary_count_result_warp = primary_rbatis_pool!()
            .query_decode(query_primary_count_sql.as_str(), vec![])
            .await;
        if primary_count_result_warp.is_err() {
            error!(
                "在统计主库的计划提醒时，发生异常:{}",
                primary_count_result_warp.unwrap_err()
            );
            let mut plan: Map<String, Value> = Map::new();
            plan.insert(String::from("name"), json!("计划"));
            plan.insert(String::from("value"), json!(0));
            rose_data.push(json!(plan));
        } else {
            let primary_rows: Vec<Value> = primary_count_result_warp.unwrap();
            for item in primary_rows {
                rose_data.push(item);
            }
        }
        let query_business_count_sql = format!("select '文件' as `name` , count(1) as `value` from `files` where `organize` = {}\n
                                                    union all\n
                                                    select '图片' as `name` , count(1) as `value` from `pictures` where `organize` = {}\n
                                                    union all\n
                                                    select '笔记簿' as `name` , count(1) as `value` from `note_book` where `organize` = {}\n
                                                    union all\n
                                                    select '笔记' as `name` , count(1) as `value` from `note_book` a inner join `notes` b on a.`id` = b.`notebook_id` where a.`organize` = {}\n
                                                    union all\n
                                                    select '动态' as `name` , count(1) as `value` from `news` where `organize` = {}", &user_info.organize,&user_info.organize,&user_info.organize,&user_info.organize,&user_info.organize);
        let business_count_result_warp = business_rbatis_pool!()
            .query_decode::<Vec<TotalTable>>(query_business_count_sql.as_str(), vec![])
            .await;

        //let mut business_rows:Vec<TotalTable> = vec![];

        if business_count_result_warp.is_err() {
            error!(
                "在统计业务库的各表总数据量时，发生异常:{}",
                business_count_result_warp.unwrap_err()
            );
            let mut files: Map<String, Value> = Map::new();
            files.insert(String::from("name"), json!("文件"));
            files.insert(String::from("value"), json!(0));
            rose_data.push(json!(files));
            let mut pictures: Map<String, Value> = Map::new();
            pictures.insert(String::from("name"), json!("图片"));
            pictures.insert(String::from("value"), json!(0));
            rose_data.push(json!(pictures));
            let mut notebook: Map<String, Value> = Map::new();
            notebook.insert(String::from("name"), json!("笔记簿"));
            notebook.insert(String::from("value"), json!(0));
            rose_data.push(json!(notebook));
            let mut notes: Map<String, Value> = Map::new();
            notes.insert(String::from("name"), json!("笔记"));
            notes.insert(String::from("value"), json!(0));
            rose_data.push(json!(notes));
            let mut news: Map<String, Value> = Map::new();
            news.insert(String::from("name"), json!("动态"));
            news.insert(String::from("value"), json!(0));
            rose_data.push(json!(news));
        } else {
            let business_rows: Vec<TotalTable> = business_count_result_warp.unwrap();
            for item in business_rows {
                let mut current_data: Map<String, Value> = Map::new();
                current_data.insert(String::from("name"), json!(item.name));
                current_data.insert(String::from("value"), json!(item.value));
                rose_data.push(json!(current_data));
            }
        }
        result.insert(String::from("rose_data"), json!(rose_data));
        return Ok(json!(result));
    }

    /// 创建提醒事项
    pub async fn add_plan(&self, req: &HttpRequest, arg: &PlanDTO) -> Result<u64> {
        TokenUtils::check_token(arg.token.clone())
            .await
            .ok_or_else(|| Error::from(util::TOKEN_ERROR_CODE))?;
        let check_flag = arg.standard_time.is_none()
            || arg.cycle.is_none()
            || arg.unit.is_none()
            || arg.content.is_none()
            || arg.content.as_ref().unwrap().is_empty();
        if check_flag {
            return Err(Error::from((
                "基准时间、重复执行周期、单位和内容不能为空!",
                util::NOT_PARAMETER_CODE,
            )));
        }
        let cycle = arg.cycle.unwrap();

        let standard_time_result = chrono::NaiveDateTime::parse_from_str(
            &arg.standard_time.clone().unwrap().as_str(),
            &util::FORMAT_Y_M_D_H_M_S,
        );
        if standard_time_result.is_err() {
            error!("格式化日期发生异常:{}", standard_time_result.unwrap_err());
            return Err(Error::from("创建提醒事项失败"));
        }
        let standard_time = standard_time_result.unwrap();

        // 计算下次执行时间
        let next_exec_time_op =
            DateUtils::plan_data_compute(&standard_time, arg.cycle.unwrap(), arg.unit.unwrap());
        if next_exec_time_op.is_none() {
            return Err(Error::from("无效的日期循环周期"));
        }
        let next_exec_time = next_exec_time_op.unwrap();
        // 生成定时cron表达式
        let cron_tab = DateUtils::data_time_to_cron(&standard_time);
        let user_info = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
        //   `cycle` 重复执行周期(1：一次性，2：天，3：周，4：月，5：年)',
        //   `unit` '重复执行周期单位',
        let plan = Plan {
            id: None,
            standard_time: arg.standard_time.clone(),
            cycle: Some(cycle),
            unit: arg.unit,
            title: arg.title.clone(),
            content: arg.content.clone(),
            // 一次性的任务，不用生成下一次执行时间
            next_exec_time: if 1 == cycle {
                None
            } else {
                DateTimeUtil::naive_date_time_to_str(
                    &Some(next_exec_time),
                    &util::FORMAT_Y_M_D_H_M_S,
                )
            },
            organize: Some(user_info.organize),
            user: Some(user_info.account.clone()),
            display: arg.display,
            create_time: DateTimeUtil::naive_date_time_to_str(
                &Some(DateUtils::now()),
                &util::FORMAT_Y_M_D_H_M_S,
            ),
            update_time: None,
        };

        let write_result = Plan::insert(primary_rbatis_pool!(), &plan).await;
        if write_result.is_err() {
            error!(
                "在保存计划提醒事项时，发生异常:{}",
                write_result.unwrap_err()
            );
            return Err(Error::from("保存计划提醒事项失败!"));
        }
        let result = write_result.unwrap();
        let plan_id = result.last_insert_id.as_u64().unwrap();
        Scheduler::add_plan(plan_id, cron_tab.as_str());
        LogMapper::record_log_by_context(primary_rbatis_pool!(), &user_info, String::from("OX022"))
            .await?;
        return Ok(result.rows_affected);
    }

    /// 修改提醒事项
    pub async fn edit_plan(&self, req: &HttpRequest, arg: &PlanDTO) -> Result<u64> {
        TokenUtils::check_token(arg.token.clone())
            .await
            .ok_or_else(|| Error::from(util::TOKEN_ERROR_CODE))?;
        let check_flag = arg.id.is_none()
            || arg.standard_time.is_none()
            || arg.cycle.is_none()
            || arg.unit.is_none()
            || arg.content.is_none()
            || arg.content.as_ref().unwrap().is_empty();
        if check_flag {
            return Err(Error::from((
                "基准时间、重复执行周期、单位和内容不能为空!",
                util::NOT_PARAMETER_CODE,
            )));
        }
        let user_info = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;

        let query_plan_wrap = Plan::select_by_id(primary_rbatis_pool!(), &arg.id.unwrap()).await;
        if query_plan_wrap.is_err() {
            error!("查询提醒事项异常：{}", query_plan_wrap.unwrap_err());
            return Err(Error::from("查询提醒事项失败!"));
        }
        let plan_warp = query_plan_wrap.unwrap().into_iter().next();
        let plan_exist = plan_warp.ok_or_else(|| {
            Error::from((
                format!("id={} 的提醒事项不存在!", arg.id.unwrap()),
                util::NOT_EXIST_CODE,
            ))
        })?;
        let cycle = arg.cycle.unwrap();

        let standard_time_result = chrono::NaiveDateTime::parse_from_str(
            &arg.standard_time.clone().unwrap().as_str(),
            &util::FORMAT_Y_M_D_H_M_S,
        );
        if standard_time_result.is_err() {
            error!("格式化日期发生异常:{}", standard_time_result.unwrap_err());
            return Err(Error::from("创建提醒事项失败"));
        }
        let standard_time = standard_time_result.unwrap();

        // 计算下次执行时间
        let next_exec_time_op =
            DateUtils::plan_data_compute(&standard_time, arg.cycle.unwrap(), arg.unit.unwrap());
        if next_exec_time_op.is_none() {
            return Err(Error::from("无效的日期循环周期"));
        }
        let next_exec_time = next_exec_time_op.unwrap();
        // 生成定时cron表达式
        let cron_tab = DateUtils::data_time_to_cron(&standard_time);
        let plan = Plan {
            id: plan_exist.id,
            standard_time: arg.standard_time.clone(),
            cycle: Some(cycle),
            unit: arg.unit,
            title: arg.title.clone(),
            content: arg.content.clone(),
            // 一次性的任务，不用生成下一次执行时间
            next_exec_time: if 1 == cycle {
                None
            } else {
                DateTimeUtil::naive_date_time_to_str(
                    &Some(next_exec_time),
                    &util::FORMAT_Y_M_D_H_M_S,
                )
            },
            organize: plan_exist.organize,
            user: Some(user_info.account.clone()),
            display: arg.display,
            create_time: None,
            update_time: DateTimeUtil::naive_date_time_to_str(
                &Some(DateUtils::now()),
                &util::FORMAT_Y_M_D_H_M_S,
            ),
        };
        let result = PlanMapper::update_plan(primary_rbatis_pool!(), &plan).await;
        if result.is_err() {
            error!(
                "在修改id={}的提醒事项时，发生异常:{}",
                arg.id.as_ref().unwrap(),
                result.unwrap_err()
            );
            return Err(Error::from("提醒事项修改失败"));
        }
        Scheduler::remove(plan_exist.id.unwrap()).await;
        Scheduler::add_plan(plan_exist.id.unwrap(), cron_tab.as_str()).await;
        LogMapper::record_log_by_context(primary_rbatis_pool!(), &user_info, String::from("OX023"))
            .await?;
        return Ok(result?.rows_affected);
    }

    /// 删除提醒事项
    pub async fn delete_plan(&self, req: &HttpRequest, id: &u64) -> Result<u64> {
        let user_info = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
        // 只能删除自己组织机构下的数据
        let write_result =
            Plan::delete_by_id_organize(primary_rbatis_pool!(), id, &user_info.organize).await;
        if write_result.is_err() {
            error!("删除提醒事项时，发生异常:{}", write_result.unwrap_err());
            return Err(Error::from("删除提醒事项失败!"));
        }
        Scheduler::remove(*id).await;
        LogMapper::record_log_by_context(primary_rbatis_pool!(), &user_info, String::from("OX024"))
            .await?;
        return Ok(write_result?.rows_affected);
    }

    /// 分页查询当前活跃的计划提醒(plan)
    pub async fn plan_page(&self, req: &HttpRequest, param: &PlanPageDTO) -> Result<Page<PlanVO>> {
        let mut extend = ExtendPageDTO {
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time: param.begin_time.clone(),
            end_time: param.end_time.clone(),
        };
        let user_info = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
        let mut arg = param.clone();
        // 用户只能看到自己组织下的数据
        arg.organize = Some(user_info.organize);

        let count_result = PlanMapper::select_count(primary_rbatis_pool!(), &arg, &extend).await;
        if count_result.is_err() {
            error!(
                "在计划提醒分页统计时，发生异常:{}",
                count_result.unwrap_err()
            );
            return Err(Error::from("计划提醒分页查询异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from((
                "未查询到符合条件的数据",
                util::NOT_EXIST_CODE,
            )));
        }
        let mut result = Page::<PlanVO>::page_query(total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no - 1) * result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = PlanMapper::select_page(primary_rbatis_pool!(), &arg, &extend).await;
        if page_result.is_err() {
            error!(
                "在计划提醒分页获取页面数据时，发生异常:{}",
                page_result.unwrap_err()
            );
            return Err(Error::from("计划提醒分页查询异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

    /// 提前完成计划提醒
    pub async fn advance_finish_plan(&self, req: &HttpRequest, arg: &PlanDTO) -> Result<u64> {
        TokenUtils::check_token(arg.token.clone())
            .await
            .ok_or_else(|| Error::from(util::TOKEN_ERROR_CODE))?;
        let id = arg.id.unwrap();
        let user_info = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
        let query_plan_wrap = Plan::select_by_id(primary_rbatis_pool!(), &id).await;
        if query_plan_wrap.is_err() {
            error!("查询提醒事项异常：{}", query_plan_wrap.unwrap_err());
            return Err(Error::from("查询提醒事项失败!"));
        }
        let plan_warp = query_plan_wrap.unwrap().into_iter().next();
        let mut plan_exist = plan_warp.ok_or_else(|| {
            Error::from((format!("id={} 的提醒事项不存在!", id), util::NOT_EXIST_CODE))
        })?;

        // 提前准备任务归档数据
        let plan_archive = PlanArchive {
            id: None,
            status: Some(3),
            title: plan_exist.title.clone(),
            content: plan_exist.content.clone(),
            archive_time: plan_exist.standard_time.clone(),
            organize: plan_exist.organize,
            user: plan_exist.user.clone(),
            display: plan_exist.display,
            create_time: DateTimeUtil::naive_date_time_to_str(
                &Some(DateUtils::now()),
                &util::FORMAT_Y_M_D_H_M_S,
            ),
            update_time: None,
        };
        if 1 == plan_exist.cycle.unwrap() {
            // 一次性的任务，计划提醒表(plan)数据删除，只用归档
            let write_result = PlanArchive::insert(primary_rbatis_pool!(), &plan_archive).await;
            if write_result.is_err() {
                error!(
                    "在归档计划提醒事项时id={}，archive_time={:?}，发生异常:{}",
                    plan_exist.id.unwrap(),
                    plan_exist.standard_time.clone(),
                    write_result.unwrap_err()
                );
            }
            // 移除这个调度任务
            Scheduler::remove(id).await;
            // 计划提醒表(plan)数据删除
            Plan::delete_by_id_organize(primary_rbatis_pool!(), &id, &user_info.organize).await;
            return Ok(0);
        }
        // 对于循环任务，需要生成下次的执行时间
        // 将上次计算好的本次时间放入到本次的基准时间
        plan_exist.standard_time = plan_exist.next_exec_time;

        let standard_time_result = chrono::NaiveDateTime::parse_from_str(
            &plan_exist.standard_time.clone().unwrap().as_str(),
            &util::FORMAT_Y_M_D_H_M_S,
        );
        if standard_time_result.is_err() {
            error!("格式化日期发生异常:{}", standard_time_result.unwrap_err());
            return Err(Error::from("创建提醒事项失败"));
        }
        let standard_time = standard_time_result.unwrap();

        // 计算下次执行时间
        let next_exec_time_op = DateUtils::plan_data_compute(
            &standard_time,
            plan_exist.cycle.unwrap(),
            plan_exist.unit.unwrap(),
        );
        if next_exec_time_op.is_none() {
            return Err(Error::from("无效的日期循环周期"));
        }
        let next_exec_time = next_exec_time_op.unwrap();

        plan_exist.next_exec_time =
            DateTimeUtil::naive_date_time_to_str(&Some(next_exec_time), &util::FORMAT_Y_M_D_H_M_S);

        let mut tx = primary_rbatis_pool!().acquire_begin().await.unwrap();
        let edit_plan_result = PlanMapper::update_plan(&mut tx, &plan_exist).await;
        if edit_plan_result.is_err() {
            error!(
                "在完成id={}的计划提醒时，发生异常:{}",
                id,
                edit_plan_result.unwrap_err()
            );
            tx.rollback().await;
            return Err(Error::from("提前完成提醒事项失败，请稍后再试"));
        }

        // 预写入任务归档数据
        let add_plan_archive_result = PlanArchive::insert(&mut tx, &plan_archive).await;
        if add_plan_archive_result.is_err() {
            error!(
                "在归档计划提醒时，发生异常:{}",
                add_plan_archive_result.unwrap_err()
            );
            tx.rollback().await;
            return Err(Error::from("提前完成提醒事项失败，请稍后再试"));
        }
        // 所有的写入都成功，最后正式提交
        tx.commit().await;
        // 生成定时cron表达式
        let cron_tab = DateUtils::data_time_to_cron(&standard_time);
        Scheduler::remove(plan_exist.id.unwrap()).await;
        Scheduler::add_plan(plan_exist.id.unwrap(), cron_tab.as_str()).await;
        LogMapper::record_log_by_context(primary_rbatis_pool!(), &user_info, String::from("OX023"))
            .await?;
        return Ok(add_plan_archive_result?.rows_affected);
    }

    /// 分页获取归档计划提醒数据(plan_archive)
    pub async fn plan_archive_page(
        &self,
        req: &HttpRequest,
        param: &PlanArchivePageDTO,
    ) -> Result<Page<PlanArchiveVO>> {
        let mut extend = ExtendPageDTO {
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time: param.begin_time.clone(),
            end_time: param.end_time.clone(),
        };
        let user_info = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
        let mut arg = param.clone();
        // 用户只能看到自己组织下的数据
        arg.organize = Some(user_info.organize);

        let count_result =
            PlanArchiveMapper::select_count(primary_rbatis_pool!(), &arg, &extend).await;
        if count_result.is_err() {
            error!(
                "分页归档计划提醒统计时，发生异常:{}",
                count_result.unwrap_err()
            );
            return Err(Error::from("分页归档计划提醒异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from((
                "未查询到符合条件的数据",
                util::NOT_EXIST_CODE,
            )));
        }
        let mut result = Page::<PlanArchiveVO>::page_query(total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no - 1) * result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result =
            PlanArchiveMapper::select_page(primary_rbatis_pool!(), &arg, &extend).await;
        if page_result.is_err() {
            error!(
                "在分页获取归档计划提醒页面数据时，发生异常:{}",
                page_result.unwrap_err()
            );
            return Err(Error::from("分页查询归档计划提醒异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

    /// 归档计划提醒的编辑(只能编辑完成与否，以及是否展示)
    pub async fn edit_plan_archive(&self, req: &HttpRequest, arg: &PlanArchiveDTO) -> Result<u64> {
        TokenUtils::check_token(arg.token.clone())
            .await
            .ok_or_else(|| Error::from(util::TOKEN_ERROR_CODE))?;
        let check_flag = arg.id.is_none();
        if check_flag {
            return Err(Error::from((
                "归档计划id不能为空!",
                util::NOT_PARAMETER_CODE,
            )));
        }
        let query_plan_archive_wrap =
            PlanArchive::select_by_id(primary_rbatis_pool!(), &arg.id.unwrap()).await;
        if query_plan_archive_wrap.is_err() {
            error!(
                "查询归档计划提醒异常：{}",
                query_plan_archive_wrap.unwrap_err()
            );
            return Err(Error::from("查询归档计划提醒失败!"));
        }
        let plan_archive_option = query_plan_archive_wrap.unwrap().into_iter().next();

        let mut plan_archive_exist = plan_archive_option.ok_or_else(|| {
            Error::from((
                format!("id={} 的归档提醒事项不存在!", arg.id.unwrap()),
                util::NOT_EXIST_CODE,
            ))
        })?;
        plan_archive_exist.status = arg.status;
        plan_archive_exist.display = arg.display;
        let result =
            PlanArchiveMapper::update_plan(primary_rbatis_pool!(), &plan_archive_exist).await;
        if result.is_err() {
            error!(
                "在修改id={}的提醒事项时，发生异常:{}",
                arg.id.as_ref().unwrap(),
                result.unwrap_err()
            );
            return Err(Error::from("提醒事项修改失败"));
        }
        let user_info = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
        LogMapper::record_log_by_context(primary_rbatis_pool!(), &user_info, String::from("OX023"))
            .await?;
        return Ok(result?.rows_affected);
    }

    /// 归档计划提醒的删除
    pub async fn delete_plan_archive(&self, req: &HttpRequest, id: &u64) -> Result<u64> {
        let user_info = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
        // 只能删除自己组织机构下的数据
        let write_result = PlanArchive::delete_by_id(primary_rbatis_pool!(), id).await;
        if write_result.is_err() {
            error!("删除归档提醒事项时，发生异常:{}", write_result.unwrap_err());
            return Err(Error::from("删除归档提醒事项失败!"));
        }
        LogMapper::record_log_by_context(primary_rbatis_pool!(), &user_info, String::from("OX024"))
            .await?;
        return Ok(write_result?.rows_affected);
    }

    /// 数据库备份日志分页
    pub async fn db_dump_log_page(&self, param: &DbDumpLogPageDTO) -> Result<Page<DbDumpLogVO>> {
        let mut extend = ExtendPageDTO {
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time: param.begin_time.clone(),
            end_time: param.end_time.clone(),
        };
        let count_result =
            DbDumpLogMapper::select_count(primary_rbatis_pool!(), &param, &extend).await;
        if count_result.is_err() {
            error!(
                "在数据库备份日志分页统计时，发生异常:{}",
                count_result.unwrap_err()
            );
            return Err(Error::from("数据库备份日志分页查询异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from((
                "未查询到符合条件的数据",
                util::NOT_EXIST_CODE,
            )));
        }
        let mut result = Page::<DbDumpLogVO>::page_query(total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no - 1) * result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result =
            DbDumpLogMapper::select_page(primary_rbatis_pool!(), &param, &extend).await;
        if page_result.is_err() {
            error!(
                "在数据库备份日志分页获取页面数据时，发生异常:{}",
                page_result.unwrap_err()
            );
            return Err(Error::from("数据库备份日志分页查询异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

    /// 获取消息动态详情[公众]
    pub async fn plan_grid(&self, organize: &u64, query_month: &str) -> Result<serde_json::Value> {
        // 只接受前端传入 archive_date = 年-月
        let _month = format!("{}-01", query_month);
        let month = _month.as_str();
        // 前端入参必须是该月的1号
        let user_month_wrap = chrono::NaiveDate::parse_from_str(month, &util::FORMAT_Y_M_D);
        if user_month_wrap.is_err() {
            return Err(Error::from(("查询时间格式有误!", util::NOT_PARAMETER_CODE)));
        }
        let user_month = user_month_wrap.unwrap();
        // 当月所有的天数
        let days =
            (DateUtils::get_current_month_days(user_month.year(), user_month.month())) as u64;
        // 得到本月1号是周几(从周末开始计算)
        let first_day = (user_month.weekday().num_days_from_sunday() + 1) as u64;
        // 表格中单元格个数（1号前的空单元格加上日历的单元格）
        let mut grid_count = days + first_day - 1;
        // 总行数
        let table_line = if grid_count % 7 == 0 {
            grid_count / 7
        } else {
            grid_count / 7 + 1
        };
        let mut result: Vec<Value> = Vec::new();
        // 统计有效的单元格（加上月尾的空白单元格）
        grid_count = table_line * 7;
        let query_sql = format!("call merge_plan({}, '{}')", organize, month);
        let compute_result_warp = primary_rbatis_pool!()
            .query_decode::<Vec<PlanArchiveVO>>(query_sql.as_str(), vec![])
            .await;
        if compute_result_warp.is_err() {
            error!(
                "在查询日期={}附近的计划安排时，发生异常:{}",
                month,
                compute_result_warp.unwrap_err()
            );
            return Err(Error::from("查询计划安排异常"));
        }
        let rows = compute_result_warp.unwrap();
        // 对计划按照当月号数进行分组
        let mut map: HashMap<u64, Vec<PlanArchiveVO>> = HashMap::new();
        for item in rows {
            let day_number = item.id.unwrap();
            if map.contains_key(&day_number) {
                let mut list = map.get(&day_number).unwrap().to_vec();
                list.push(item);
                map.insert(day_number, list);
            } else {
                map.insert(day_number, vec![item]);
            }
        }
        for number in 1..(grid_count + 1) {
            let mut day: Map<String, Value> = Map::new();
            let mut plan: Vec<Value> = Vec::new();
            if number >= first_day && number <= (days + first_day - 1) {
                // 今日的号数
                let today = number - (first_day - 1);
                day.insert(String::from("flag"), json!(1));
                day.insert(String::from("number"), json!(today));
                if map.contains_key(&today) {
                    // 本日有安排
                    let plans = map.get(&today).unwrap().to_vec();
                    for item in plans {
                        let mut value: Map<String, Value> = Map::new();
                        value.insert(String::from("archive_time"), json!(item.archive_time));
                        value.insert(String::from("title"), json!(item.title));
                        value.insert(String::from("content"), json!(item.content));
                        plan.push(json!(value));
                    }
                    day.insert(String::from("value"), json!(plan));
                }
            } else {
                // 输出空白
                day.insert(String::from("flag"), json!(0));
                day.insert(String::from("number"), json!(0));
            }
            result.push(json!(day));
        }
        return Ok(json!(result));
    }
}
