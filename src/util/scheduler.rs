use std::borrow::Borrow;
use std::collections::HashMap;
use std::thread;
use log::{error, info};
use crate::service::{CONTEXT, SCHEDULER};
use chrono::{Duration, Local, NaiveDateTime};
use cron_tab::Cron;
use rbatis::crud::{CRUD, CRUDMut};
use crate::dao::plan_mapper::PlanMapper;
use crate::entity::domain::primary_database_tables::{DbDumpLog, Plan, PlanArchive, User};
use crate::entity::dto::plan::PlanPageDTO;
use crate::util;
use crate::util::date_time::{DateTimeUtil, DateUtils};
use crate::util::scheduler;
use std::process::Command;
use std::fs::File;
use std::ops::Sub;
use rbatis::value::DateTimeNow;
use crate::dao::plan_archive_mapper::PlanArchiveMapper;
use crate::entity::vo::plan_archive::PlanArchiveVO;
use crate::util::mail_util::MailUtils;

/// 调度任务 https://crates.io/crates/cron_tab
pub struct Scheduler {
    pub scheduler: Cron<Local>,
    pub plan_pool: HashMap<u64, i32>,
}

/// 落实mysql 的 mysqldump操作
pub async fn do_execute_mysqldump(){
    let start_time = DateTimeUtil::naive_date_time_to_str(&Some(chrono::NaiveDateTime::now()), util::FORMAT_Y_M_D_H_M_S);
    // 确定要备份的路径
    let archive_date = NaiveDateTime::now().sub(Duration::days(1));
    let today_op = DateTimeUtil::naive_date_time_to_str(&Some(archive_date.date()), util::FORMAT_YMD);
    let today = today_op.unwrap();
    // 数据备份的相对路径
    let db_path = format!("/{}/db/db_dump_{}.sql",util::DOCUMENT_PATH,today);
    // 数据备份的完整路径
    let save_path = format!("{}{}", &CONTEXT.config.data_dir,db_path.clone());
    let mut command = Command::new("sh");
    command.arg("-c").arg(&CONTEXT.config.mysqldump);
    command.stdout(File::create(save_path).unwrap());
    let code = command.status().unwrap().code();
    let end_time = DateTimeUtil::naive_date_time_to_str(&Some(chrono::NaiveDateTime::now()), util::FORMAT_Y_M_D_H_M_S);
    match code {
        Some(code) => {
            // 写入备份成功的数据记录
            let log = DbDumpLog{
                id:None,
                url: Some(db_path),
                archive_date: Some(chrono::NaiveDateTime::now().sub(Duration::days(1)).date()),
                execute_data: Some(chrono::NaiveDateTime::now())
            };
            let write_result = CONTEXT.primary_rbatis.save(&log, &[]).await;
            if  write_result.is_err(){
                error!("备份数据库时，发生异常:{}",write_result.unwrap_err());
            }else {
                MailUtils::send_dump_massage(DateTimeUtil::naive_date_time_to_str(&Some(archive_date.date()), util::FORMAT_Y_M_D).unwrap().as_str(),start_time.unwrap().as_str(),end_time.unwrap().as_str())
            }
            info!("Exit Status: {}", code);
        }
        None => {
            // 发送备份失败的消息
            error!("Process terminated.");
        }
    }
}

/// 落实调度任务的具体执行
pub async fn do_plan_notice(date:chrono::NaiveDateTime) {
    println!("begin execute task at: {}", date.to_string());
    // 查询所有的用户信息，并放入map中
    let user_query_result = CONTEXT.primary_rbatis.fetch_list().await;
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

    let query_where = PlanPageDTO{id: None,standard_time:Some(date),cycle: None, unit: None,content: None,next_exec_time: None, user: None,display: None, page_no: None,page_size: None,begin_time: None,end_time: None,organize: None };
    let list_result = PlanMapper::select_list(&mut CONTEXT.primary_rbatis.as_executor(), &query_where).await;
    if list_result.is_err() {
        // 查询此刻的计划提醒发生异常
        error!("触发定时任务后，查询计划提醒发生异常:{}",list_result.unwrap_err());
        return;
    }
    let plans = list_result.unwrap().unwrap();
    let mut scheduler = SCHEDULER.lock().unwrap();
    for mut plan in plans {
        // 提前准备任务归档数据
        let plan_archive = PlanArchive{
            id: None,
            plan_id: plan.id,
            status: Some(1),
            content: plan.clone().content,
            archive_time: Some(date),
            create_time: Some(chrono::NaiveDateTime::now()),
            update_time: None
        };
        if 1 == plan.cycle.unwrap() {
            // 一次性的任务，计划提醒表(plan)按兵不动，只用归档
            let write_result = CONTEXT.primary_rbatis.save(&plan_archive, &[]).await;
            if  write_result.is_err(){
                error!("在归档计划提醒事项时id={}，archive_time={}，发生异常:{}",plan.id.unwrap(),date,write_result.unwrap_err());
            }
            scheduler.remove(plan.id.unwrap());
            continue;
        }


        // 将上次计算好的本次时间放入到本次的基准时间
        plan.standard_time = plan.next_exec_time;
        // 计算下次执行时间
        let next_exec_time = DateUtils::plan_data_compute(&plan.standard_time.clone().unwrap(),plan.cycle.unwrap(),plan.unit.unwrap());
        plan.next_exec_time = Some(next_exec_time);
        let mut tx = CONTEXT.primary_rbatis.acquire_begin().await.unwrap();
        let edit_plan_result = PlanMapper::update_plan(&mut tx.as_executor(), &plan).await;
        if edit_plan_result.is_err() {
            error!("在修改id={}的计划提醒时，发生异常:{}",plan.id.as_ref().unwrap(),edit_plan_result.unwrap_err());
            tx.rollback();
            continue;
        }

        // 预写入任务归档数据
        let add_plan_archive_result = tx.save(&plan_archive, &[]).await;
        if add_plan_archive_result.is_err() {
            error!("在归档计划提醒时，发生异常:{}",add_plan_archive_result.unwrap_err());
            tx.rollback();
            continue;
        }
        // 所有的写入都成功，最后正式提交
        tx.commit().await;
        // 生成定时cron表达式
        let cron_tab = DateUtils::data_time_to_cron(&plan.standard_time.clone().unwrap());
        scheduler.remove(plan.id.unwrap());
        scheduler.add(plan.id.unwrap(),cron_tab.as_str());
        // TODO 发一次邮件给予提示
        if !plan_pool.contains_key(plan.user.clone().unwrap().as_str()){
            return;
        }
        let user_op = plan_pool.get(plan.user.clone().unwrap().as_str());
        let user = user_op.unwrap();
        let mut contets:Vec<String> =  Vec::new();
        let content = plan.content.clone().unwrap();
        contets.push(content);
        MailUtils::send_plan_massage(true,user,contets)
    }
}

/// 落实未完成的任务提醒
pub async fn do_undone_plan_notice(){
    // 查询截止到今日都还没有完成的任务
    let query_list_result = PlanArchiveMapper::select_undone_list(&mut CONTEXT.primary_rbatis.as_executor()).await;
    if query_list_result.is_err() {
        // 查询此刻的计划提醒发生异常
        error!("触发定时任务后，查询未完成的计划提醒发生异常:{}",query_list_result.unwrap_err());
        return;
    }
    let plans:Vec<PlanArchiveVO> = query_list_result.unwrap().unwrap();
    // 对提醒按照用户进行分组
    let mut map:HashMap<String,Vec<PlanArchiveVO>> = HashMap::new();
    for item in plans {
        let user_account = item.user_account.clone().unwrap();
        if map.contains_key(&user_account) {
            let mut list = map.get(&user_account).unwrap().to_vec();
            list.push(item);
            map.insert(user_account, list);
        }else {
            map.insert(user_account, vec![item]);
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

/// 触发执行调度执行
pub fn execute_mysqldump() {
    futures::executor::block_on(do_execute_mysqldump());
}

/// 触发执行调度执行
pub fn plan_notice() {
    futures::executor::block_on(do_plan_notice(chrono::NaiveDateTime::now()));
}

/// 触发执行调度执行
pub fn undone_plan_notice() {
    futures::executor::block_on(do_undone_plan_notice());
}

impl Scheduler {

    /// 调度任务的初始化
    pub async fn start() {
        // 查询现存所有活跃的提醒计划
        let plan_result= CONTEXT.primary_rbatis.fetch_list().await;
        let mut poll = SCHEDULER.lock().unwrap();
        // 添加一个未完成计划的定时任务
        poll.scheduler.add_fn("0 0 2 * * ?",scheduler::undone_plan_notice);
        // 添加一个备份数据库的定时任务
        poll.scheduler.add_fn("0 0 3 * * ?",scheduler::execute_mysqldump);
        if plan_result.is_ok(){
            let plans:Vec<Plan> = plan_result.unwrap();
            for plan in plans {
                let cron_tab = DateUtils::data_time_to_cron(&plan.standard_time.clone().unwrap());
                poll.add(plan.id.unwrap(),cron_tab.as_str());
            }
        }
        poll.scheduler.start();
        // sleep 2 second
        info!(" - cron pool init finish!");
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    pub fn add(&mut self, plan_id: u64,cron: &str) {
        // let job_id = self.scheduler.add_fn("* * * * * * *",  scheduler::plan_notice).unwrap();
        let job_id = self.scheduler.add_fn(cron,  scheduler::plan_notice).unwrap();
        self.plan_pool.insert(plan_id, job_id);
        info!(" - cron[plan_id={}] insert finish! cron uuid = {}",plan_id,job_id);
    }

    pub fn remove(&mut self,plan_id:u64){
        if self.plan_pool.contains_key(&plan_id) {
            let uuid = self.plan_pool.get(&plan_id);
            self.scheduler.remove(*uuid.unwrap());
            info!(" - cron[{}]  already remove",plan_id);
        }
    }

}