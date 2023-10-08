use std::fs::File;
use std::io::{Read, Write};
use crate::util::error::{Error, Result};
use std::path::Path;
use actix_multipart::Field;
use actix_web::web;
use futures_util::TryStreamExt;
use log::error;
use tokio::fs;
use crate::util::FILE_IO_ERROR_CODE;

/// 创建目录（不存在的情况下）
pub async fn mkdir_if_not_exists(save_path: &Path) -> Result<bool> {
    if !save_path.exists() {
        let create_result = std::fs::create_dir_all(save_path);
        if create_result.is_err() {
            error!("create folder fail,cause by:{:?}",create_result.unwrap_err());
            return Err(Error::from(("create folder fail",FILE_IO_ERROR_CODE)));
        }
    }
    Ok(true)
}

/// 删除文件
pub async fn remove_file_if_exists(path: &Path) -> Result<bool> {
    if path.exists() {
        let remove_result = fs::remove_file(path).await;
        if remove_result.is_err() {
            error!("fail remove fail,cause by:{:?}",remove_result.unwrap_err());
            return Err(Error::from(("fail remove fail",FILE_IO_ERROR_CODE)));
        }
    }
    Ok(true)
}

/// 写入文本到文件中，最后返回的是文件的位置，没有成功则是空
pub async fn save_content(save_path: &Path, file_name: &String,content: String) -> Result<String>{
    let path_check_result = mkdir_if_not_exists(save_path).await;
    if path_check_result.is_err() {
        return Err(path_check_result.unwrap_err());
    }
    let file_path = format!("{}/{}", save_path.to_str().unwrap(), file_name);
    let file_result = File::create(file_path.clone());
    if file_result.is_err() {
        error!("create content parent path fail,cause by:{:?}",file_result.unwrap_err());
        return Err(Error::from(("create content parent path fail",FILE_IO_ERROR_CODE)));
    }
    let mut file = file_result.unwrap();
    let write_result = file.write_all(content.as_bytes());
    if write_result.is_err() {
        error!("file write fail,cause by:{:?}",write_result.unwrap_err());
        return Err(Error::from(("file write fail",FILE_IO_ERROR_CODE)));
    }
    Ok(file_path)
}

pub async fn edit_content(save_path: &Path,content: String) -> Result<bool>{
    if !Path::new(&save_path).exists() {
        // 判断父目录是否存在
        let parent_result = mkdir_if_not_exists(Path::new((&save_path).parent().unwrap())).await;
        if parent_result.is_err() {
            error!("create content parent path fail,cause by:{:?}",parent_result.unwrap_err());
            return Err(Error::from(("create content parent path fail",FILE_IO_ERROR_CODE)));
        }

    }
    let file_result = File::create(save_path);
    if file_result.is_err() {
        error!("file create fail,cause by:{:?}",file_result.unwrap_err());
        return Err(Error::from(("file create fail",FILE_IO_ERROR_CODE)));
    }
    let mut file = file_result.unwrap();
    let write_result = file.write_all(content.as_bytes());
    if write_result.is_err() {
        error!("file write fail,cause by:{:?}",write_result.unwrap_err());
        return Err(Error::from(("file write fail",FILE_IO_ERROR_CODE)));
    }
    Ok(true)
}


/// 读取文件内容，最后返回的是文件的内容，没有成功则是空
pub async fn read_content(path: &Path) -> Result<String>{
    if !Path::new(&path).exists() {
        error!("file not exists,cause by:{:?}",path);
        return Err(Error::from(("file not exists",FILE_IO_ERROR_CODE)));
    }
    let file_result = File::open(path);
    if file_result.is_err() {
        error!("file open fail,cause by:{:?}",file_result.unwrap_err());
        return Err(Error::from(("file open fail",FILE_IO_ERROR_CODE)));
    }
    let mut file = file_result.unwrap();
    let mut result = String::new();
    let read_result = file.read_to_string(&mut result);
    if read_result.is_err() {
        error!("file read fail,cause by:{:?}",read_result.unwrap_err());
        return Err(Error::from(("file read fail",FILE_IO_ERROR_CODE)));
    }
    Ok(result)
}

/// 保存文件，最后返回的是文件的位置
pub async fn save_file(save_path: &Path, file_name: &String, mut field: Field) -> Result<String> {
    let path_check_result = mkdir_if_not_exists(save_path).await;
    if path_check_result.is_err() {
        return Err(path_check_result.unwrap_err());
    }
    let file_path = format!("{}/{}", save_path.to_str().unwrap(), file_name);
    let result = file_path.clone();
    let mut f = web::block( || File::create(file_path)).await.unwrap().unwrap();
    while let Some(chunk) = field.try_next().await.unwrap() {
        // filesystem operations are blocking, we have to use threadpool
        f = web::block(move || f.write_all(&chunk).map(|_| f)).await.unwrap().unwrap();
    }
    Ok(result)
}

/// 保存base64的图片，最后返回的是文件的位置
pub async fn save_base64(save_path: &Path, content: &str, file_name: &String) -> Result<String> {
    let path_check_result = mkdir_if_not_exists(save_path).await;
    if path_check_result.is_err() {
        return Err(path_check_result.unwrap_err());
    }
    let file_path = format!("{}/{}", save_path.to_str().unwrap(), file_name);
    let result = file_path.clone();
    let mut f = web::block(|| File::create(file_path)).await.unwrap().unwrap();
    let bytes = base64::decode(content).unwrap();
    // f = web::block(move || f.write_all(bytes.as_slice()).map(|_| f)).await.unwrap().unwrap();
    web::block(move || f.write_all(bytes.as_slice()).map(|_| f)).await.unwrap().unwrap();
    Ok(result)
}