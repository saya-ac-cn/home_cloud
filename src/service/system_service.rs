use std::collections::HashMap;
use actix_http::StatusCode;
use crate::error::Error;
use crate::error::Result;
use crate::service::{CONTEXT, SCHEDULER};
use crate::entity::vo::user::{UserOwnOrganizeVO, UserVO};
use crate::util::password_encoder::PasswordEncoder;
use actix_web::{HttpRequest, HttpResponse};
use chrono::Datelike;
use log::error;
use rbson::Bson;
use crate::dao::log_mapper::LogMapper;
use crate::dao::user_mapper::UserMapper;
use crate::entity::dto::page::{ExtendPageDTO};
use crate::entity::dto::sign_in::SignInDTO;
use crate::entity::dto::user::{UserDTO, UserPageDTO};
use crate::entity::vo::jwt::JWTToken;
use crate::entity::vo::sign_in::SignInVO;
use crate::util::Page;
use crate::entity::domain::primary_database_tables::{Plan, PlanArchive, User};
use crate::entity::dto::db_dump_log::DbDumpLogPageDTO;
use crate::entity::dto::log::LogPageDTO;
use crate::entity::dto::plan::{PlanDTO, PlanPageDTO};
use crate::entity::dto::plan_archive::{PlanArchiveDTO, PlanArchivePageDTO};
use crate::entity::vo::db_dump_log::DbDumpLogVO;
use crate::entity::vo::log::LogVO;
use crate::entity::vo::log_type::LogTypeVO;
use crate::entity::vo::plan::PlanVO;
use crate::entity::vo::plan_archive::PlanArchiveVO;
use crate::{business_rbatis_pool, primary_rbatis_pool, util};
use crate::util::date_time::{DateTimeUtil, DateUtils};
extern crate simple_excel_writer as excel;
use excel::*;
use rbs::to_value;
use crate::dao::db_dump_log_mapper::DbDumpLogMapper;
use crate::dao::log_type_mapper::LogTypeMapper;
use crate::dao::plan_archive_mapper::PlanArchiveMapper;
use crate::dao::plan_mapper::PlanMapper;
use crate::entity::domain::business_database_tables::Pictures;
use crate::entity::vo::total_log::TotalLogVO;
use crate::entity::vo::total_pre_6_month::TotalPre6MonthVO;
use crate::entity::vo::total_table::TotalTable;
use crate::util::ip_util::IpUtils;

/// 系统服务
pub struct SystemService {}

impl SystemService {
    /// 登录
    pub async fn login(&self, req: &HttpRequest, arg: &SignInDTO) -> Result<SignInVO> {
        if arg.account.is_none()
            || arg.account.as_ref().unwrap().is_empty()
            || arg.password.is_none()
            || arg.password.as_ref().unwrap().is_empty()
        {
            return Err(Error::from(("账号和密码不能为空!", util::NOT_PARAMETER)));
        }

        let query_user_wrap = User::select_by_account(primary_rbatis_pool!(),&arg.account.clone().unwrap()).await;
        if query_user_wrap.is_err() {
            error!("查询用户异常：{}",query_user_wrap.unwrap_err());
            return Err(Error::from("查询用户失败!"));
        }
        let user_wrap = query_user_wrap.unwrap().into_iter().next();
        let user = user_wrap.ok_or_else(|| Error::from((format!("账号:{} 不存在!", &arg.account.clone().unwrap()), util::NOT_EXIST)))?;
        // 判断用户是否被锁定，2为锁定
        if user.state.eq(&Some(0)) {
            return Err(Error::from("账户被禁用!"));
        }
        let mut error = None;
        if !PasswordEncoder::verify(
            user.password
                .as_ref()
                .ok_or_else(|| Error::from(("错误的用户数据，密码为空!", util::NOT_PARAMETER)))?,
            &arg.password.clone().unwrap(),
        ) {
            error = Some(Error::from("密码不正确!"));
        }
        if error.is_some() {
            // TODO 这里还应该设置失败锁
            return Err(error.unwrap());
        }
        let sign_in_vo = self.user_get_info(req, &user).await?;
        // 通过上面生成的token，完整记录日志
        let extract_result = &JWTToken::extract_token(&sign_in_vo.access_token);
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(), &extract_result.clone().unwrap(), String::from("OX001")).await;
        return Ok(sign_in_vo);
    }

    /// 生成用户jwt并返回
    pub async fn user_get_info(&self, req: &HttpRequest, user: &User) -> Result<SignInVO> {
        //去除密码，增加安全性
        let mut user = user.clone();
        let ip = if req.peer_addr().is_some() {
            req.peer_addr().unwrap().ip().to_string()
        } else {
            req.connection_info().realip_remote_addr().unwrap().parse().unwrap()
        };
        let city = if ip.eq("127.0.0.1") || ip.eq("localhost") {
            String::from("局域网地址")
        }else {
            IpUtils::city_location(&ip).await
        };

        user.password = None;
        // 准备壁纸
        let mut user_vo = UserVO::from(user.clone());
        let query_picture_wrap = Pictures::select_by_column(business_rbatis_pool!(),Pictures::id(),&user_vo.background).await;
        if query_picture_wrap.is_err() {
            error!("查询壁纸异常：{}",query_picture_wrap.unwrap_err());
        }else{
            let picture = query_picture_wrap.unwrap().into_iter().next();
            user_vo.background_url = if picture.is_some() { picture.unwrap().web_url}else { None }
        }
        let mut sign_vo = SignInVO {
            user: Some(user_vo),
            access_token: String::new(),
            plan:None,
            log:None
        };
        // 查询准备今日计划安排
        let query_today_plan_sql = "select concat(`title`,'[',date_format(`archive_time`,'%Y-%m-%d'),']') as item from `plan_archive` where `status` != 3 and `user` = ? and `archive_time` <= date_format(now(),'%Y-%m-%d 23:59:59')\n
            union all\n
            select concat(b.`title`,'[',date_format(b.`standard_time`,'%Y-%m-%d'),']') as item from `plan` b where b.`user` = ? and b.`standard_time` <= date_format(now(),'%Y-%m-%d 23:59:59')";
        let today_plan_result_warp = primary_rbatis_pool!().fetch_decode::<Vec<HashMap<String, String>>>(query_today_plan_sql, vec![to_value!(user.account.clone()),to_value!(user.account.clone())]).await;
        if today_plan_result_warp.is_ok(){
            sign_vo.plan = Some(today_plan_result_warp.unwrap());
        }

        // 查询最近的一次操作日志
        let log_warp = LogMapper::select_recently(primary_rbatis_pool!(), &user.account.clone().unwrap()).await;
        if log_warp.is_ok() {
            sign_vo.log = log_warp.unwrap();
        }

        let jwt_token = JWTToken {
            account: user.account.unwrap_or_default(),
            name: user.name.clone().unwrap_or_default(),
            ip,
            organize:user.organize_id.unwrap(),
            city: city,
            exp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as usize,// 时间和校验的时候保持一致，统一秒
        };
        sign_vo.access_token = jwt_token.create_token(&CONTEXT.config.jwt_secret)?;
        return Ok(sign_vo);
    }

    /// 刷新token
    pub async fn token_refresh(&self, req: &HttpRequest) -> Result<String>{
        let mut jwt_token = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        // 时间和校验的时候保持一致，统一秒
        jwt_token.exp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as usize;
        let access_token = jwt_token.create_token(&CONTEXT.config.jwt_secret).unwrap();
        Ok(access_token)
    }

    /// 登出后台
    pub async fn logout(&self, req: &HttpRequest) {
        let user_info = JWTToken::extract_user_by_request(req).unwrap();
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(), &user_info, String::from("OX002")).await;
    }

    /// 用户分页
    pub async fn user_page(&self, arg: &UserPageDTO) -> Result<Page<UserVO>> {
        let mut extend = ExtendPageDTO {
            page_no: arg.page_no,
            page_size: arg.page_size,
            begin_time: arg.begin_time.clone(),
            end_time: arg.end_time.clone()
        };
        let count_result = UserMapper::select_count(primary_rbatis_pool!(), &arg, &extend).await;
        if count_result.is_err() {
            error!("在用户分页统计时，发生异常:{}",count_result.unwrap_err());
            return Err(Error::from("用户分页查询异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from(("未查询到符合条件的数据", util::NOT_EXIST)));
        }
        let mut result = Page::<UserVO>::page_query(total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no - 1) * result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = UserMapper::select_page(primary_rbatis_pool!(), &arg, &extend).await;
        if page_result.is_err() {
            error!("在用户分页获取页面数据时，发生异常:{}",page_result.unwrap_err());
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
        let check_flag = arg.account.is_none() || arg.account.as_ref().unwrap().is_empty() || arg.name.is_none() || arg.name.as_ref().unwrap().is_empty() || arg.email.is_none() || arg.email.as_ref().unwrap().is_empty() || arg.phone.is_none() || arg.phone.as_ref().unwrap().is_empty() || arg.organize_id.is_none();
        if check_flag {
            return Err(Error::from(("账号、姓名、手机号、邮箱以及所属组织不能为空!", util::NOT_PARAMETER)));
        }

        let query_user_wrap = User::select_by_account(primary_rbatis_pool!(),&arg.account.clone().unwrap()).await;
        if query_user_wrap.is_err() {
            error!("查询用户异常：{}",query_user_wrap.unwrap_err());
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
            create_time: DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()),&util::FORMAT_Y_M_D_H_M_S),
            update_time: None,
        };
        let write_result = User::insert(primary_rbatis_pool!(),&user).await;
        if write_result.is_err() {
            error!("创建账号时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("创建账号时，发生异常!"));
        }
        // 当前不允许创建用户操作
        // LogMapper::record_log(&CONTEXT.primary_rbatis,String::from(""));
        return Ok(write_result?.rows_affected);
    }

    /// 通过token获取用户信息
    pub async fn user_get_info_by_token(&self, req: &HttpRequest) -> Result<SignInVO> {
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let query_user_wrap = User::select_by_account(primary_rbatis_pool!(), &user_info.account).await;
        if query_user_wrap.is_err() {
            error!("查询用户异常：{}",query_user_wrap.unwrap_err());
            return Err(Error::from("查询用户失败!"));
        }
        let user_warp = query_user_wrap.unwrap().into_iter().next();
        let user = user_warp.ok_or_else(|| Error::from((format!("账号:{} 不存在!", &user_info.account), util::NOT_EXIST)))?;
        return self.user_get_info(req, &user).await;
    }

    /// 修改用户信息
    pub async fn user_edit(&self, req: &HttpRequest, arg: &UserDTO) -> Result<u64> {
        if arg.account.is_none() || arg.account.as_ref().unwrap().is_empty() {
            return Err(Error::from(("账号account不能为空!", util::NOT_PARAMETER)));
        }
        // 首先判断要修改的用户是否存在
        let query_user_wrap = User::select_by_account(primary_rbatis_pool!(),  &arg.account.clone().unwrap()).await;
        if query_user_wrap.is_err() {
            error!("查询用户异常：{}",query_user_wrap.unwrap_err());
            return Err(Error::from("查询用户失败!"));
        }
        let user_warp = query_user_wrap.unwrap().into_iter().next();
        let user_exist = user_warp.ok_or_else(|| Error::from((format!("账号:{} 不存在!", &arg.account.clone().unwrap()), util::NOT_EXIST)))?;

        let user_edit = User {
            account: user_exist.account,
            name: arg.name.clone(),
            password: if arg.password.is_some() { Some(PasswordEncoder::encode(arg.password.as_ref().unwrap())) } else { user_exist.password },
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
            update_time: DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()),&util::FORMAT_Y_M_D_H_M_S),
        };
        let result = UserMapper::update_user(primary_rbatis_pool!(), &user_edit).await;//CONTEXT.primary_rbatis.update_by_column(User::account(),&user_edit).await?;
        if result.is_err() {
            error!("在修改用户{}的信息时，发生异常:{}",arg.account.as_ref().unwrap(),result.unwrap_err());
            return Err(Error::from(format!("修改账户[{}]信息失败!", arg.account.as_ref().unwrap())));
        }
        let jwt_token = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(), &jwt_token, String::from("OX003")).await;
        Ok(result.unwrap().rows_affected)
    }

    /// 删除用户
    pub async fn user_remove(&self, account: &str) -> Result<u64> {
        if account.is_empty() {
            return Err(Error::from(("account 不能为空！", util::NOT_PARAMETER)));
        }
        let r = User::delete_by_column(primary_rbatis_pool!(),User::account(),&account).await?;
        return Ok(r.rows_affected);
    }

    /// 用户详情
    pub async fn user_detail(&self, arg: &UserDTO) -> Result<UserVO> {
        let account = arg.account.clone().unwrap_or_default();
        let query_user_wrap = User::select_by_account(primary_rbatis_pool!(),  &account.clone()).await;
        if query_user_wrap.is_err() {
            error!("查询用户异常：{}",query_user_wrap.unwrap_err());
            return Err(Error::from("查询用户失败!"));
        }
        let user_warp = query_user_wrap.unwrap().into_iter().next();
        let user = user_warp.ok_or_else(|| Error::from((format!("账号:{} 不存在!", &account.clone()), util::NOT_EXIST)))?;
        let user_vo = UserVO::from(user);
        return Ok(user_vo);
    }

    /// 获取当前用户所在组织的用户列表
    pub async fn user_get_own_organize(&self, req: &HttpRequest) -> Result<Vec<UserOwnOrganizeVO>> {
        let jwt_token = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let query_result = UserMapper::select_own_organize_user(primary_rbatis_pool!(), &jwt_token.account).await;
        if query_result.is_err() {
            error!("在查询用户所属组织下的用户列表时，发生异常:{}",query_result.unwrap_err());
            return Err(Error::from(format!("查询我所属组织的用户列表异常")));
        }
        return Ok(query_result.unwrap().unwrap());
    }

    /// 修改用户密码
    pub async fn user_update_password(&self, req: &HttpRequest, arg: &UserDTO) -> Result<u64> {
        if arg.password.is_none() || arg.password.as_ref().unwrap().is_empty() {
            return Err(Error::from(("密码不能为空!", util::NOT_PARAMETER)));
        }
        // 首先判断要修改的用户是否存在

        let query_user_wrap = User::select_by_account(primary_rbatis_pool!(),  &arg.account.clone().unwrap()).await;
        if query_user_wrap.is_err() {
            error!("查询用户异常：{}",query_user_wrap.unwrap_err());
            return Err(Error::from("查询用户失败!"));
        }
        let user_warp = query_user_wrap.unwrap().into_iter().next();
        let user_exist = user_warp.ok_or_else(|| Error::from((format!("账号:{} 不存在!", &arg.account.clone().unwrap()), util::NOT_EXIST)))?;

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
            update_time: DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()),&util::FORMAT_Y_M_D_H_M_S),
        };
        let result = UserMapper::update_user(primary_rbatis_pool!(), &user_edit).await;//CONTEXT.primary_rbatis.update_by_column(User::account(),&user_edit).await?;
        if result.is_err() {
            error!("在修改用户{}的密码时，发生异常:{}",arg.account.as_ref().unwrap(),result.unwrap_err());
            return Err(Error::from(format!("修改账户[{}]密码失败!", arg.account.as_ref().unwrap())));
        }
        let jwt_token = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(), &jwt_token, String::from("OX004")).await;
        Ok(result.unwrap().rows_affected)
    }

    /// 日志类别列表
    pub async fn log_get_type(&self) -> Result<Vec<LogTypeVO>> {
        let query_result = LogTypeMapper::select_all(primary_rbatis_pool!()).await;
        if query_result.is_err() {
            error!("在查询日志类型列表时，发生异常:{}",query_result.unwrap_err());
            return Err(Error::from("查询日志类型列表异常"));
        }
        return Ok(query_result.unwrap().unwrap());
    }

    /// 日志分页
    pub async fn log_page(&self, req: &HttpRequest,param: &LogPageDTO) -> Result<Page<LogVO>>  {
        let mut extend = ExtendPageDTO{
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time:param.begin_time.clone(),
            end_time:param.end_time.clone()
        };
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let mut arg= param.clone();
        arg.organize = Some(user_info.organize);

        let count_result = LogMapper::select_count(primary_rbatis_pool!(), &arg,&extend).await;
        if count_result.is_err(){
            error!("在日志分页统计时，发生异常:{}",count_result.unwrap_err());
            return Err(Error::from("日志分页查询异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from(("未查询到符合条件的数据",util::NOT_EXIST)));
        }
        let mut result = Page::<LogVO>::page_query( total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no-1)*result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = LogMapper::select_page(primary_rbatis_pool!(), &arg,&extend).await;
        if page_result.is_err() {
            error!("在日志分页获取页面数据时，发生异常:{}",page_result.unwrap_err());
            return Err(Error::from("日志分页查询异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

    /// 导出日志分页
    pub async fn log_excel(&self, req: &HttpRequest,param: &LogPageDTO) -> HttpResponse  {
        let mut response = HttpResponse::Ok();
        let mut extend = ExtendPageDTO{
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time:param.begin_time.clone(),
            end_time:param.end_time.clone()
        };
        let user_info = JWTToken::extract_user_by_request(req).unwrap();
        let mut arg= param.clone();
        arg.organize = Some(user_info.organize);

        let count_result = LogMapper::select_count(primary_rbatis_pool!(), &arg,&extend).await;
        if count_result.is_err(){
            error!("在日志分页统计时，发生异常:{}",count_result.unwrap_err());
            response.status(StatusCode::INTERNAL_SERVER_ERROR);
            return response.finish()
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            response.status(StatusCode::NOT_FOUND);
            return response.finish()
        }
        let result = Page::<LogVO>::page_query( total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no-1)*result.page_size);
        extend.page_size = Some(total_row);
        let page_result = LogMapper::select_page(primary_rbatis_pool!(), &arg,&extend).await;
        if page_result.is_err() {
            error!("在日志分页获取页面数据时，发生异常:{}",page_result.unwrap_err());
            response.status(StatusCode::INTERNAL_SERVER_ERROR);
            return response.finish()
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
                sw.append_row(row![item.user.unwrap(),item.detail.unwrap(),item.ip.unwrap(),item.city.unwrap(),item.date.unwrap()]);
            }
            Ok(())
        }).expect("write excel error!");
        // 这里面是直接返回流的
        let excel_stream = wb.close().expect("close excel error!");
        response.content_type("application/octet-stream;charset=UTF-8");
        response.insert_header((actix_web::http::header::CONTENT_DISPOSITION, "attachment;filename=log.xlsx"));
        response.body(excel_stream.unwrap())
    }

    /// 计算近6个月的活跃情况
    pub async fn compute_pre6_logs(&self, req: &HttpRequest,month:&String) ->Result<TotalLogVO> {
        let user_info = JWTToken::extract_user_by_request(req).unwrap();
        let user_month_wrap = chrono::NaiveDate::parse_from_str(month.as_str(),&util::FORMAT_Y_M_D);
        if user_month_wrap.is_err() {
            return Err(Error::from(("统计月份不能为空!", util::NOT_PARAMETER)));
        }
        let user_month = user_month_wrap.unwrap();
        // 查询用户指定的月份日志数量
        let query_current_month_log_sql = "select count(1) from `log` where `organize` = ? and  `date` like concat(date_format(?,'%Y-%m'),'%')";
        let current_month_log_result_warp = primary_rbatis_pool!().fetch_decode::<u64>(query_current_month_log_sql, vec![to_value!(user_info.organize),to_value!(month.as_str())]).await;
        let mut current_month_log:u64 = 0;
        if current_month_log_result_warp.is_ok(){
            current_month_log = current_month_log_result_warp.unwrap()
        }else {
            error!("在查询指定月份的日志数据总数时，发生异常:{}",current_month_log_result_warp.unwrap_err());
        }
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
        let avg:u64 = current_month_log/(days as u64);

        let query_sql = format!("call count_pre6_logs({}, '{}')", &user_info.organize,month);
        let param:Vec<Bson> = Vec::new();
        let compute_result_warp = primary_rbatis_pool!().fetch_decode::<Vec<TotalPre6MonthVO>>(query_sql.as_str(), vec![]).await;
        if compute_result_warp.is_err(){
            error!("在统计近6个月的活跃情况时，发生异常:{}",compute_result_warp.unwrap_err());
            return Err(Error::from("统计近6个月的活跃情况异常"));
        }
        let rows:Vec<TotalPre6MonthVO> = compute_result_warp.unwrap();
        let result = TotalLogVO{
            avg:Some(avg),
            count:Some(current_month_log),
            log6:Some(rows)
        };
        return Ok(result);
    }

    /// 统计各个表的数据体量
    pub async fn compute_object_rows(&self, req: &HttpRequest)->Result<rbson::Document> {
        // 最终结果集的容器
        let mut rose_data: Vec<Bson> = rbson::Array::new();
        let mut word_cloud: Vec<Bson> = rbson::Array::new();
        let mut result = rbson::Document::new();
        let user_info = JWTToken::extract_user_by_request(req).unwrap();

        let query_notebook_sql = "select a.`name`, count(b.`id`) as value from `note_book` a left join `notes` b on a.`id` = b.`notebook_id` where a.`organize` = ? group by a.`id`";
        let query_notebook_result_warp = business_rbatis_pool!().fetch_decode(query_notebook_sql, vec![to_value!(user_info.organize)]).await;
        if query_notebook_result_warp.is_ok(){
            let business_rows:Vec<TotalTable> = query_notebook_result_warp.unwrap();
            for item in business_rows {
                let mut current_data = rbson::Document::new();
                current_data.insert("name",item.name);
                current_data.insert("value",item.value);
                word_cloud.push(Bson::Document(current_data));
            }
        }else {
            error!("在分类别查询笔记簿时，发生异常:{}",query_notebook_result_warp.unwrap_err());
        }
        result.insert("word_cloud", word_cloud);

        let query_primary_count_sql = format!("select '计划' as `name` , count(1) as `value` from `plan` where `organize` = {};", &user_info.organize);
        let param:Vec<Bson> = Vec::new();
        let primary_count_result_warp = primary_rbatis_pool!().fetch_decode(query_primary_count_sql.as_str(), vec![]).await;
        // 异常健壮处理
        let mut error_plan = Bson::Null;
        let mut primary_rows:rbson::Array = rbson::Array::new();
        if primary_count_result_warp.is_err(){
            error!("在统计主库的计划提醒时，发生异常:{}",primary_count_result_warp.unwrap_err());
            let mut plan = rbson::Document::new();
            plan.insert("name","计划");
            plan.insert("value",0);
            error_plan = Bson::Document(plan);
            rose_data.push(error_plan);
        }else {
            primary_rows = primary_count_result_warp.unwrap();
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
        let param:Vec<Bson> = Vec::new();
        let business_count_result_warp = business_rbatis_pool!().fetch_decode::<Vec<TotalTable>>(query_business_count_sql.as_str(), vec![]).await;

        // 异常健壮处理
        let mut error_files = Bson::Null;
        let mut error_pictures = Bson::Null;
        let mut error_notebook = Bson::Null;
        let mut error_note = Bson::Null;
        let mut error_news = Bson::Null;
        let mut business_rows:Vec<TotalTable> = vec![];

        if business_count_result_warp.is_err(){
            error!("在统计业务库的各表总数据量时，发生异常:{}",business_count_result_warp.unwrap_err());
            let mut files = rbson::Document::new();
            files.insert("name","文件");
            files.insert("value",0);
            error_files = Bson::Document(files);
            rose_data.push(error_files);
            let mut pictures = rbson::Document::new();
            pictures.insert("name","图片");
            pictures.insert("value",0);
            error_pictures = Bson::Document(pictures);
            rose_data.push(error_pictures);
            let mut notebook = rbson::Document::new();
            notebook.insert("name","笔记簿");
            notebook.insert("value",0);
            error_notebook = Bson::Document(notebook);
            rose_data.push(error_notebook);
            let mut notes = rbson::Document::new();
            notes.insert("name","笔记");
            notes.insert("value",0);
            error_note = Bson::Document(notes);
            rose_data.push(error_note);
            let mut news = rbson::Document::new();
            news.insert("name","动态");
            news.insert("value",0);
            error_news = Bson::Document(news);
            rose_data.push(error_news);
        }else {
            business_rows = business_count_result_warp.unwrap();
            for item in business_rows {
                let mut current_data = rbson::Document::new();
                current_data.insert("name",item.name);
                current_data.insert("value",item.value);
                rose_data.push(Bson::Document(current_data));
            }
        }
        result.insert("rose_data", rose_data);
        return Ok(result);
    }

    /// 创建提醒事项
    pub async fn add_plan(&self, req: &HttpRequest,arg: &PlanDTO) -> Result<u64> {
        let check_flag = arg.standard_time.is_none() || arg.cycle.is_none() || arg.unit.is_none() || arg.content.is_none() || arg.content.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("基准时间、重复执行周期、单位和内容不能为空!",util::NOT_PARAMETER)));
        }
        let cycle = arg.cycle.unwrap();

        let standard_time_result = chrono::NaiveDateTime::parse_from_str(&arg.standard_time.clone().unwrap().as_str(),&util::FORMAT_Y_M_D_H_M_S);
        if standard_time_result.is_err() {
            error!("格式化日期发生异常:{}",standard_time_result.unwrap_err());
            return Err(Error::from("创建提醒事项失败"));
        }
        let standard_time = standard_time_result.unwrap();

        // 计算下次执行时间
        let next_exec_time = DateUtils::plan_data_compute(&standard_time,arg.cycle.unwrap(),arg.unit.unwrap());
        // 生成定时cron表达式
        let cron_tab = DateUtils::data_time_to_cron(&standard_time);
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        //   `cycle` 重复执行周期(1：一次性，2：天，3：周，4：月，5：年)',
        //   `unit` '重复执行周期单位',
        let plan = Plan{
            id:None,
            standard_time: arg.standard_time.clone(),
            cycle: Some(cycle),
            unit: arg.unit,
            title:arg.title.clone(),
            content: arg.content.clone(),
            // 一次性的任务，不用生成下一次执行时间
            next_exec_time: if 1 == cycle { None } else {DateTimeUtil::naive_date_time_to_str(&Some(next_exec_time),&util::FORMAT_Y_M_D_H_M_S)},
            organize: Some(user_info.organize),
            user: Some(user_info.account.clone()),
            display: arg.display,
            create_time: DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()),&util::FORMAT_Y_M_D_H_M_S),
            update_time: None
        };

        let write_result = Plan::insert(primary_rbatis_pool!(),&plan).await;
        if  write_result.is_err(){
            error!("在保存计划提醒事项时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("保存计划提醒事项失败!"));
        }
        let result = write_result.unwrap();
        let plan_id = result.last_insert_id.as_u64().unwrap();
        SCHEDULER.lock().unwrap().add(plan_id,cron_tab.as_str());
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX022")).await;
        return Ok(result.rows_affected);
    }

    /// 修改提醒事项
    pub async fn edit_plan(&self, req: &HttpRequest,arg: &PlanDTO) -> Result<u64> {
        let check_flag = arg.id.is_none() || arg.standard_time.is_none() || arg.cycle.is_none() || arg.unit.is_none() || arg.content.is_none() || arg.content.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("基准时间、重复执行周期、单位和内容不能为空!",util::NOT_PARAMETER)));
        }
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;

        let query_plan_wrap = Plan::select_by_column(primary_rbatis_pool!(),  Plan::id(), &arg.id).await;
        if query_plan_wrap.is_err() {
            error!("查询提醒事项异常：{}",query_plan_wrap.unwrap_err());
            return Err(Error::from("查询提醒事项失败!"));
        }
        let plan_warp = query_plan_wrap.unwrap().into_iter().next();
        let plan_exist = plan_warp.ok_or_else(|| Error::from((format!("id={} 的提醒事项不存在!", arg.id.unwrap()), util::NOT_EXIST)))?;
        let cycle = arg.cycle.unwrap();

        let standard_time_result = chrono::NaiveDateTime::parse_from_str(&arg.standard_time.clone().unwrap().as_str(),&util::FORMAT_Y_M_D_H_M_S);
        if standard_time_result.is_err() {
            error!("格式化日期发生异常:{}",standard_time_result.unwrap_err());
            return Err(Error::from("创建提醒事项失败"));
        }
        let standard_time = standard_time_result.unwrap();

        // 计算下次执行时间
        let next_exec_time = DateUtils::plan_data_compute(&standard_time,arg.cycle.unwrap(),arg.unit.unwrap());
        // 生成定时cron表达式
        let cron_tab = DateUtils::data_time_to_cron(&standard_time);
        let plan = Plan{
            id:plan_exist.id,
            standard_time: arg.standard_time.clone(),
            cycle: Some(cycle),
            unit: arg.unit,
            title:arg.title.clone(),
            content: arg.content.clone(),
            // 一次性的任务，不用生成下一次执行时间
            next_exec_time: if 1 == cycle { None } else {DateTimeUtil::naive_date_time_to_str(&Some(next_exec_time),&util::FORMAT_Y_M_D_H_M_S)},
            organize: plan_exist.organize,
            user: Some(user_info.account.clone()),
            display: arg.display,
            create_time: None,
            update_time: DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()),&util::FORMAT_Y_M_D_H_M_S)
        };
        let result = PlanMapper::update_plan(primary_rbatis_pool!(),&plan).await;
        if result.is_err() {
            error!("在修改id={}的提醒事项时，发生异常:{}",arg.id.as_ref().unwrap(),result.unwrap_err());
            return Err(Error::from("提醒事项修改失败"));
        }
        let mut scheduler = SCHEDULER.lock().unwrap();
        scheduler.remove(plan_exist.id.unwrap());
        scheduler.add(plan_exist.id.unwrap(),cron_tab.as_str());
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX023")).await;
        return Ok(result?.rows_affected);
    }

    /// 删除提醒事项
    pub async fn delete_plan(&self, req: &HttpRequest,id: &u64) -> Result<u64> {
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        // 只能删除自己组织机构下的数据
        let write_result = Plan::delete_by_id_organize(primary_rbatis_pool!(),id,&user_info.organize).await;
        if write_result.is_err(){
            error!("删除提醒事项时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("删除提醒事项失败!"));
        }
        SCHEDULER.lock().unwrap().remove(*id);
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX024")).await;
        return Ok(write_result?.rows_affected);
    }

    /// 分页查询当前活跃的计划提醒(plan)
    pub async fn plan_page(&self, req: &HttpRequest, param: &PlanPageDTO) -> Result<Page<PlanVO>>  {
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

        let count_result = PlanMapper::select_count(primary_rbatis_pool!(), &arg,&extend).await;
        if count_result.is_err(){
            error!("在计划提醒分页统计时，发生异常:{}",count_result.unwrap_err());
            return Err(Error::from("计划提醒分页查询异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from(("未查询到符合条件的数据",util::NOT_EXIST)));
        }
        let mut result = Page::<PlanVO>::page_query( total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no-1)*result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = PlanMapper::select_page(primary_rbatis_pool!(), &arg,&extend).await;
        if page_result.is_err() {
            error!("在计划提醒分页获取页面数据时，发生异常:{}",page_result.unwrap_err());
            return Err(Error::from("计划提醒分页查询异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

    /// 提前完成计划提醒
    pub async fn advance_finish_plan(&self, req: &HttpRequest,id: &u64) -> Result<u64> {
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let query_plan_wrap = Plan::select_by_column(primary_rbatis_pool!(),  Plan::id(), id).await;
        if query_plan_wrap.is_err() {
            error!("查询提醒事项异常：{}",query_plan_wrap.unwrap_err());
            return Err(Error::from("查询提醒事项失败!"));
        }
        let plan_warp = query_plan_wrap.unwrap().into_iter().next();
        let mut plan_exist = plan_warp.ok_or_else(|| Error::from((format!("id={} 的提醒事项不存在!", id), util::NOT_EXIST)))?;

        // 提前准备任务归档数据
        let plan_archive = PlanArchive{
            id: None,
            status: Some(3),
            title: plan_exist.title.clone(),
            content: plan_exist.content.clone(),
            archive_time: plan_exist.standard_time.clone(),
            organize:plan_exist.organize,
            user:plan_exist.user.clone(),
            display:plan_exist.display,
            create_time: DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()),&util::FORMAT_Y_M_D_H_M_S),
            update_time: None
        };
        let mut scheduler = SCHEDULER.lock().unwrap();
        if 1 == plan_exist.cycle.unwrap() {
            // 一次性的任务，计划提醒表(plan)按兵不动，只用归档
            let write_result = PlanArchive::insert(primary_rbatis_pool!(),&plan_archive).await;
            if  write_result.is_err(){
                error!("在归档计划提醒事项时id={}，archive_time={:?}，发生异常:{}",plan_exist.id.unwrap(),plan_exist.standard_time.clone(),write_result.unwrap_err());
            }
            // 移除这个调度任务
            scheduler.remove(*id);
            return Ok(0);
        }
        // 对于循环任务，需要生成下次的执行时间
        // 将上次计算好的本次时间放入到本次的基准时间
        plan_exist.standard_time = plan_exist.next_exec_time;

        let standard_time_result = chrono::NaiveDateTime::parse_from_str(&plan_exist.standard_time.clone().unwrap().as_str(),&util::FORMAT_Y_M_D_H_M_S);
        if standard_time_result.is_err() {
            error!("格式化日期发生异常:{}",standard_time_result.unwrap_err());
            return Err(Error::from("创建提醒事项失败"));
        }
        let standard_time = standard_time_result.unwrap();

        // 计算下次执行时间
        let next_exec_time = DateUtils::plan_data_compute(&standard_time,plan_exist.cycle.unwrap(),plan_exist.unit.unwrap());
        plan_exist.next_exec_time = DateTimeUtil::naive_date_time_to_str(&Some(next_exec_time),&util::FORMAT_Y_M_D_H_M_S);

        let mut tx = primary_rbatis_pool!().acquire_begin().await.unwrap();
        let edit_plan_result = PlanMapper::update_plan(&mut tx, &plan_exist).await;
        if edit_plan_result.is_err() {
            error!("在完成id={}的计划提醒时，发生异常:{}",id,edit_plan_result.unwrap_err());
            tx.rollback();
            return Err(Error::from("提前完成提醒事项失败，请稍后再试"));
        }

        // 预写入任务归档数据
        let add_plan_archive_result = PlanArchive::insert(&mut tx,&plan_archive).await;
        if add_plan_archive_result.is_err() {
            error!("在归档计划提醒时，发生异常:{}",add_plan_archive_result.unwrap_err());
            tx.rollback();
            return Err(Error::from("提前完成提醒事项失败，请稍后再试"));
        }
        // 所有的写入都成功，最后正式提交
        tx.commit().await;
        // 生成定时cron表达式
        let cron_tab = DateUtils::data_time_to_cron(&standard_time);
        scheduler.remove(plan_exist.id.unwrap());
        scheduler.add(plan_exist.id.unwrap(),cron_tab.as_str());
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX023")).await;
        return Ok(add_plan_archive_result?.rows_affected);
    }

    /// 分页获取归档计划提醒数据(plan_archive)
    pub async fn plan_archive_page(&self, req: &HttpRequest, param: &PlanArchivePageDTO) -> Result<Page<PlanArchiveVO>>  {
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

        let count_result = PlanArchiveMapper::select_count(primary_rbatis_pool!(), &arg,&extend).await;
        if count_result.is_err(){
            error!("分页归档计划提醒统计时，发生异常:{}",count_result.unwrap_err());
            return Err(Error::from("分页归档计划提醒异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from(("未查询到符合条件的数据",util::NOT_EXIST)));
        }
        let mut result = Page::<PlanArchiveVO>::page_query( total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no-1)*result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = PlanArchiveMapper::select_page(primary_rbatis_pool!(), &arg,&extend).await;
        if page_result.is_err() {
            error!("在分页获取归档计划提醒页面数据时，发生异常:{}",page_result.unwrap_err());
            return Err(Error::from("分页查询归档计划提醒异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

    /// 归档计划提醒的编辑(只能编辑完成与否，以及是否展示)
    pub async fn edit_plan_archive(&self, req: &HttpRequest,arg: &PlanArchiveDTO) -> Result<u64> {
        let check_flag = arg.id.is_none();
        if check_flag{
            return Err(Error::from(("归档计划id不能为空!",util::NOT_PARAMETER)));
        }
        let query_plan_archive_wrap = PlanArchive::select_by_column(primary_rbatis_pool!(),  PlanArchive::id(), &arg.id).await;
        if query_plan_archive_wrap.is_err() {
            error!("查询归档计划提醒异常：{}",query_plan_archive_wrap.unwrap_err());
            return Err(Error::from("查询归档计划提醒失败!"));
        }
        let plan_archive_option = query_plan_archive_wrap.unwrap().into_iter().next();

        let mut plan_archive_exist = plan_archive_option.ok_or_else(|| Error::from((format!("id={} 的归档提醒事项不存在!", arg.id.unwrap()), util::NOT_EXIST)))?;
        plan_archive_exist.status = arg.status;
        plan_archive_exist.display = arg.display;
        let result = PlanArchiveMapper::update_plan(primary_rbatis_pool!(),&plan_archive_exist).await;
        if result.is_err() {
            error!("在修改id={}的提醒事项时，发生异常:{}",arg.id.as_ref().unwrap(),result.unwrap_err());
            return Err(Error::from("提醒事项修改失败"));
        }
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX023")).await;
        return Ok(result?.rows_affected);
    }

    /// 归档计划提醒的删除
    pub async fn delete_plan_archive(&self, req: &HttpRequest,id: &u64) -> Result<u64> {
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        // 只能删除自己组织机构下的数据
        let write_result = PlanArchive::delete_by_column(primary_rbatis_pool!(),PlanArchive::id(),id).await;
        if write_result.is_err(){
            error!("删除归档提醒事项时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("删除归档提醒事项失败!"));
        }
        LogMapper::record_log_by_jwt(primary_rbatis_pool!(),&user_info,String::from("OX024")).await;
        return Ok(write_result?.rows_affected);
    }

    /// 数据库备份日志分页
    pub async fn db_dump_log_page(&self, req: &HttpRequest,param: &DbDumpLogPageDTO) -> Result<Page<DbDumpLogVO>>  {
        let mut extend = ExtendPageDTO{
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time:param.begin_time.clone(),
            end_time:param.end_time.clone()
        };
        let count_result = DbDumpLogMapper::select_count(primary_rbatis_pool!(), &param,&extend).await;
        if count_result.is_err(){
            error!("在数据库备份日志分页统计时，发生异常:{}",count_result.unwrap_err());
            return Err(Error::from("数据库备份日志分页查询异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from(("未查询到符合条件的数据",util::NOT_EXIST)));
        }
        let mut result = Page::<DbDumpLogVO>::page_query( total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no-1)*result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = DbDumpLogMapper::select_page(primary_rbatis_pool!(), &param,&extend).await;
        if page_result.is_err() {
            error!("在数据库备份日志分页获取页面数据时，发生异常:{}",page_result.unwrap_err());
            return Err(Error::from("数据库备份日志分页查询异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

    /// 获取消息动态详情[公众]
    pub async fn plan_grid(&self,organize:&u64,query_month:&str) -> Result<Vec<Bson>>{
        // 只接受前端传入 archive_date = 年-月
        let _month = format!("{}-01",query_month);
        let month = _month.as_str();
        // 前端入参必须是该月的1号
        let user_month_wrap = chrono::NaiveDate::parse_from_str(month,&util::FORMAT_Y_M_D);
        if user_month_wrap.is_err() {
            return Err(Error::from(("查询时间格式有误!", util::NOT_PARAMETER)));
        }
        let user_month = user_month_wrap.unwrap();
        // 当月所有的天数
        let days = (DateUtils::get_current_month_days(user_month.year(),user_month.month())) as u64;
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
        let mut result: Vec<Bson> = rbson::Array::new();
        // 统计有效的单元格（加上月尾的空白单元格）
        grid_count = table_line * 7;
        let query_sql = format!("call merge_plan({}, '{}')",organize,month);
        let compute_result_warp = primary_rbatis_pool!().fetch_decode::<Vec<PlanArchiveVO>>(query_sql.as_str(), vec![]).await;
        if compute_result_warp.is_err(){
            error!("在查询日期={}附近的计划安排时，发生异常:{}",month,compute_result_warp.unwrap_err());
            return Err(Error::from("查询计划安排异常"));
        }
        let rows = compute_result_warp.unwrap();
        // 对计划按照当月号数进行分组
        let mut map:HashMap<u64,Vec<PlanArchiveVO>> = HashMap::new();
        for item in rows {
            let day_number = item.id.unwrap();
            if map.contains_key(&day_number) {
                let mut list = map.get(&day_number).unwrap().to_vec();
                list.push(item);
                map.insert(day_number, list);
            }else {
                map.insert(day_number, vec![item]);
            }
        }
        for number in 1..grid_count {
            let mut day = rbson::Document::new();
            let mut plan: Vec<Bson> = rbson::Array::new();
            if number >= first_day && number <= (days + first_day - 1) {
                day.insert("flag", 1);
                day.insert("number", number - (first_day - 1));
                if map.contains_key(&number) {
                    // 本日有安排
                    let plans = map.get(&number).unwrap().to_vec();
                    for item in plans {
                        let mut value = rbson::Document::new();
                        value.insert("archive_time", item.archive_time);
                        value.insert("title", item.title);
                        value.insert("content", item.content);
                        plan.push(Bson::Document(value));
                    }
                    day.insert("value", plan);
                }
            }else {
                // 输出空白
                day.insert("flag", 0);
                day.insert("number", 0);
                day.insert("value", plan);
            }
            result.push(Bson::Document(day));
        }
        return Ok(result);
    }

}
