use rbatis::crud::{CRUD, CRUDMut};
use rbatis::DateTimeNative;
use actix_multipart::{Field, Multipart};
use futures_util::{ StreamExt, TryStreamExt};
use crate::error::Result;
use crate::service::CONTEXT;
use actix_web::{middleware, web,App, Error,HttpResponse, HttpServer};
use std::io::Write;
use std::ops::Add;
use std::path::Path;
use crate::util::string::IsEmptyString;


pub struct FileService{}

impl FileService {

    // pub async fn upload_file(&self, mut payload: Multipart) -> Result<HttpResponse> {
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

    pub async fn save_file(&self, file_name:&String, mut field: Field) -> Result<HttpResponse>{
        let warehouse_path = Path::new("./tmp/");
        if !warehouse_path.exists(){
            let create_result = std::fs::create_dir_all(warehouse_path);
            if create_result.is_err() {
                println!("create folder fail,cause by:{:?}",create_result.unwrap_err());
                //return Err(Error::from("create folder fail"));
            }
        }
        let file_path = format!("{}{}", warehouse_path.to_str().unwrap(), file_name);
        let result = Ok(file_path.clone());
        println!("save path{}", file_path);
        let mut f = web::block(|| std::fs::File::create(file_path)).await?;
        while let Some(chunk) = field.try_next().await?{
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.unwrap().write_all(&chunk).map(|_| f)).await??;
        }
        Ok(HttpResponse::Ok().into())
    }

}