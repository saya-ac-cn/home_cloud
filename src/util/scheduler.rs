use std::collections::HashMap;
use actix_web::web::service;
use lazy_static::lazy::Lazy;
use log::info;
use crate::service::{CONTEXT, SCHEDULER};
use std::sync::Mutex;
use chrono::{FixedOffset, Local, TimeZone, Utc};
use cron_tab::Cron;
use crate::util::scheduler;

/// 调度任务 https://crates.io/crates/cron_tab
pub struct Scheduler {
    pub scheduler: Cron<Utc>,
    pub plan_pool: HashMap<u64, i32>,
}

pub fn plan_notice() {
    println!("now: {}", Local::now().to_string());
}

impl Scheduler {

    pub async fn start() {
        SCHEDULER.lock().unwrap().scheduler.start();
        // sleep 2 second
        info!(" - cron pool init finish!");
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    pub fn add(&mut self, plan_id: u64) {
        let job_id = self.scheduler.add_fn("* * * * * * *",  scheduler::plan_notice).unwrap();
        self.plan_pool.insert(plan_id, job_id);
        info!(" - cron[{}] insert finish! uuid = {}",plan_id,job_id);
    }

    pub fn remove(&mut self,plan_id:u64){
        if self.plan_pool.contains_key(&plan_id) {
            let uuid = self.plan_pool.get(&plan_id);
            self.scheduler.remove(*uuid.unwrap());
            info!(" - cron[{}]  already remove",plan_id);
        }
    }

}