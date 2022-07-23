use std::borrow::Borrow;
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
use crate::dao::pictures_mapper::PicturesMapper;
use crate::entity::domain::business_database_tables::{Files, Pictures};
use crate::entity::domain::primary_database_tables::User;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::dto::picture_base64::Base64PictureDTO;
use crate::entity::dto::pictures::PicturesPageDTO;
use crate::entity::vo::jwt::JWTToken;
use crate::entity::vo::pictures::PicturesVO;
use crate::service::CONTEXT;
use crate::util;
use crate::util::date_time::DateTimeUtil;
use crate::util::Page;

extern crate base64;

pub struct FileService{}

impl FileService {

    /// 删除图片或壁纸
    pub async fn picture_delete(&self, req: &HttpRequest,id:u64) -> Result<u64>{
        let picture_op: Option<Pictures> = CONTEXT.business_rbatis.fetch_by_wrapper(CONTEXT.business_rbatis.new_wrapper().eq(Pictures::id(), &id)).await?;
        let picture = picture_op.ok_or_else(|| Error::from((format!("id为:{} 的图片或壁纸不存在!", id),util::NOT_EXIST)))?;
        // 判断文件是否存在，存在才删除
        if picture.web_url.is_some() && !picture.web_url.as_ref().unwrap().is_empty() {
            let file_path_str = format!("{}/{}", &CONTEXT.config.data_dir,picture.web_url.unwrap());
            let file_path = Path::new(&file_path_str);
            if file_path.exists(){
                std::fs::remove_file(file_path);
            }
        }
        let write_result = CONTEXT.business_rbatis.remove_by_column::<Pictures, _>(Pictures::id(), &id).await;
        if write_result.is_err(){
            error!("删除图片或壁纸时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("删除图片或壁纸失败!"));
        }
        let token = req.headers().get("access_token");
        LogMapper::record_log_by_token(&CONTEXT.primary_rbatis,token,String::from("OX012")).await;
        return Ok(write_result?);
    }

    /// 图片分页
    pub async fn pictures_page(&self, req: &HttpRequest, param: &PicturesPageDTO) -> Result<Page<PicturesVO>>  {
        let mut extend = ExtendPageDTO{
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time:param.begin_time,
            end_time:param.end_time
        };
        let mut arg= param.clone();
        if arg.organize.is_none() || arg.organize.as_ref().unwrap().len() == 0 {
            let token = req.headers().get("access_token");
            let extract_result = &JWTToken::extract_token_by_header(token);
            if extract_result.is_err() {
                log::error!("在获取用户信息时，发生异常:{}",extract_result.clone().unwrap_err().to_string());
                return Err(crate::error::Error::from(String::from("获取用户信息失败")));
            }
            let user_info = extract_result.clone().unwrap();
            arg.organize = Some(vec![user_info.account])
        }

        let count_result = PicturesMapper::select_count(&mut CONTEXT.business_rbatis.as_executor(), &arg,&extend).await;
        if count_result.is_err(){
            error!("在用户分页统计时，发生异常:{}",count_result.unwrap_err());
            return Err(Error::from("图片分页查询异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from(("未查询到符合条件的数据",util::NOT_EXIST)));
        }
        let mut result = Page::<PicturesVO>::page_query( total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no-1)*result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = PicturesMapper::select_page(&mut CONTEXT.business_rbatis.as_executor(), &arg,&extend).await;
        if page_result.is_err() {
            error!("在图片分页获取页面数据时，发生异常:{}",page_result.unwrap_err());
            return Err(Error::from("动态分页查询异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

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

    pub async fn upload_file_picture(&self,req: &HttpRequest,mut payload: Multipart) -> Result<String>{
        let token = req.headers().get("access_token");
        let extract_result = &JWTToken::extract_token_by_header(token);
        if extract_result.is_err() {
            error!("在获取用户信息时，发生异常:{}",extract_result.clone().unwrap_err().to_string());
            return Err(crate::error::Error::from(String::from("获取用户信息失败")));
        }
        let user_info = extract_result.clone().unwrap();
        // 首先判断要修改的用户是否存在
        let user_option: Option<User> = CONTEXT.primary_rbatis.fetch_by_wrapper(CONTEXT.primary_rbatis.new_wrapper().eq(User::account(), &user_info.account)).await?;
        let user_exist = user_option.ok_or_else(|| Error::from((format!("用户:{} 不存在!", &user_info.account.clone()),util::NOT_EXIST)))?;
        let today_op = DateTimeUtil::naive_date_time_to_str(&Some(NaiveDateTime::now().date()), util::FORMAT_YMD);
        let today = today_op.unwrap();
        let save_path = format!("{}/{}/{}/{}", &CONTEXT.config.data_dir,util::WALLPAPER_PATH ,&user_info.account.clone(),today.clone());
        let allow_picture = vec!["gif","png","jpg","jpeg","bmp"];
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
                let origin_name = content_disposition.get_filename();
                let origin_name_copy = format!("{}",origin_name.clone().unwrap());
                let lowercase_file_name =  origin_name.clone().unwrap().to_lowercase();
                let image_ext:Vec<&str> = lowercase_file_name.split(".").collect();
                if image_ext.len() < 2{
                    return Err(Error::from(("无效的图片!",util::NOT_PARAMETER)));
                }
                if !allow_picture.contains(&image_ext[1]){
                    return Err(Error::from(("请上传GIF、PNG、JPG、JPEG、BMP格式的图片!",util::NOT_PARAMETER)));
                }
                let file_name = format!("{}{}.{}", today.clone(),rand::thread_rng().gen_range(10000..=99999),&image_ext[1]);
                // 调用文件的保存接口
                let save_result = self.save_file(Path::new(&save_path),&file_name,field).await;
                if save_result.is_err() {
                    error!("在保存{}用户的图片时，发生异常:{}",&user_info.account,save_result.unwrap_err().to_string());
                    return Err(crate::error::Error::from(String::from("图片上传失败")));
                }
                let local_path = save_result.ok().clone();
                let http_path = local_path.clone().unwrap().replace(&CONTEXT.config.data_dir,&String::from(""));
                let picture = Pictures{
                    id: None,
                    category: Some(1),
                    file_name: Some(origin_name_copy),
                    descript: None,
                    file_url: local_path.clone(),
                    web_url: Some(http_path),
                    source: Some(user_info.account.clone()),
                    date: Some(rbatis::DateTimeNative::now())
                };
                let write_result = CONTEXT.business_rbatis.save(&picture, &[]).await;
                if  write_result.is_err(){
                    error!("保存图片时，发生异常:{}",write_result.unwrap_err());
                    return Err(Error::from("图片失败!"));
                }
                LogMapper::record_log_by_token(&CONTEXT.primary_rbatis,token,String::from("OX005")).await;
                return Ok(picture.web_url.unwrap());
            }
        }
        Err(crate::error::Error::from(String::from("图片上传失败")))
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

    pub async fn upload_file(&self, req: &HttpRequest,mut payload: Multipart) -> Result<String> {
        // 默认的一些回填值
        let mut uid = String::from("null");
        let file_belong_type = &String::from("complex");
        let mut origin_name_copy:String = String::new();
        let mut local_path:Option<String> = None;
        let mut file_belong_type:Option<&String> = None;

        let token = req.headers().get("access_token");
        let extract_result = &JWTToken::extract_token_by_header(token);
        if extract_result.is_err() {
            error!("在获取用户信息时，发生异常:{}",extract_result.clone().unwrap_err().to_string());
            return Err(crate::error::Error::from(String::from("获取用户信息失败")));
        }
        let user_info = extract_result.clone().unwrap();
        // 首先判断要修改的用户是否存在
        let user_option: Option<User> = CONTEXT.primary_rbatis.fetch_by_wrapper(CONTEXT.primary_rbatis.new_wrapper().eq(User::account(), &user_info.account)).await?;
        let user_exist = user_option.ok_or_else(|| Error::from((format!("用户:{} 不存在!", &user_info.account.clone()),util::NOT_EXIST)))?;
        let today_op = DateTimeUtil::naive_date_time_to_str(&Some(NaiveDateTime::now().date()), util::FORMAT_YMD);
        let today = today_op.unwrap();
        let save_path = format!("{}/{}/{}/{}", &CONTEXT.config.data_dir,util::DOCUMENT_PATH ,&user_info.account.clone(),today.clone());

        while let Some(item) = payload.next().await {
            let mut field = item.unwrap();
            let content_disposition = field.content_disposition();
            // 提取字段名
            let field_name_op = content_disposition.get_name();
            if field_name_op.is_empty() {
                continue;
            }
            let field_name = field_name_op.unwrap().to_string();
            println!("field name:{}",field_name);
            // 对待文件域字段单独处理上传
            match field_name == String::from("file") {
                true =>{
                    // 提取文件名
                    let origin_name = content_disposition.get_filename();
                    origin_name_copy = format!("{}",origin_name.clone().unwrap());
                    let lowercase_file_name =  origin_name.clone().unwrap().to_lowercase();
                    let image_ext:Vec<&str> = lowercase_file_name.split(".").collect();
                    if image_ext.len() < 2{
                        return Err(Error::from(("无效的文件!",util::NOT_PARAMETER)));
                    }
                    let file_belong_type_op = CONTEXT.config.file_type_map.get(image_ext[1]);
                    if file_belong_type_op.is_some() {
                        file_belong_type = file_belong_type_op.clone();
                    }
                    let file_name = format!("{}{}.{}", today.clone(),rand::thread_rng().gen_range(10000..=99999),&image_ext[1]);
                    // 调用文件的保存接口
                    let save_result = self.save_file(Path::new(&save_path),&file_name,field).await;
                    if save_result.is_err() {
                        error!("在保存{}用户的文件时，发生异常:{}",&user_info.account,save_result.unwrap_err().to_string());
                        return Err(crate::error::Error::from(String::from("文件上传失败")));
                    }
                    local_path = save_result.ok().clone();
                }
                _ => {
                    if field_name == String::from("uid"){
                        uid = String::new();
                        while let Some(chunk) = field.next().await {
                            uid = uid.add( std::str::from_utf8(&chunk.unwrap()).unwrap());
                        }
                    }
                }
            };
        }
        if local_path.is_none() {
            return Err(Error::from(("文件上传失败!",util::NOT_PARAMETER)));
        }
        let http_path = local_path.clone().unwrap().replace(&CONTEXT.config.data_dir,&String::from(""));
        let files = Files{
            id:None,
            uid:Some(uid),
            file_name: Some(origin_name_copy),
            file_url: Some(http_path),
            file_type: if file_belong_type.is_some() { Some(file_belong_type.unwrap().to_string())}else { Some(String::from("complex")) },
            source: Some(user_info.account.clone()),
            status: Some(String::from("2")),
            date: Some(rbatis::DateTimeNative::now())
        };
        let write_result = CONTEXT.business_rbatis.save(&files, &[]).await;
        if  write_result.is_err(){
            error!("保存文件时，发生异常:{}",write_result.unwrap_err());
            return Err(Error::from("保存文件失败!"));
        }
        LogMapper::record_log_by_token(&CONTEXT.primary_rbatis,token,String::from("OX013")).await;
        return Ok(files.file_url.unwrap());
    }

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