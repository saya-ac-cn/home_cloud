use std::io::Write;
use std::ops::Add;
use std::path::Path;
use actix_multipart::{Field, Multipart};
use actix_web::{HttpRequest, HttpResponse, web};
use chrono::NaiveDateTime;
use crate::error::Error;
use crate::error::Result;
use futures_util::{StreamExt, TryStreamExt};
use crate::util::string::IsEmptyString;
use log::error;
use rand::Rng;
use rbatis::crud::CRUD;
use rbatis::value::DateTimeNow;
use crate::dao::log_mapper::LogMapper;
use crate::entity::domain::primary_database_tables::User;
use crate::entity::vo::jwt::JWTToken;
use crate::service::CONTEXT;
use crate::util;
use crate::util::date_time::DateTimeUtil;

pub struct FileService{}

impl FileService {

    pub async fn upload_logo(&self,req: &HttpRequest,mut payload: Multipart) -> Result<HttpResponse> {
        let token = req.headers().get("access_token");
        let extract_result = &JWTToken::extract_token_by_header(token);
        if extract_result.is_err() {
            error!("在获取用户信息时，发生异常:{}",extract_result.clone().unwrap_err().to_string());
            return Err(crate::error::Error::from(String::from("获取用户信息失败")));
        }
        let user_info = extract_result.clone().unwrap();
        // 首先判断要修改的用户是否存在
        let user_option: Option<User> = CONTEXT.primary_rbatis.fetch_by_wrapper(CONTEXT.primary_rbatis.new_wrapper().eq(User::account(), &user_info.account)).await?;
        let mut user_exist = user_option.ok_or_else(|| Error::from(format!("用户:{} 不存在!", &user_info.account.clone())))?;
        let today_op = DateTimeUtil::naive_date_time_to_str(&Some(NaiveDateTime::now().date()), util::FORMAT_YMD);
        let today = today_op.unwrap();
        let save_path = format!("{}/{}/{}/{}/{}{}", &CONTEXT.config.data_dir,util::LOGO_PATH ,&user_info.account.clone(),today.clone(),today.clone(),rand::thread_rng().gen_range(10000..=99999));
        while let Some(item) = payload.next().await {
            let mut field = item.unwrap();
            let content_disposition = field.content_disposition();
            // 提取字段名
            let field_name_op = content_disposition.get_name();
            if field_name_op.is_empty() {
                continue;
            }
            let field_name = field_name_op.unwrap().to_string();
            // 对待文件域字段单独处理上传
            if field_name == String::from("file")  {
                // 提取文件名
                let file_name = content_disposition.get_filename();
                // 调用文件的保存接口
                let save_result = self.save_file(Path::new(&save_path),&file_name.unwrap().to_string(),field).await;
                if save_result.is_err() {
                    error!("在保存{}用户的头像时，发生异常:{}",&user_info.account,save_result.unwrap_err().to_string());
                    return Err(crate::error::Error::from(String::from("修改头像失败")));
                }
                let local_path = save_result.ok().clone();
                let http_path = local_path.unwrap().replace(&CONTEXT.config.data_dir,&String::from(""));
                user_exist.logo = Some(http_path);
                user_exist.update_time = Some(rbatis::DateTimeNative::now());
                CONTEXT.primary_rbatis.update_by_column(User::account(), &mut user_exist).await?;
                LogMapper::record_log_by_token(&CONTEXT.primary_rbatis,token,String::from("OX005")).await;
            }
        }
        Ok(HttpResponse::Ok().into())
    }

    // pub async fn upload_logo2(&self, mut payload: Multipart) -> Result<HttpResponse> {
    //     while let Some(item) = payload.next().await {
    //         let mut field = item.unwrap();
    //         let content_disposition = field.content_disposition();
    //         // 提取字段名
    //         let field_name_op = content_disposition.get_name();
    //         if field_name_op.is_empty() {
    //             continue;
    //         }
    //         let field_name = field_name_op.unwrap().to_string();
    //         println!("field name:{}",field_name);
    //         // 对待文件域字段单独处理上传
    //         match field_name == String::from("file") {
    //             true =>{
    //                 // 提取文件名
    //                 let file_name = content_disposition.get_filename();
    //                 println!("file字段类型，文件名{:?}",file_name);
    //                 // 调用文件的保存接口
    //                 let save_result = self.save_file(&file_name.unwrap().to_string(),field).await;
    //                 if save_result.is_err() {
    //                     //save_result.unwrap_err()
    //                 }
    //                 println!("保存路径{:?}",save_result.unwrap());
    //             }
    //             _ => {
    //                 let mut field_value = String::new();
    //                 while let Some(chunk) = field.next().await {
    //                     field_value = field_value.add( std::str::from_utf8(&chunk.unwrap()).unwrap());
    //                 }
    //                 println!("field value:{:?}",field_value);
    //             }
    //         };
    //     }
    //     Ok(HttpResponse::Ok().into())
    // }

    pub async fn save_file(&self,save_path:&Path, file_name:&String, mut field: Field) -> Result<String>{
        if !save_path.exists(){
            let create_result = std::fs::create_dir_all(save_path);
            if create_result.is_err() {
                error!("create folder fail,cause by:{:?}",create_result.unwrap_err());
                return Err(Error::from("create folder fail"));
            }
        }
        let file_path = format!("{}/{}", save_path.to_str().unwrap(), file_name);
        let result = Ok(file_path.clone());
        let mut f = web::block(|| std::fs::File::create(file_path)).await.unwrap().unwrap();
        while let Some(chunk) = field.try_next().await.unwrap() {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await.unwrap().unwrap();
        }
        return result;
    }

}