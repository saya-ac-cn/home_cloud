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
use crate::entity::dto::picture_base64::Base64PictureDTO;
use crate::entity::vo::jwt::JWTToken;
use crate::service::CONTEXT;
use crate::util;
use crate::util::date_time::DateTimeUtil;
extern crate base64;

pub struct FileService{}

impl FileService {

    /// 上传图片文件
    pub async fn upload_logo(&self,req: &HttpRequest,mut payload: Multipart) -> Result<String> {
        let token = req.headers().get("access_token");
        let extract_result = &JWTToken::extract_token_by_header(token);
        if extract_result.is_err() {
            error!("在获取用户信息时，发生异常:{}",extract_result.clone().unwrap_err().to_string());
            return Err(crate::error::Error::from(String::from("获取用户信息失败")));
        }
        let user_info = extract_result.clone().unwrap();
        // 首先判断要修改的用户是否存在
        let user_option: Option<User> = CONTEXT.primary_rbatis.fetch_by_wrapper(CONTEXT.primary_rbatis.new_wrapper().eq(User::account(), &user_info.account)).await?;
        let mut user_exist = user_option.ok_or_else(|| Error::from((format!("用户:{} 不存在!", &user_info.account.clone()),util::NOT_EXIST)))?;
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
        Ok(String::from("上传成功"))
    }

    /// 上传base64的图片
    /// 处理过程，将base64字符串进行切割（"data:image/jpeg;base64,/9j/4AAQSkZJRgABAQEBLAEsAAD/"），然后将后
    /// 半部分进行base64解码成byte数组保存到文件
    pub async fn upload_base64_picture(&self,req: &HttpRequest,arg:&Base64PictureDTO)-> Result<String>{
        if arg.content.is_none() || arg.content.as_ref().unwrap().is_empty(){
            return Err(Error::from(("请选择图片!",util::NOT_PARAMETER)));
        }
        let base64_picture = arg.content.clone().unwrap();
        let image_arr:Vec<&str> = base64_picture.split(",").collect();
        if image_arr.len() < 2{
            return Err(Error::from(("无效的图片!",util::NOT_PARAMETER)));
        }
        if !image_arr[0].starts_with("data:image"){
            return Err(Error::from(("非法的Base64图片!",util::NOT_PARAMETER)));
        }
        let token = req.headers().get("access_token");
        let extract_result = &JWTToken::extract_token_by_header(token);
        if extract_result.is_err() {
            error!("在获取用户信息时，发生异常:{}",extract_result.clone().unwrap_err().to_string());
            return Err(crate::error::Error::from(String::from("获取用户信息失败")));
        }
        let user_info = extract_result.clone().unwrap();
        let today_op = DateTimeUtil::naive_date_time_to_str(&Some(NaiveDateTime::now().date()), util::FORMAT_YMD);
        let today = today_op.unwrap();
        let file_name = format!("{}{}.png", today.clone(),rand::thread_rng().gen_range(10000..=99999));
        let save_path = format!("{}/{}/{}/{}", &CONTEXT.config.data_dir,util::ILLUSTRATED_PATH ,&user_info.account.clone(),today.clone());
        let save_result = self.save_base64(Path::new(&save_path),image_arr[1],&file_name).await;
        if save_result.is_err() {
            error!("在保存base64图片时，发生异常:{}",save_result.unwrap_err().to_string());
            return Err(crate::error::Error::from(String::from("保存base64图片")));
        }
        let local_path = save_result.ok().clone();
        let http_path = local_path.unwrap().replace(&CONTEXT.config.data_dir,&String::from(""));
        Ok(http_path)
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

    /// 保存base64的图片
    pub async fn save_base64(&self,save_path:&Path,content: &str, file_name:&String) -> Result<String>{
        let path_check_result = self.mkdir_if_not_exists(save_path).await;
        if path_check_result.is_err() {
            return Err(path_check_result.unwrap_err());
        }
        let today_op = DateTimeUtil::naive_date_time_to_str(&Some(NaiveDateTime::now().date()), util::FORMAT_YMD);
        let today = today_op.unwrap();
        let file_path = format!("{}/{}", save_path.to_str().unwrap(), file_name);
        let result = Ok(file_path.clone());
        let mut f = web::block(|| std::fs::File::create(file_path)).await.unwrap().unwrap();
        let bytes = base64::decode(content).unwrap();
        f = web::block(move || f.write_all(bytes.as_slice()).map(|_| f)).await.unwrap().unwrap();
        return result;
    }

    // 保存文件
    pub async fn save_file(&self,save_path:&Path, file_name:&String, mut field: Field) -> Result<String>{
        let path_check_result = self.mkdir_if_not_exists(save_path).await;
        if path_check_result.is_err() {
            return Err(path_check_result.unwrap_err());
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

    /// 创建目录（不存在的情况下）
    pub async fn mkdir_if_not_exists(&self,save_path:&Path) -> Result<bool>{
        if !save_path.exists(){
            let create_result = std::fs::create_dir_all(save_path);
            if create_result.is_err() {
                error!("create folder fail,cause by:{:?}",create_result.unwrap_err());
                return Err(Error::from(("create folder fail",util::FILE_IO_ERROR)));
            }
        }
        Ok(true)
    }

}