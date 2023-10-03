use crate::entity::dto::log::LogPageDTO;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::dto::sign_in::SignInDTO;
use crate::entity::dto::user::{UserDTO, UserPageDTO};
use crate::entity::vo::log::LogVO;
use crate::entity::vo::log_type::LogTypeVO;
use crate::entity::vo::sign_in::SignInVO;
use crate::entity::vo::user::{UserVO};
use crate::entity::table::{User};
use crate::dao::log_mapper::LogMapper;
use crate::dao::user_mapper::UserMapper;
use crate::service::CONTEXT;
use crate::util::date_time::{DateTimeUtil, DateUtils};
use crate::util::error::Error;
use crate::util::error::Result;
use crate::util::password_encoder_util::PasswordEncoder;
use crate::util::Page;
use actix_http::StatusCode;
use actix_web::{HttpRequest, HttpResponse};
use log::error;
use std::time::Duration;

extern crate simple_excel_writer as excel;

use crate::dao::log_type_mapper::LogTypeMapper;
use crate::entity::vo::user_context::UserContext;
use crate::util::token_util::TokenUtils;
use crate::{primary_rbatis_pool, util};
use excel::*;

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
            // TODO 模板填充，实际项目中 使用工具生成 IpUtils::city_location(&ip).await
            String::from("局域网地址")
        };
        // 准备壁纸
        let user_vo = UserVO::from(user.clone());
        let mut sign_vo = SignInVO {
            user: Some(user_vo),
            access_token: String::new(),
            plan: None,
            log: None,
        };
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
            .redis_service
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
            .redis_service
            .scan(&format!("{:}:{:}", &util::USER_CACHE_PREFIX, account))
            .await;
        if check.is_err() {
            return Err(check.unwrap_err());
        }
        let keys: Vec<String> = check.unwrap();
        CONTEXT.redis_service.batch_delete(&keys).await;
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
}
