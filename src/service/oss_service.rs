use crate::entity::dto::files::{FilesDTO, FilesPageDTO};
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::dto::picture_base64::Base64PictureDTO;
use crate::entity::dto::pictures::PicturesPageDTO;
use crate::dao::files_mapper::FilesMapper;
use crate::dao::log_mapper::LogMapper;
use crate::dao::pictures_mapper::PicturesMapper;
use crate::entity::table::{Files, Pictures, User};
use crate::entity::vo::files::FilesVO;
use crate::entity::vo::pictures::PicturesVO;
use crate::config::user_context::UserContext;
use crate::config::CONTEXT;
use crate::util::date_time::{DateTimeUtil, DateUtils};
use crate::util::error::{Error, Result};
use crate::util::string::IsEmptyString;
use crate::util::token_util::TokenUtils;
use crate::util::Page;
use crate::{business_rbatis_pool, primary_rbatis_pool, util};
use actix_http::StatusCode;
use actix_multipart::{Field, Multipart};
use actix_web::web::BufMut;
use actix_web::{web, HttpRequest, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use log::error;
use rustflake::Snowflake;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::Add;
use std::path::Path;

extern crate base64;

pub struct OssService {}

impl OssService {
    /// 文件下载
    pub async fn files_download(&self, id: u64) -> HttpResponse {
        let mut response = HttpResponse::Ok();
        let result_wrap = Files::select_by_id(business_rbatis_pool!(), &id).await;
        if result_wrap.is_err() {
            error!("在获取文件数据时，发生异常:{}", result_wrap.unwrap_err());
            response.status(StatusCode::NOT_FOUND);
            return response.finish();
        }
        let files_wrap = result_wrap.unwrap().into_iter().next();
        if files_wrap.is_none() {
            response.status(StatusCode::NOT_FOUND);
            return response.finish();
        }
        let files = files_wrap.unwrap();
        // 判断文件是否存在，存在才下载
        if files.file_url.is_none() || files.file_url.as_ref().unwrap().is_empty() {
            response.status(StatusCode::NOT_FOUND);
            return response.finish();
        }
        let file_path_str = format!("{}/{}", &CONTEXT.config.data_dir, files.file_url.unwrap());
        let file_path = Path::new(&file_path_str);
        if !file_path.exists() {
            response.status(StatusCode::NOT_FOUND);
            return response.finish();
        }

        let mut file = File::open(file_path).unwrap();
        let mut buffer = [0u8; 4096];
        let mut bytes: Vec<u8> = Vec::new();
        loop {
            let read_count = file.read(&mut buffer).unwrap();
            bytes.put_slice(&buffer[..read_count]);
            if read_count < buffer.len() {
                break;
            }
        }

        let body = actix_web::web::Bytes::from(bytes);
        //且仅当此对象抽象路径名表示的文件或目录存在时，返回true
        response.content_type("application/x-download");
        //控制下载文件的名字（乱码参考：https://www.iefans.net/xiazai-wenjian-http-bianma-content-disposition/）
        // let content_disposition = format!("attachment;filename*=utf-8''{}",urlencoding::encode(&files.file_name.unwrap()));
        let content_disposition =
            format!("attachment;filename*=utf-8''{}", &files.file_name.unwrap());
        response.insert_header((
            actix_web::http::header::CONTENT_DISPOSITION,
            content_disposition,
        ));
        response.body(body)
    }

    /// 修改文件
    pub async fn files_update(&self, req: &HttpRequest, arg: &FilesDTO) -> Result<u64> {
        TokenUtils::check_token(arg.token.clone())
            .await
            .ok_or_else(|| Error::from(util::TOKEN_ERROR_CODE))?;
        if arg.id.is_none() || arg.status.is_none() {
            return Err(Error::from(("文件id不能为空!", util::NOT_PARAMETER_CODE)));
        }
        let user_info = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;

        let query_file_wrap = Files::select_by_id_and_organize(
            business_rbatis_pool!(),
            &arg.id.clone().unwrap(),
            &user_info.organize,
        )
        .await;
        if query_file_wrap.is_err() {
            error!("查询文件异常：{}", query_file_wrap.unwrap_err());
            return Err(Error::from("查询文件失败!"));
        }
        let files_option = query_file_wrap.unwrap().into_iter().next();
        let files_exist = files_option.ok_or_else(|| {
            Error::from((
                format!("id={} 的文件不存在!", &arg.id.clone().unwrap()),
                util::NOT_EXIST_CODE,
            ))
        })?;
        let files = Files {
            id: arg.id,
            uid: None,
            file_name: None,
            file_url: None,
            file_type: None,
            organize: files_exist.organize,
            source: Some(user_info.account.clone()),
            status: arg.status,
            create_time: None,
            update_time: DateTimeUtil::naive_date_time_to_str(
                &Some(DateUtils::now()),
                &util::FORMAT_Y_M_D_H_M_S,
            ),
        };
        let result = FilesMapper::update_files(business_rbatis_pool!(), &files).await;
        if result.is_err() {
            error!(
                "在修改id={}的文件时，发生异常:{}",
                arg.id.as_ref().unwrap(),
                result.unwrap_err()
            );
            return Err(Error::from("文件修改失败"));
        }
        LogMapper::record_log_by_context(primary_rbatis_pool!(), &user_info, String::from("OX015"))
            .await;
        return Ok(result?.rows_affected);
    }

    /// 删除文件
    pub async fn files_delete(&self, req: &HttpRequest, arg: &FilesDTO) -> Result<u64> {
        TokenUtils::check_token(arg.token.clone())
            .await
            .ok_or_else(|| Error::from(util::TOKEN_ERROR_CODE))?;
        let user_info = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
        // 只能查看自己组织机构下的数据
        let mut query_where = arg.clone();
        query_where.organize = Some(user_info.organize);

        let result_wrap = FilesMapper::select_one(business_rbatis_pool!(), &query_where).await;
        if result_wrap.is_err() {
            error!("在获取文件数据时，发生异常:{}", result_wrap.unwrap_err());
            return Err(Error::from("删除文件失败!"));
        }
        let files_op = result_wrap.unwrap();
        if files_op.is_none() {
            return Err(Error::from("删除文件失败!"));
        }
        let files = files_op.unwrap();

        // 判断文件是否存在，存在才删除
        if files.file_url.is_some() && !files.file_url.as_ref().unwrap().is_empty() {
            let file_path_str = format!("{}/{}", &CONTEXT.config.data_dir, files.file_url.unwrap());
            let file_path = Path::new(&file_path_str);
            if file_path.exists() {
                std::fs::remove_file(file_path);
            }
        }
        let write_result =
            Files::delete_by_id(business_rbatis_pool!(), &files.id.clone().unwrap()).await;
        if write_result.is_err() {
            error!("删除文件时，发生异常:{}", write_result.unwrap_err());
            return Err(Error::from("删除文件失败!"));
        }

        LogMapper::record_log_by_context(primary_rbatis_pool!(), &user_info, String::from("OX014"))
            .await;
        return Ok(write_result?.rows_affected);
    }

    /// 文件分页
    pub async fn files_page(
        &self,
        req: &HttpRequest,
        organize: Option<u64>,
        param: &FilesPageDTO,
    ) -> Result<Page<FilesVO>> {
        let mut extend = ExtendPageDTO {
            page_no: param.page_no,
            page_size: param.page_size,
            begin_time: param.begin_time.clone(),
            end_time: param.end_time.clone(),
        };
        let mut arg = param.clone();
        if organize.is_none() {
            let user_info = UserContext::extract_user_by_request(req)
                .await
                .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
            // 用户只能看到自己组织下的数据
            arg.organize = Some(user_info.organize);
        } else {
            arg.organize = organize;
        }
        let count_result = FilesMapper::select_count(business_rbatis_pool!(), &arg, &extend).await;
        if count_result.is_err() {
            error!("在文件分页统计时，发生异常:{}", count_result.unwrap_err());
            return Err(Error::from("文件分页查询异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from((
                "未查询到符合条件的数据",
                util::NOT_EXIST_CODE,
            )));
        }
        let mut result = Page::<FilesVO>::page_query(total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no - 1) * result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = FilesMapper::select_page(business_rbatis_pool!(), &arg, &extend).await;
        if page_result.is_err() {
            error!(
                "在文件分页获取页面数据时，发生异常:{}",
                page_result.unwrap_err()
            );
            return Err(Error::from("文件分页查询异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

    /// 删除图片或壁纸
    pub async fn picture_delete(&self, req: &HttpRequest, id: u64) -> Result<u64> {
        let user_info = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
        // 只能查看自己组织机构下的数据

        let query_picture_wrap =
            Pictures::select_by_id_and_organize(business_rbatis_pool!(), &id, &user_info.organize)
                .await;
        if query_picture_wrap.is_err() {
            error!("查询图片异常：{}", query_picture_wrap.unwrap_err());
            return Err(Error::from("查询图片失败!"));
        }
        let picture_op = query_picture_wrap.unwrap().into_iter().next();
        let picture = picture_op.ok_or_else(|| {
            Error::from((
                format!("id为:{} 的图片或壁纸不存在!", id),
                util::NOT_EXIST_CODE,
            ))
        })?;
        // 判断文件是否存在，存在才删除
        if picture.web_url.is_some() && !picture.web_url.as_ref().unwrap().is_empty() {
            let file_path_str =
                format!("{}/{}", &CONTEXT.config.data_dir, picture.web_url.unwrap());
            let file_path = Path::new(&file_path_str);
            if file_path.exists() {
                std::fs::remove_file(file_path);
            }
        }
        let write_result = Pictures::delete_by_id(business_rbatis_pool!(), &id).await;
        if write_result.is_err() {
            error!("删除图片或壁纸时，发生异常:{}", write_result.unwrap_err());
            return Err(Error::from("删除图片或壁纸失败!"));
        }

        LogMapper::record_log_by_context(primary_rbatis_pool!(), &user_info, String::from("OX012"))
            .await;
        return Ok(write_result?.rows_affected);
    }

    /// 图片分页
    pub async fn pictures_page(
        &self,
        req: &HttpRequest,
        param: &PicturesPageDTO,
    ) -> Result<Page<PicturesVO>> {
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
        let count_result =
            PicturesMapper::select_count(business_rbatis_pool!(), &arg, &extend).await;
        if count_result.is_err() {
            error!("在用户分页统计时，发生异常:{}", count_result.unwrap_err());
            return Err(Error::from("图片分页查询异常"));
        }
        let total_row = count_result.unwrap().unwrap();
        if total_row <= 0 {
            return Err(Error::from((
                "未查询到符合条件的数据",
                util::NOT_EXIST_CODE,
            )));
        }
        let mut result = Page::<PicturesVO>::page_query(total_row, &extend);
        // 重新设置limit起始位置
        extend.page_no = Some((result.page_no - 1) * result.page_size);
        extend.page_size = Some(result.page_size);
        let page_result = PicturesMapper::select_page(business_rbatis_pool!(), &arg, &extend).await;
        if page_result.is_err() {
            error!(
                "在图片分页获取页面数据时，发生异常:{}",
                page_result.unwrap_err()
            );
            return Err(Error::from("动态分页查询异常"));
        }
        let page_rows = page_result.unwrap();
        result.records = page_rows;
        return Ok(result);
    }

    /// 修改用户头像
    pub async fn upload_logo(&self, req: &HttpRequest, arg: &Base64PictureDTO) -> Result<String> {
        let user_info = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
        // 首先判断要修改的用户是否存在
        let query_user_wrap =
            User::select_by_account(primary_rbatis_pool!(), &user_info.account).await;
        if query_user_wrap.is_err() {
            error!("查询用户异常：{}", query_user_wrap.unwrap_err());
            return Err(Error::from("查询用户失败!"));
        }
        let user_option = query_user_wrap.unwrap().into_iter().next();
        let mut user_exist = user_option.ok_or_else(|| {
            Error::from((
                format!("用户:{} 不存在!", &user_info.account.clone()),
                util::NOT_EXIST_CODE,
            ))
        })?;
        if arg.content.is_none() || arg.content.as_ref().unwrap().is_empty() {
            return Err(Error::from(("请选择base64图片!", util::NOT_PARAMETER_CODE)));
        }
        let base64_picture = arg.content.clone().unwrap();
        let image_arr: Vec<&str> = base64_picture.split(",").collect();
        if image_arr.len() < 2 {
            return Err(Error::from(("无效的图片!", util::NOT_PARAMETER_CODE)));
        }
        if !image_arr[0].starts_with("data:image") {
            return Err(Error::from(("非法的Base64图片!", util::NOT_PARAMETER_CODE)));
        }
        let today_op =
            DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()), util::FORMAT_YMD);
        let today = today_op.unwrap();
        let file_name = format!("{}{}.png", today.clone(), &Snowflake::default().generate());
        let save_path = format!(
            "{}/{}/{}/{}",
            &CONTEXT.config.data_dir,
            util::LOGO_PATH,
            &user_info.account.clone(),
            today.clone()
        );
        let save_result = self
            .save_base64(Path::new(&save_path), image_arr[1], &file_name)
            .await;
        if save_result.is_err() {
            error!(
                "在保存用户的base64头像时，发生异常:{}",
                save_result.unwrap_err().to_string()
            );
            return Err(Error::from(String::from("头像保存失败")));
        }
        let local_path = save_result.ok().clone();
        // 去除基准路径
        let http_path = local_path
            .clone()
            .unwrap()
            .replace(&CONTEXT.config.data_dir, &String::from(""));
        let view_path = format!("/{}{}", &util::PUBLIC_VIEW_ROOT_PATH, http_path);

        user_exist.logo = Some(view_path);
        user_exist.update_time = DateTimeUtil::naive_date_time_to_str(
            &Some(DateUtils::now()),
            &util::FORMAT_Y_M_D_H_M_S,
        );

        let write_result =
            User::update_by_column(primary_rbatis_pool!(), &user_exist, "account").await;
        if write_result.is_err() {
            error!("在修改用户头像时，发生异常:{}", write_result.unwrap_err());
            return Err(Error::from("修改用户头像失败!"));
        }
        LogMapper::record_log_by_context(primary_rbatis_pool!(), &user_info, String::from("OX005"))
            .await;
        Ok(user_exist.logo.unwrap())
    }

    /// 上传文件类型的图片
    pub async fn upload_file_picture(
        &self,
        req: &HttpRequest,
        mut payload: Multipart,
    ) -> Result<String> {
        let user_info = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
        // 首先判断要修改的用户是否存在
        // 首先判断要修改的用户是否存在
        let query_user_wrap =
            User::select_by_account(primary_rbatis_pool!(), &user_info.account).await;
        if query_user_wrap.is_err() {
            error!("查询用户异常：{}", query_user_wrap.unwrap_err());
            return Err(Error::from("查询用户失败!"));
        }
        let user_option = query_user_wrap.unwrap().into_iter().next();
        user_option.ok_or_else(|| {
            Error::from((
                format!("用户:{} 不存在!", &user_info.account.clone()),
                util::NOT_EXIST_CODE,
            ))
        })?;
        let today_op =
            DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()), util::FORMAT_YMD);
        let today = today_op.unwrap();
        let save_path = format!(
            "{}/{}/{}/{}",
            &CONTEXT.config.data_dir,
            util::WALLPAPER_PATH,
            &user_info.account.clone(),
            today.clone()
        );
        let allow_picture = vec!["gif", "png", "jpg", "jpeg", "bmp"];
        while let Some(item) = payload.next().await {
            let field = item.unwrap();
            let content_disposition = field.content_disposition();
            // 提取字段名
            let field_name_op = content_disposition.get_name();
            if field_name_op.is_empty() {
                continue;
            }
            let field_name = field_name_op.unwrap().to_string();
            // 对待文件域字段单独处理上传
            if field_name == String::from("file") {
                // 提取文件名
                let origin_name = content_disposition.get_filename();
                let origin_name_copy = format!("{}", origin_name.clone().unwrap());
                let lowercase_file_name = origin_name.clone().unwrap().to_lowercase();
                let image_ext: Vec<&str> = lowercase_file_name.split(".").collect();
                if image_ext.len() < 2 {
                    return Err(Error::from(("无效的图片!", util::NOT_PARAMETER_CODE)));
                }
                if !allow_picture.contains(&image_ext[image_ext.len() - 1]) {
                    return Err(Error::from((
                        "请上传GIF、PNG、JPG、JPEG、BMP格式的图片!",
                        util::NOT_PARAMETER_CODE,
                    )));
                }
                let file_name = format!(
                    "{}{}.{}",
                    today.clone(),
                    &Snowflake::default().generate(),
                    &image_ext[image_ext.len() - 1]
                );
                // 调用文件的保存接口
                let save_result = self
                    .save_file(Path::new(&save_path), &file_name, field)
                    .await;
                if save_result.is_err() {
                    error!(
                        "在保存{}用户的图片时，发生异常:{}",
                        &user_info.account,
                        save_result.unwrap_err().to_string()
                    );
                    return Err(Error::from(String::from("图片上传失败")));
                }
                let local_path = save_result.ok().clone();
                let http_path = local_path
                    .clone()
                    .unwrap()
                    .replace(&CONTEXT.config.data_dir, &String::from(""));
                // 去除基准路径
                let view_path = format!("/{}{}", &util::PUBLIC_VIEW_ROOT_PATH, http_path);

                let picture = Pictures {
                    id: None,
                    category: Some(1),
                    file_name: Some(origin_name_copy),
                    descript: None,
                    file_url: local_path.clone(),
                    web_url: Some(view_path),
                    organize: Some(user_info.organize),
                    source: Some(user_info.account.clone()),
                    create_time: DateTimeUtil::naive_date_time_to_str(
                        &Some(DateUtils::now()),
                        &util::FORMAT_Y_M_D_H_M_S,
                    ),
                    update_time: None,
                };
                let write_result = Pictures::insert(business_rbatis_pool!(), &picture).await;
                if write_result.is_err() {
                    error!("保存图片时，发生异常:{}", write_result.unwrap_err());
                    return Err(Error::from("图片失败!"));
                }
                LogMapper::record_log_by_context(
                    primary_rbatis_pool!(),
                    &user_info,
                    String::from("OX005"),
                )
                .await;
                return Ok(picture.web_url.unwrap());
            }
        }
        Err(Error::from(String::from("图片上传失败")))
    }

    /// 上传base64类型的图片
    /// 处理过程，将base64字符串进行切割（"data:image/jpeg;base64,/9j/4AAQSkZJRgABAQEBLAEsAAD/"），然后将后
    /// 半部分进行base64解码成byte数组保存到文件
    pub async fn upload_base64_picture(
        &self,
        req: &HttpRequest,
        arg: &Base64PictureDTO,
    ) -> Result<String> {
        if arg.content.is_none() || arg.content.as_ref().unwrap().is_empty() {
            return Err(Error::from(("请选择图片!", util::NOT_PARAMETER_CODE)));
        }
        let base64_picture = arg.content.clone().unwrap();
        let image_arr: Vec<&str> = base64_picture.split(",").collect();
        if image_arr.len() < 2 {
            return Err(Error::from(("无效的图片!", util::NOT_PARAMETER_CODE)));
        }
        if !image_arr[0].starts_with("data:image") {
            return Err(Error::from(("非法的Base64图片!", util::NOT_PARAMETER_CODE)));
        }
        let user_info = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
        let today_op =
            DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()), util::FORMAT_YMD);
        let today = today_op.unwrap();
        let file_name = format!("{}{}.png", today.clone(), &Snowflake::default().generate());
        let save_path = format!(
            "{}/{}/{}/{}",
            &CONTEXT.config.data_dir,
            util::ILLUSTRATED_PATH,
            &user_info.account.clone(),
            today.clone()
        );
        let save_result = self
            .save_base64(Path::new(&save_path), image_arr[1], &file_name)
            .await;
        if save_result.is_err() {
            error!(
                "在保存base64图片时，发生异常:{}",
                save_result.unwrap_err().to_string()
            );
            return Err(Error::from(String::from("保存base64图片")));
        }
        let local_path = save_result.ok().clone();
        // 去除基准路径
        let http_path = local_path
            .clone()
            .unwrap()
            .replace(&CONTEXT.config.data_dir, &String::from(""));
        let view_path = format!("/{}{}", &util::PUBLIC_VIEW_ROOT_PATH, http_path);

        let picture = Pictures {
            id: None,
            category: Some(2),
            file_name: Some(file_name),
            descript: None,
            file_url: local_path.clone(),
            web_url: Some(view_path),
            organize: Some(user_info.organize),
            source: Some(user_info.account.clone()),
            create_time: DateTimeUtil::naive_date_time_to_str(
                &Some(DateUtils::now()),
                &util::FORMAT_Y_M_D_H_M_S,
            ),
            update_time: None,
        };
        let write_result = Pictures::insert(business_rbatis_pool!(), &picture).await;
        if write_result.is_err() {
            error!("保存图片时，发生异常:{}", write_result.unwrap_err());
            return Err(Error::from("图片失败!"));
        }
        LogMapper::record_log_by_context(primary_rbatis_pool!(), &user_info, String::from("OX005"))
            .await;
        return Ok(picture.web_url.unwrap());
    }

    /// 执行文件上传
    pub async fn upload_file(&self, req: &HttpRequest, mut payload: Multipart) -> Result<String> {
        // 默认的一些回填值
        let mut uid = String::from("null");
        let mut origin_name_copy: String = String::new();
        let mut local_path: Option<String> = None;
        let mut file_belong_type: Option<&String> = None;
        let user_info = UserContext::extract_user_by_request(req)
            .await
            .ok_or_else(|| Error::from(util::NOT_AUTHORIZE_CODE))?;
        // 首先判断要修改的用户是否存在
        let query_user_wrap =
            User::select_by_account(primary_rbatis_pool!(), &user_info.account).await;
        if query_user_wrap.is_err() {
            error!("查询用户异常：{}", query_user_wrap.unwrap_err());
            return Err(Error::from("查询用户失败!"));
        }
        let user_option = query_user_wrap.unwrap().into_iter().next();
        user_option.ok_or_else(|| {
            Error::from((
                format!("用户:{} 不存在!", &user_info.account.clone()),
                util::NOT_EXIST_CODE,
            ))
        })?;
        let today_op =
            DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()), util::FORMAT_YMD);
        let today = today_op.unwrap();
        let save_path = format!(
            "{}/{}/{}/{}",
            &CONTEXT.config.data_dir,
            util::DOCUMENT_PATH,
            &user_info.account.clone(),
            today.clone()
        );

        while let Some(item) = payload.next().await {
            let mut field = item.unwrap();
            let content_disposition = field.content_disposition();
            // 提取字段名
            let field_name_op = content_disposition.get_name();
            if field_name_op.is_empty() {
                continue;
            }
            let field_name = field_name_op.unwrap().to_string();
            println!("field name:{}", field_name);
            // 对待文件域字段单独处理上传
            match field_name == String::from("file") {
                true => {
                    // 提取文件名
                    let origin_name = content_disposition.get_filename();
                    origin_name_copy = format!("{}", origin_name.clone().unwrap());
                    let lowercase_file_name = origin_name.clone().unwrap().to_lowercase();
                    let file_ext: Vec<&str> = lowercase_file_name.split(".").collect();
                    if file_ext.len() < 2 {
                        return Err(Error::from(("无效的文件!", util::NOT_PARAMETER_CODE)));
                    }
                    let file_belong_type_op = CONTEXT
                        .config
                        .file_type_map
                        .get(file_ext[file_ext.len() - 1]);
                    if file_belong_type_op.is_some() {
                        file_belong_type = file_belong_type_op.clone();
                    }
                    let file_name = format!(
                        "{}{}.{}",
                        today.clone(),
                        &Snowflake::default().generate(),
                        &file_ext[file_ext.len() - 1]
                    );
                    // 调用文件的保存接口
                    let save_result = self
                        .save_file(Path::new(&save_path), &file_name, field)
                        .await;
                    if save_result.is_err() {
                        error!(
                            "在保存{}用户的文件时，发生异常:{}",
                            &user_info.account,
                            save_result.unwrap_err().to_string()
                        );
                        return Err(Error::from(String::from("文件上传失败")));
                    }
                    local_path = save_result.ok().clone();
                }
                _ => {
                    if field_name == String::from("uid") {
                        uid = String::new();
                        while let Some(chunk) = field.next().await {
                            uid = uid.add(std::str::from_utf8(&chunk.unwrap()).unwrap());
                        }
                    }
                }
            };
        }
        if local_path.is_none() {
            return Err(Error::from(("文件上传失败!", util::NOT_PARAMETER_CODE)));
        }
        let http_path = local_path
            .clone()
            .unwrap()
            .replace(&CONTEXT.config.data_dir, &String::from(""));
        let files = Files {
            id: None,
            uid: Some(uid),
            file_name: Some(origin_name_copy),
            file_url: Some(http_path),
            file_type: if file_belong_type.is_some() {
                Some(file_belong_type.unwrap().to_string())
            } else {
                Some(String::from("complex"))
            },
            organize: Some(user_info.organize),
            source: Some(user_info.account.clone()),
            status: Some(2),
            create_time: DateTimeUtil::naive_date_time_to_str(
                &Some(DateUtils::now()),
                &util::FORMAT_Y_M_D_H_M_S,
            ),
            update_time: None,
        };
        let write_result = Files::insert(business_rbatis_pool!(), &files).await;
        if write_result.is_err() {
            error!("保存文件时，发生异常:{}", write_result.unwrap_err());
            return Err(Error::from("保存文件失败!"));
        }
        LogMapper::record_log_by_context(primary_rbatis_pool!(), &user_info, String::from("OX013"))
            .await;
        return Ok(files.file_url.unwrap());
    }

    /// 保存base64的图片
    pub async fn save_base64(
        &self,
        save_path: &Path,
        content: &str,
        file_name: &String,
    ) -> Result<String> {
        let path_check_result = self.mkdir_if_not_exists(save_path).await;
        if path_check_result.is_err() {
            return Err(path_check_result.unwrap_err());
        }
        let file_path = format!("{}/{}", save_path.to_str().unwrap(), file_name);
        let result = Ok(file_path.clone());
        let mut f = web::block(|| std::fs::File::create(file_path))
            .await
            .unwrap()
            .unwrap();
        let bytes = base64::decode(content).unwrap();
        // f = web::block(move || f.write_all(bytes.as_slice()).map(|_| f)).await.unwrap().unwrap();
        web::block(move || f.write_all(bytes.as_slice()).map(|_| f))
            .await
            .unwrap()
            .unwrap();
        return result;
    }

    /// 保存文件
    pub async fn save_file(
        &self,
        save_path: &Path,
        file_name: &String,
        mut field: Field,
    ) -> Result<String> {
        let path_check_result = self.mkdir_if_not_exists(save_path).await;
        if path_check_result.is_err() {
            return Err(path_check_result.unwrap_err());
        }
        let file_path = format!("{}/{}", save_path.to_str().unwrap(), file_name);
        let result = Ok(file_path.clone());
        let mut f = web::block(|| std::fs::File::create(file_path))
            .await
            .unwrap()
            .unwrap();
        while let Some(chunk) = field.try_next().await.unwrap() {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f))
                .await
                .unwrap()
                .unwrap();
        }
        return result;
    }

    /// 创建目录（不存在的情况下）
    pub async fn mkdir_if_not_exists(&self, save_path: &Path) -> Result<bool> {
        if !save_path.exists() {
            let create_result = std::fs::create_dir_all(save_path);
            if create_result.is_err() {
                error!(
                    "create folder fail,cause by:{:?}",
                    create_result.unwrap_err()
                );
                return Err(Error::from((
                    "create folder fail",
                    util::FILE_IO_ERROR_CODE,
                )));
            }
        }
        Ok(true)
    }
}
