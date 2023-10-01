use std::collections::HashMap;
use log::{error, info};
use crate::service::{CONTEXT, SCHEDULER};
use chrono::{Duration};
use crate::domain::mapper::plan_mapper::PlanMapper;
use crate::domain::table::{DbDumpLog, Plan, PlanArchive, User};
use crate::domain::dto::plan::PlanPageDTO;
use crate::{primary_rbatis_pool, util};
use crate::util::date_time::{DateTimeUtil, DateUtils};
use std::process::Command;
use std::fs::File;
use std::ops::Sub;
use crate::domain::mapper::plan_archive_mapper::PlanArchiveMapper;
use crate::domain::vo::plan_archive::PlanArchiveVO;
use crate::util::mail_util::MailUtils;
use delay_timer::prelude::{Task, TaskBuilder, TaskError};

/// 调度任务 https://github.com/BinChengZhao/delay-timer
pub struct Scheduler {}

/// 落实mysql 的 mysqldump操作
pub async fn execute_mysqldump_body(){
    let start_time = DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()),&util::FORMAT_Y_M_D_H_M_S);
    // 确定要备份的路径
    let archive_date = DateUtils::now().sub(Duration::days(1));
    let today_op = DateTimeUtil::naive_date_time_to_str(&Some(archive_date),&util::FORMAT_YMD);
    let today = today_op.unwrap();
    // 数据备份的相对路径
    let db_path = format!("/{}/db_dump_{}.sql",util::DATABASE_PATH,today);
    // 数据备份的完整路径
    let save_path = format!("{}{}", &CONTEXT.config.data_dir,db_path.clone());
    let mut command = Command::new("sh");
    command.arg("-c").arg(&CONTEXT.config.mysql_dump);
    command.stdout(File::create(save_path).unwrap());
    let code = command.status().unwrap().code();
    let end_time = DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()),&util::FORMAT_Y_M_D_H_M_S);
    match code {
        Some(code) => {
            // 写入备份成功的数据记录
            let log = DbDumpLog{
                id:None,
                url: Some(db_path),
                archive_date: Some(today.clone()),
                execute_data: Some(start_time.clone().unwrap())
            };
            let write_result = DbDumpLog::insert(primary_rbatis_pool!(),&log).await;
            if  write_result.is_err(){
                error!("备份数据库时，发生异常:{}",write_result.unwrap_err());
            }else {
                MailUtils::send_dump_massage(today.clone().as_str(),start_time.unwrap().as_str(),end_time.unwrap().as_str())
            }
            info!("Exit Status: {}", code);
        }
        None => {
            // 发送备份失败的消息
            error!("Process terminated.");
        }
    }
}

/// 构造MySQL备份计划任务
fn build_mysqldump_async_task() -> Result<Task, TaskError> {
    let mut task_builder = TaskBuilder::default();
    // let name = String::from("someting");
    // let body = move || {
    //     let name_ref = name.clone();
    //     async move {
    //         execute_mysqldump_body().await
    //     }
    // };
    task_builder
        .set_frequency_repeated_by_cron_str("0 0 3 * * ?")
        .set_task_id(0)
        .set_maximum_running_time(300)
        .spawn_async_routine(execute_mysqldump_body)
}

/// 落实未完成的任务提醒
pub async fn undone_plan_notice_body(){
    // 查询截止到今日都还没有完成的任务
    let query_list_result = PlanArchiveMapper::select_undone_list(primary_rbatis_pool!()).await;
    if query_list_result.is_err() {
        // 查询此刻的计划提醒发生异常
        error!("触发定时任务后，查询未完成的计划提醒发生异常:{}",query_list_result.unwrap_err());
        return;
    }
    let plans:Vec<PlanArchiveVO> = query_list_result.unwrap().unwrap();
    if plans.len() > 0 {
        info!("今日不存在未完成的事项");
        return;
    }
    // 对提醒按照用户进行分组
    let mut map:HashMap<String,Vec<PlanArchiveVO>> = HashMap::new();
    for item in plans {
        let account = item.user.clone().unwrap();
        if map.contains_key(&account) {
            let mut list = map.get(&account).unwrap().to_vec();
            list.push(item);
            map.insert(account, list);
        }else {
            map.insert(account, vec![item]);
        }
    }
    // 遍历这个map
    for (account,plans) in map.iter() {
        let user_info = plans.get(0).clone().unwrap();
        let mut contets:Vec<String> =  Vec::new();
        let user_mail_info = User{account: Some(account.clone()),name: user_info.clone().user_name,password: None,sex: None,qq: None, email: user_info.clone().user_mail,phone: None,birthday: None,hometown: None, autograph: None,logo: None,background: None,organize_id: None,state: None,create_time: None,update_time: None };
        // 对这个用户下的提醒整理成一个列表
        for plan in plans {
            let content = plan.content.clone().unwrap();
            contets.push(content);
        }
        // 发送邮件
        MailUtils::send_plan_massage(false,&user_mail_info,contets)
    }
}

/// 构造未完成的待办提醒任务
fn build_undone_plan_async_task() -> Result<Task, TaskError> {
    let mut task_builder = TaskBuilder::default();
    // let body = || async {
    //     undone_plan_notice_body().await;
    // };

    task_builder
        .set_frequency_repeated_by_cron_str("0 0 8 * * ?")
        .set_task_id(5)
        .set_maximum_running_time(300)
        .spawn_async_routine(undone_plan_notice_body)
}

/// 落实待办项任务的具体执行
pub async fn do_plan_notice() {
    let date:String = DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()),&util::FORMAT_Y_M_D_H_M_S).unwrap();
    info!("begin execute task at: {}", date.clone());
    // 查询所有的用户信息，并放入map中
    let user_query_result = User::select_all(primary_rbatis_pool!()).await;
    if user_query_result.is_err(){
        error!("查询用户发送异常:{}",user_query_result.unwrap_err());
        return;
    }
    let user_list:Vec<User>  = user_query_result.unwrap();
    if user_list.is_empty() {
        return;
    }
    let mut plan_pool: HashMap<String, User>= HashMap::new();
    for user in user_list {
        let account = user.account.clone().unwrap();
        plan_pool.insert(account,user);
    }

    let query_where = PlanPageDTO{id: None,standard_time:Some(date.clone()),cycle: None, unit: None,title:None,content: None,next_exec_time: None, user: None,display: None, page_no: None,page_size: None,begin_time: None,end_time: None,organize: None };
    let list_result = PlanMapper::select_list(primary_rbatis_pool!(), &query_where).await;
    if list_result.is_err() {
        // 查询此刻的计划提醒发生异常
        error!("触发定时任务后，查询计划提醒发生异常:{}",list_result.unwrap_err());
        return;
    }
    let plans = list_result.unwrap().unwrap();
    for mut plan in plans {
        // 提前准备任务归档数据
        let plan_archive = PlanArchive{
            id: None,
            status: Some(1),
            title: plan.title.clone(),
            content: plan.content.clone(),
            archive_time: Some(date.clone()),
            organize:plan.organize,
            user:plan.user.clone(),
            display:plan.display,
            create_time: DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()),&util::FORMAT_Y_M_D_H_M_S),
            update_time: None
        };
        if 1 == plan.cycle.unwrap() {
            // 一次性的任务，计划提醒表(plan)按兵不动，只用归档
            let write_result = PlanArchive::insert(primary_rbatis_pool!(),&plan_archive).await;
            if  write_result.is_err(){
                error!("在归档计划提醒事项时id={}，archive_time={}，发生异常:{}",plan.id.unwrap(),date,write_result.unwrap_err());
            }
            Scheduler::remove(plan.id.unwrap()).await;
            continue;
        }

        // 将上次计算好的本次时间放入到本次的基准时间
        plan.standard_time = plan.next_exec_time;
        let standard_time_result = chrono::NaiveDateTime::parse_from_str(plan.standard_time.clone().unwrap().as_str(),&util::FORMAT_Y_M_D_H_M_S);
        if standard_time_result.is_err() {
            error!("格式化日期发生异常:{}",standard_time_result.unwrap_err());
            return;
        }
        let standard_time = standard_time_result.unwrap();

        // 计算下次执行时间
        let next_exec_time_op = DateUtils::plan_data_compute(&standard_time.clone(),plan.cycle.unwrap(),plan.unit.unwrap());
        if next_exec_time_op.is_none() {
            error!("无效的日期(standard_time={},cycle={},unit={})循环周期",&standard_time.clone(),plan.cycle.unwrap(),plan.unit.unwrap());
            continue;
        }
        let next_exec_time = next_exec_time_op.unwrap();
        plan.next_exec_time = DateTimeUtil::naive_date_time_to_str(&Some(next_exec_time),&util::FORMAT_Y_M_D_H_M_S);
        let mut tx = primary_rbatis_pool!().acquire_begin().await.unwrap();
        let edit_plan_result = PlanMapper::update_plan(&mut tx, &plan).await;
        if edit_plan_result.is_err() {
            error!("在修改id={}的计划提醒时，发生异常:{}",plan.id.as_ref().unwrap(),edit_plan_result.unwrap_err());
            let check = tx.rollback().await;
            if check.is_ok() {
                continue;
            }else {
                return;
            }
        }

        // 预写入任务归档数据
        let add_plan_archive_result = PlanArchive::insert(&mut tx,&plan_archive).await;
        if add_plan_archive_result.is_err() {
            error!("在归档计划提醒时，发生异常:{}",add_plan_archive_result.unwrap_err());
            let check = tx.rollback().await;
            if check.is_ok() {
                continue;
            }else {
                return;
            }
        }
        // 所有的写入都成功，最后正式提交
        let check = tx.commit().await;
        if check.is_err() {
            error!("在归档计划提醒时，发生异常:{}",check.unwrap_err());
            return;
        }
        // 生成定时cron表达式
        let cron_tab = DateUtils::data_time_to_cron(&standard_time.clone());
        Scheduler::edit_plan(plan.id.unwrap(),cron_tab.as_str()).await;
        let user_op = plan_pool.get(plan.user.clone().unwrap().as_str());
        let user = user_op.unwrap();
        let mut contets:Vec<String> =  Vec::new();
        let content = plan.content.clone().unwrap();
        contets.push(content);
        MailUtils::send_plan_massage(true,user,contets)
    }
}

/// 构造待办项任务
fn build_plan_notice_async_task(task_id:u64,cron:&str) -> Result<Task, TaskError> {
    let mut task_builder = TaskBuilder::default();
    task_builder
        .set_frequency_once_by_cron_str(cron)
        .set_task_id(task_id)
        .set_maximum_running_time(300)
        .spawn_async_routine(do_plan_notice)
}

impl Scheduler {
    /// 初始化系统级别的调度任务（发生在系统每次启动时）
    pub async fn init_system_scheduler() {
        let scheduler = SCHEDULER.lock().await;
        // 添加一个备份数据库的定时任务
        scheduler.add_task(build_mysqldump_async_task().unwrap());
        // 添加一个未完成计划的定时任务
        scheduler.add_task(build_undone_plan_async_task().unwrap());
        // 查询现存所有活跃的提醒计划
        let plan_result= Plan::select_current_after(primary_rbatis_pool!()).await;
        if plan_result.is_ok(){
            let plans:Vec<Plan> = plan_result.unwrap();
            for plan in plans {
                let standard_time_result = chrono::NaiveDateTime::parse_from_str(plan.standard_time.clone().unwrap().as_str(),&util::FORMAT_Y_M_D_H_M_S);
                if standard_time_result.is_err() {
                    error!("格式化日期发生异常:{}",standard_time_result.unwrap_err());
                    return;
                }
                let standard_time = standard_time_result.unwrap();
                let cron_tab = DateUtils::data_time_to_cron(&standard_time);
                info!(" - task add success cron={}",&cron_tab);
                scheduler.add_task(build_plan_notice_async_task(plan.id.unwrap(),cron_tab.as_str()).unwrap());
            }
        }
        info!(" - cron pool init finish!");
    }

    pub async fn add_plan(task_id: u64, cron: &str) {
        let scheduler = SCHEDULER.lock().await;
        let result:Result<(), TaskError> = scheduler.add_task(build_plan_notice_async_task(task_id,cron).unwrap());
        if result.is_ok(){
            info!(" - task_id:{} add success",task_id);
        }else {
            error!(" - task_id:{} add failed:{:?}",task_id,result.unwrap_err());
        }
    }

    pub async fn edit_plan(task_id:u64,cron: &str){
        let scheduler = SCHEDULER.lock().await;
        let result:Result<(), TaskError> = scheduler.remove_task(task_id);
        if result.is_ok(){
            info!(" - task_id:{}  already remove",task_id);
            let _result:Result<(), TaskError> = scheduler.add_task(build_plan_notice_async_task(task_id,cron).unwrap());
            if _result.is_ok(){
                info!(" - task_id:{} edit success",task_id);
            }else {
                error!(" - task_id:{} edit failed:{:?}",task_id,_result.unwrap_err());
            }
        }else {
            error!(" - task_id:{}  remove failed:{:?}",task_id,result.unwrap_err());
        }
    }

    pub async fn remove(task_id:u64){
        let scheduler = SCHEDULER.lock().await;
        let result:Result<(), TaskError> = scheduler.remove_task(task_id);
        if result.is_ok(){
            info!(" - task_id:{}  already remove",task_id);
        }else {
            error!(" - task_id:{} remove failed:{:?}",task_id,result.unwrap_err());
        }
    }

}