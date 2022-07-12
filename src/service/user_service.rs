use actix_multipart::Multipart;
use crate::error::Error;
use crate::error::Result;
use crate::service::CONTEXT;
use rbatis::DateTimeNative;
use rbatis::crud::{CRUD};
use crate::entity::vo::user::{UserOwnOrganizeVO, UserVO};
use crate::util::password_encoder::PasswordEncoder;
use actix_web::HttpRequest;
use futures_util::TryStreamExt;
use log::error;
use crate::util::options::OptionStringRefUnwrapOrDefault;
use crate::dao::log_mapper::LogMapper;
use crate::dao::user_mapper::UserMapper;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::dto::sign_in::SignInDTO;
use crate::entity::dto::user::{UserDTO, UserPageDTO};
use crate::entity::vo::jwt::JWTToken;
use crate::entity::vo::sign_in::SignInVO;
use crate::util::Page;
use crate::entity::domain::primary_database_tables::User;

/// 用户服务
pub struct UserService {}

 impl UserService {
    /// 用户分页
    pub async fn page(&self, arg: &UserPageDTO) -> Result<Page<UserVO>> {
        let mut extend = ExtendPageDTO{
            page_no: arg.page_no,
            page_size: arg.page_size,
            begin_time:arg.begin_time,
            end_time:arg.end_time
        };
        let count_result = UserMapper::select_count(&mut CONTEXT.primary_rbatis.as_executor(), &arg,&extend).await;
        if count_result.is_err(){
            error!("在用户分页统计时，发生异常:{}",count_result.unwrap_err());
            return Err(Error::from(format!("用户分页查询异常")));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from(format!("未查询到符合条件的数据")));
        }
        let mut result = Page::<UserVO>::page_query( total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no-1)*result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = UserMapper::select_page(&mut CONTEXT.primary_rbatis.as_executor(), &arg,&extend).await;
        if page_result.is_err() {
            error!("在用户分页获取页面数据时，发生异常:{}",page_result.unwrap_err());
            return Err(Error::from(format!("用户分页查询异常")));
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
    pub async fn add(&self, arg: &UserDTO) -> Result<u64> {
        let check_flag = arg.account.is_none() || arg.account.as_ref().unwrap().is_empty() || arg.name.is_none() || arg.name.as_ref().unwrap().is_empty() || arg.email.is_none() || arg.email.as_ref().unwrap().is_empty() || arg.phone.is_none() || arg.phone.as_ref().unwrap().is_empty() || arg.organize_id.is_none();
        if check_flag{
            return Err(Error::from("账号、姓名、手机号、邮箱以及所属组织不能为空!"));
        }
        let old_user = UserMapper::find_by_account(&CONTEXT.primary_rbatis,arg.account.as_ref().unwrap_or_default()).await?;
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
        let user = User{
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
            background:arg.background,
            organize_id: arg.organize_id,
            state: 1.into(),
            create_time: Some(rbatis::DateTimeNative::now()),
            update_time: None,
        };
        let write_result = CONTEXT.primary_rbatis.save(&user, &[]).await;
        if  write_result.is_err(){
            error!("创建账号时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from(format!("创建账号时，发生异常!")));
        }
        // 当前不允许创建用户操作
        // LogMapper::record_log(&CONTEXT.primary_rbatis,String::from(""));
        return Ok(write_result?.rows_affected);
    }

    ///登录后台
    pub async fn login(&self,req: &HttpRequest, arg: &SignInDTO) -> Result<SignInVO>  {
        if arg.account.is_none()
            || arg.account.as_ref().unwrap().is_empty()
            || arg.password.is_none()
            || arg.password.as_ref().unwrap().is_empty()
        {
            return Err(Error::from("账号和密码不能为空!"));
        }
        let user: Option<User> = CONTEXT
            .primary_rbatis
            .fetch_by_wrapper(CONTEXT.primary_rbatis.new_wrapper().eq(User::account(), &arg.account))
            .await?;
        let user = user.ok_or_else(|| Error::from(format!("账号:{} 不存在!", &arg.account.clone().unwrap())))?;
        // 判断用户是否被锁定，2为锁定
        if user.state.eq(&Some(0)) {
            return Err(Error::from("账户被禁用!"));
        }
        let mut error = None;
        if !PasswordEncoder::verify(
            user.password
                .as_ref()
                .ok_or_else(|| Error::from("错误的用户数据，密码为空!"))?,
            &arg.password.clone().unwrap(),
        ) {
            error = Some(Error::from("密码不正确!"));
        }
        if error.is_some(){
            // TODO 这里还应该设置失败锁
            return Err(error.unwrap())
        }
        let sign_in_vo = self.get_user_info(req,&user).await?;
        // 通过上面生成的token，完整记录日志
        let extract_result = &JWTToken::extract_token(&sign_in_vo.access_token);
        LogMapper::record_log_by_jwt(&CONTEXT.primary_rbatis,&extract_result.clone().unwrap(),String::from("OX001")).await;
        return Ok(sign_in_vo);
    }

     // 生成用户jwt并返回
     pub async fn get_user_info(&self,req: &HttpRequest, user: &User) -> Result<SignInVO> {
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
             city:String::from("云南西双版纳"),
             exp: DateTimeNative::now().timestamp() as usize,// 时间和校验的时候保持一致，统一秒
         };
         sign_vo.access_token = jwt_token.create_token(&CONTEXT.config.jwt_secret)?;
         return Ok(sign_vo);
     }


    /// 通过token获取用户信息
    pub async fn get_user_info_by_token(&self, req: &HttpRequest) -> Result<SignInVO> {
        let token = req.headers().get("access_token");
        let extract_result = &JWTToken::extract_token_by_header(token);
        match extract_result {
            Ok(token) => {
                let user: Option<User> = CONTEXT.primary_rbatis.fetch_by_wrapper(CONTEXT.primary_rbatis.new_wrapper().eq(User::account(), &token.account)).await?;
                let user = user.ok_or_else(|| Error::from(format!("账号:{} 不存在!", token.account)))?;
                return self.get_user_info(req,&user).await;
            }
            Err(e) => {
                return Err(crate::error::Error::from(e.to_string()));
            }
        }
    }

    /// 登出后台
    pub async fn logout(&self,req: &HttpRequest) {
        let token = req.headers().get("access_token");
        LogMapper::record_log_by_token(&CONTEXT.primary_rbatis,token,String::from("OX002")).await;
    }

    /// 修改用户信息
    pub async fn edit(&self,req: &HttpRequest, arg: &UserDTO) -> Result<u64> {
        let token = req.headers().get("access_token");
        if arg.account.is_none() || arg.account.as_ref().unwrap().is_empty() {
            return Err(Error::from("账号account不能为空!"));
        }
        // 首先判断要修改的用户是否存在
        let user_option: Option<User> = CONTEXT.primary_rbatis.fetch_by_wrapper(CONTEXT.primary_rbatis.new_wrapper().eq(User::account(), &arg.account)).await?;
        let user_exist = user_option.ok_or_else(|| Error::from(format!("用户:{} 不存在!", &arg.account.clone().unwrap())))?;

        let user_edit = User {
            account: user_exist.account,
            name: arg.name.clone(),
            password: if arg.password.is_some() { Some(PasswordEncoder::encode(arg.password.as_ref().unwrap()))}else { user_exist.password },
            sex: arg.sex.clone(),
            qq: arg.qq.clone(),
            email: arg.email.clone(),
            phone: arg.phone.clone(),
            birthday: arg.birthday.clone(),
            hometown: arg.hometown.clone(),
            autograph: arg.autograph.clone(),
            logo: arg.logo.clone(),
            background:arg.background,
            organize_id: arg.organize_id,
            state: arg.state,
            create_time: user_exist.create_time,
            update_time: Some(rbatis::DateTimeNative::now()),
        };
        let result = UserMapper::update_user(&mut CONTEXT.primary_rbatis.as_executor(),&user_edit).await;//CONTEXT.primary_rbatis.update_by_column(User::account(),&user_edit).await?;
        if result.is_err() {
            error!("在修改用户{}的信息时，发生异常:{}",arg.account.as_ref().unwrap(),result.unwrap_err());
            return Err(Error::from(format!("修改账户[{}]信息失败!", arg.account.as_ref().unwrap())));
        }
        LogMapper::record_log_by_token(&CONTEXT.primary_rbatis,token,String::from("OX003")).await;
        Ok(result.unwrap().rows_affected)
    }

    /// 删除用户
    pub async fn remove(&self, account: &str) -> Result<u64> {
        if account.is_empty() {
            return Err(Error::from("account 不能为空！"));
        }
        let r = CONTEXT.primary_rbatis.remove_by_column::<User, _>(User::account(), &account).await;
        return Ok(r?);
    }

    /// 用户详情
    pub async fn detail(&self, arg: &UserDTO) -> Result<UserVO> {
        let account = arg.account.clone().unwrap_or_default();
        let user = UserMapper::find_by_account(&CONTEXT.primary_rbatis,&account).await?.ok_or_else(|| Error::from(format!("用户:{} 不存在！", account)))?;
        let user_vo = UserVO::from(user);
        return Ok(user_vo);
    }

     /// 获取当前用户所在组织的用户列表
     pub async fn get_own_organize_user(&self, req: &HttpRequest) -> Result<Vec<UserOwnOrganizeVO>> {
         let token = req.headers().get("access_token");
         let extract_result = &JWTToken::extract_token_by_header(token);
         match extract_result {
             Ok(token) => {
                 let query_result = UserMapper::select_own_organize_user(&mut CONTEXT.primary_rbatis.as_executor(),&token.account).await;
                 if query_result.is_err() {
                     error!("在查询用户所属组织下的用户列表时，发生异常:{}",query_result.unwrap_err());
                     return Err(Error::from(format!("查询我所属组织的用户列表异常")));
                 }
                 return Ok(query_result.unwrap().unwrap());
             }
             Err(e) => {
                 return Err(crate::error::Error::from(e.to_string()));
             }
         }
     }

     pub async fn upload_logo(&self, mut payload: Multipart) -> Result<i32> {
         while let Some(mut field) = payload.try_next().await.unwrap() {
             let content_disposition = field.content_disposition();
             let aa = content_disposition.clone();
             println!("key:{:?}",aa.get_name().unwrap());
             println!("value:{:?}",field.headers());
         }
         return Ok(1);
     }

     // pub async fn upload_file(&self, mut payload: Multipart,arg: &web::Form<SignInDTO>) -> Result<HttpResponse> {
     //     println!("form-data:{:?}",arg);
     //     // iterate over multipart stream
     //     while let Some(mut field) = payload.try_next().await.unwrap() {
     //         // A multipart/form-data stream has to contain `content_disposition`
     //         let content_disposition = field.content_disposition().unwrap();
     //         let aa = content_disposition.clone();
     //
     //         println!("key:{:?}",aa.get_name().unwrap());
     //         println!("value:{:?}",field);
     //
     //         // let file_name = content_disposition.get_filename();
     //         // let warehouse_path = Path::new("./tmp/");
     //         // if !warehouse_path.exists(){
     //         //     match std::fs::create_dir_all(warehouse_path) {
     //         //         Ok(f) => {
     //         //             println!("created folder")
     //         //         },
     //         //         Err(err) => {
     //         //             println!("{:?}", err);
     //         //         }
     //         //     };
     //         // }
     //         //
     //         // let filepath = format!("{}{}",warehouse_path.to_str().unwrap(),file_name.unwrap());
     //         // println!("path{}",filepath);
     //         // // File::create is blocking operation, use threadpool
     //         // let mut f = web::block(|| std::fs::File::create(filepath)).await.unwrap();
     //         //
     //         // // Field in turn is stream of *Bytes* object
     //         // while let Some(chunk) = field.try_next().await.unwrap() {
     //         //     // filesystem operations are blocking, we have to use threadpool
     //         //     f = web::block(move || f.write_all(&chunk).map(|_| f)).await.unwrap();
     //         // }
     //     }
     //     Ok(HttpResponse::Ok().into())
     // }

}
