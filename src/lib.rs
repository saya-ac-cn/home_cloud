#![allow(unused_variables)] //允许未使用的变量
#![allow(dead_code)] //允许未使用的代码
#![allow(unused_must_use)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;


#[macro_use]
///工具类
pub mod util;
///配置模块
pub mod config;
///接口模块
pub mod controller;
///数据库模块
pub mod dao;
///领域模型模块
pub mod entity;
///错误结构体
pub mod error;
///actix-web中间件
pub mod middleware;
///服务模块
pub mod service;