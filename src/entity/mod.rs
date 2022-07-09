///DDD分层架构，分为
///
/// * 领域层（domain）,该层存放数据库结构体模型（只在服务端流转，表结构）
pub mod domain;
/// * 数据传输层（dto，Data Transfer Object ）,存放接口传输的结构体（请求到服务端）
pub mod dto;
/// * 展示层（vo，View Object），存放展示的结构体（返回给无线端）
pub mod vo;
