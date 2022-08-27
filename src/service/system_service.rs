use crate::error::Error;
use crate::error::Result;
use crate::service::CONTEXT;
use rbatis::DateTimeNative;
use rbatis::crud::{CRUD, CRUDMut};
use crate::entity::vo::user::{UserOwnOrganizeVO, UserVO};
use crate::util::password_encoder::PasswordEncoder;
use actix_web::HttpRequest;
use log::error;
use rbson::Bson;
use rust_decimal::prelude::ToPrimitive;
use crate::util::options::OptionStringRefUnwrapOrDefault;
use crate::dao::log_mapper::LogMapper;
use crate::dao::log_type_mapper::LogTypeMapper;
use crate::dao::user_mapper::UserMapper;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::dto::sign_in::SignInDTO;
use crate::entity::dto::user::{UserDTO, UserPageDTO};
use crate::entity::vo::jwt::JWTToken;
use crate::entity::vo::sign_in::SignInVO;
use crate::util::Page;
use crate::entity::domain::primary_database_tables::{Plan, PlanArchive, User};
use crate::entity::dto::log::LogPageDTO;
use crate::entity::dto::plan::PlanDTO;
use crate::entity::vo::log::LogVO;
use crate::entity::vo::log_type::LogTypeVO;
use crate::util;
use crate::util::date_time::DateUtils;

/// 系统服务
pub struct SystemService {}

impl SystemService {
    ///登录
    pub async fn login(&self, req: &HttpRequest, arg: &SignInDTO) -> Result<SignInVO> {
        if arg.account.is_none()
            || arg.account.as_ref().unwrap().is_empty()
            || arg.password.is_none()
            || arg.password.as_ref().unwrap().is_empty()
        {
            return Err(Error::from(("账号和密码不能为空!", util::NOT_PARAMETER)));
        }
        let user: Option<User> = CONTEXT
            .primary_rbatis
            .fetch_by_wrapper(CONTEXT.primary_rbatis.new_wrapper().eq(User::account(), &arg.account))
            .await?;
        let user = user.ok_or_else(|| Error::from((format!("账号:{} 不存在!", &arg.account.clone().unwrap()), util::NOT_EXIST)))?;
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
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis, &extract_result.clone().unwrap(), String::from("OX001")).await;
        return Ok(sign_in_vo);
    }

    /// 登出后台
    pub async fn logout(&self, req: &HttpRequest) {
        let token = req.headers().get("access_token");
        LogMapper::record_log_by_token(&CONTEXT.primary_rbatis, token, String::from("OX002")).await;
    }

    /// 用户分页
    pub async fn user_page(&self, arg: &UserPageDTO) -> Result<Page<UserVO>> {
        let mut extend = ExtendPageDTO {
            page_no: arg.page_no,
            page_size: arg.page_size,
            begin_time: arg.begin_time,
            end_time: arg.end_time,
        };
        let count_result = UserMapper::select_count(&mut CONTEXT.primary_rbatis.as_executor(), &arg, &extend).await;
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
        let page_result = UserMapper::select_page(&mut CONTEXT.primary_rbatis.as_executor(), &arg, &extend).await;
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
        let old_user = UserMapper::find_by_account(&CONTEXT.primary_rbatis, arg.account.as_ref().unwrap_or_default()).await?;
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
            create_time: Some(rbatis::DateTimeNative::now()),
            update_time: None,
        };
        let write_result = CONTEXT.primary_rbatis.save(&user, &[]).await;
        if write_result.is_err() {
            error!("创建账号时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("创建账号时，发生异常!"));
        }
        // 当前不允许创建用户操作
        // LogMapper::record_log(&CONTEXT.primary_rbatis,String::from(""));
        return Ok(write_result?.rows_affected);
    }

    /// 生成用户jwt并返回
    pub async fn user_get_info(&self, req: &HttpRequest, user: &User) -> Result<SignInVO> {
        //去除密码，增加安全性
        let mut user = user.clone();
        let ip = req.peer_addr().unwrap().ip().to_string();
        // if req.peer_addr().is_some() {
        //     ip = req.peer_addr().unwrap().ip().to_string();
        // } else if req.connection_info().remote_addr().is_some(){
        //     ip = req.connection_info().remote_addr().unwrap().parse().unwrap();
        // }else if req.connection_info().realip_remote_addr().is_some() {
        //     ip = req.connection_info().realip_remote_addr().unwrap().parse().unwrap()
        // }
        // println!("remote_addr{:?}",req.peer_addr().unwrap().ip().to_string());
        // println!("remote_addr{:?}",req.connection_info().remote_addr().unwrap());
        // println!("realip_remote_addr{:?}",req.connection_info().realip_remote_addr().unwrap());
        user.password = None;
        let mut sign_vo = SignInVO {
            user: Some(user.clone().into()),
            access_token: String::new(),
        };
        let jwt_token = JWTToken {
            account: user.account.unwrap_or_default(),
            name: user.name.clone().unwrap_or_default(),
            ip,
            organize:user.organize_id.unwrap(),
            city: String::from("云南西双版纳"),
            exp: DateTimeNative::now().timestamp() as usize,// 时间和校验的时候保持一致，统一秒
        };
        sign_vo.access_token = jwt_token.create_token(&CONTEXT.config.jwt_secret)?;
        return Ok(sign_vo);
    }

    /// 通过token获取用户信息
    pub async fn user_get_info_by_token(&self, req: &HttpRequest) -> Result<SignInVO> {
        let token = req.headers().get("access_token");
        let extract_result = &JWTToken::extract_token_by_header(token);
        match extract_result {
            Ok(token) => {
                let user: Option<User> = CONTEXT.primary_rbatis.fetch_by_wrapper(CONTEXT.primary_rbatis.new_wrapper().eq(User::account(), &token.account)).await?;
                let user = user.ok_or_else(|| Error::from((format!("账号:{} 不存在!", token.account), util::NOT_EXIST)))?;
                return self.user_get_info(req, &user).await;
            }
            Err(e) => {
                error!("在获取用户信息时，发生异常:{}",e.to_string());
                return Err(crate::error::Error::from(String::from("获取用户信息失败")));
            }
        }
    }

    /// 修改用户信息
    pub async fn user_edit(&self, req: &HttpRequest, arg: &UserDTO) -> Result<u64> {
        let token = req.headers().get("access_token");
        if arg.account.is_none() || arg.account.as_ref().unwrap().is_empty() {
            return Err(Error::from(("账号account不能为空!", util::NOT_PARAMETER)));
        }
        // 首先判断要修改的用户是否存在
        let user_option: Option<User> = CONTEXT.primary_rbatis.fetch_by_wrapper(CONTEXT.primary_rbatis.new_wrapper().eq(User::account(), &arg.account)).await?;
        let user_exist = user_option.ok_or_else(|| Error::from((format!("用户:{} 不存在!", &arg.account.clone().unwrap()), util::NOT_EXIST)))?;

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
            update_time: Some(rbatis::DateTimeNative::now()),
        };
        let result = UserMapper::update_user(&mut CONTEXT.primary_rbatis.as_executor(), &user_edit).await;//CONTEXT.primary_rbatis.update_by_column(User::account(),&user_edit).await?;
        if result.is_err() {
            error!("在修改用户{}的信息时，发生异常:{}",arg.account.as_ref().unwrap(),result.unwrap_err());
            return Err(Error::from(format!("修改账户[{}]信息失败!", arg.account.as_ref().unwrap())));
        }
        LogMapper::record_log_by_token(&CONTEXT.primary_rbatis, token, String::from("OX003")).await;
        Ok(result.unwrap().rows_affected)
    }

    /// 删除用户
    pub async fn user_remove(&self, account: &str) -> Result<u64> {
        if account.is_empty() {
            return Err(Error::from(("account 不能为空！", util::NOT_PARAMETER)));
        }
        let r = CONTEXT.primary_rbatis.remove_by_column::<User, _>(User::account(), &account).await;
        return Ok(r?);
    }

    /// 用户详情
    pub async fn user_detail(&self, arg: &UserDTO) -> Result<UserVO> {
        let account = arg.account.clone().unwrap_or_default();
        let user = UserMapper::find_by_account(&CONTEXT.primary_rbatis, &account).await?.ok_or_else(|| Error::from((format!("用户:{} 不存在！", account), util::NOT_EXIST)))?;
        let user_vo = UserVO::from(user);
        return Ok(user_vo);
    }

    /// 获取当前用户所在组织的用户列表
    pub async fn user_get_own_organize(&self, req: &HttpRequest) -> Result<Vec<UserOwnOrganizeVO>> {
        let token = req.headers().get("access_token");
        let extract_result = &JWTToken::extract_token_by_header(token);
        match extract_result {
            Ok(token) => {
                let query_result = UserMapper::select_own_organize_user(&mut CONTEXT.primary_rbatis.as_executor(), &token.account).await;
                if query_result.is_err() {
                    error!("在查询用户所属组织下的用户列表时，发生异常:{}",query_result.unwrap_err());
                    return Err(Error::from(format!("查询我所属组织的用户列表异常")));
                }
                return Ok(query_result.unwrap().unwrap());
            }
            Err(e) => {
                error!("在获取用户信息时，发生异常:{}",e.to_string());
                return Err(crate::error::Error::from(String::from("获取用户信息失败")));
            }
        }
    }

    /// 修改用户密码
    pub async fn user_update_password(&self, req: &HttpRequest, arg: &UserDTO) -> Result<u64> {
        let token = req.headers().get("access_token");
        if arg.password.is_none() || arg.password.as_ref().unwrap().is_empty() {
            return Err(Error::from(("密码不能为空!", util::NOT_PARAMETER)));
        }
        // 首先判断要修改的用户是否存在
        let user_option: Option<User> = CONTEXT.primary_rbatis.fetch_by_wrapper(CONTEXT.primary_rbatis.new_wrapper().eq(User::account(), &arg.account)).await?;
        let user_exist = user_option.ok_or_else(|| Error::from((format!("用户:{} 不存在!", &arg.account.clone().unwrap()), util::NOT_EXIST)))?;
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
            update_time: Some(rbatis::DateTimeNative::now()),
        };
        let result = UserMapper::update_user(&mut CONTEXT.primary_rbatis.as_executor(), &user_edit).await;//CONTEXT.primary_rbatis.update_by_column(User::account(),&user_edit).await?;
        if result.is_err() {
            error!("在修改用户{}的密码时，发生异常:{}",arg.account.as_ref().unwrap(),result.unwrap_err());
            return Err(Error::from(format!("修改账户[{}]密码失败!", arg.account.as_ref().unwrap())));
        }
        LogMapper::record_log_by_token(&CONTEXT.primary_rbatis, token, String::from("OX004")).await;
        Ok(result.unwrap().rows_affected)
    }

    /// 日志类别列表
    pub async fn log_get_type(&self) -> Result<Vec<LogTypeVO>> {
        let query_result = LogTypeMapper::select_all(&mut CONTEXT.primary_rbatis.as_executor()).await;
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
            begin_time:param.begin_time,
            end_time:param.end_time
        };
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let mut arg= param.clone();
        arg.organize = Some(user_info.organize);

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

    /// 计算近6个月的活跃情况
    pub async fn compute_pre6_logs(&self, req: &HttpRequest,month:&String) ->Result<rbson::Array> {
        // 按月查询统计账单并排序
        let user_info = JWTToken::extract_user_by_request(req).unwrap();
        let query_sql = format!("call count_pre6_logs({}, '{}')", &user_info.organize,month);
        let param:Vec<Bson> = Vec::new();
        let compute_result_warp = CONTEXT.primary_rbatis.fetch(query_sql.as_str(), param).await;
        if compute_result_warp.is_err(){
            error!("在统计近6个月的活跃情况时，发生异常:{}",compute_result_warp.unwrap_err());
            return Err(Error::from("统计近6个月的活跃情况异常"));
        }
        let rows:rbson::Array = compute_result_warp.unwrap();
        return Ok(rows);
    }

    /// 创建提醒事项
    pub async fn add_plan(&self, req: &HttpRequest,arg: &PlanDTO) -> Result<u64> {
        let check_flag = arg.standard_time.is_none() || arg.cycle.is_none() || arg.unit.is_none() || arg.content.is_none() || arg.content.as_ref().unwrap().is_empty();
        if check_flag{
            return Err(Error::from(("基准时间、重复执行周期、单位和内容不能为空!",util::NOT_PARAMETER)));
        }
        // 计算下次执行时间
        let last_exec_time = DateUtils::plan_data_compute(&arg.standard_time.clone().unwrap(),arg.cycle.unwrap(),arg.unit.unwrap());
        let user_info = JWTToken::extract_user_by_request(req).ok_or_else(|| Error::from(("获取用户信息失败，请登录",util::NOT_CHECKING)))?;
        let plan = Plan{
            id:None,
            standard_time: None,
            cycle: arg.cycle,
            unit: arg.unit,
            content: arg.content.clone(),
            last_exec_time: Some(last_exec_time),
            organize: Some(user_info.organize),
            user: Some(user_info.account.clone()),
            display: Some(1),
            create_time: Some(rbatis::DateTimeNative::now()),
            update_time: None
        };
        // 写入提醒事项
        let mut tx = CONTEXT.primary_rbatis.acquire_begin().await.unwrap();
        let add_plan_result = tx.save(&plan, &[]).await;
        if add_plan_result.is_err() {
            error!("在保存提醒事项时，发生异常:{}",add_plan_result.unwrap_err());
            tx.rollback();
            return Err(Error::from("保存提醒事项失败"));
        }
        let plan_id = add_plan_result.unwrap().last_insert_id;
        // 构造任务归档
        let plan_archive = PlanArchive{
            id: None,
            plan_id: plan_id.unwrap().to_u64(),
            status: Some(1),
            content: plan.content,
            archive_time: plan.last_exec_time,
            create_time: Some(rbatis::DateTimeNative::now()),
            update_time: None
        };
        // 预写入任务归档数据
        let add_plan_archive_result = tx.save(&plan_archive, &[]).await;
        if add_plan_archive_result.is_err() {
            error!("在保存流水时，发生异常:{}",add_plan_archive_result.unwrap_err());
            tx.rollback();
            return Err(Error::from("保存流水失败"));
        }
        // 所有的写入都成功，最后正式提交
        tx.commit().await;
        // TODO 创建一个定时调度任务 tokio-cron-scheduler
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&user_info,String::from("OX008")).await;
        return Ok(add_plan_archive_result?.rows_affected);
    }
}
