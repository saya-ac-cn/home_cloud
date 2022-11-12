use std::ops::Add;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::string::String;
use lettre::message::{header, MultiPart, SinglePart};
use crate::entity::domain::primary_database_tables::User;
use crate::service::CONTEXT;
use crate::util;
use crate::util::date_time::{DateTimeUtil, DateUtils};

pub struct MailUtils{}

impl MailUtils {

    /// 发送数据库备份通知邮件
    pub fn send_dump_massage(archive_date:&str,start_date:&str,end_date:&str) {

        let html_template = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>极客印记-邮件系统</title>
    <meta http-equiv="Content-Type" content="text/html; charset=UTF-8"/>
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="icon" href="https://saya.ac.cn/favicon.svg"/>
</head>
<body>
    <div style="width:available;display: flex;justify-content: center;align-items: center;">
        <div style="width: 700px;">
            <div style="height: 100px;display: flex;justify-content: flex-start;align-items: flex-end;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                管理员，您好：
            </div>
            <div style="display: flex;justify-content: flex-start;flex-direction: column;padding-bottom: 15px;padding-top: 15px;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                <div style="text-indent:30px; margin-bottom: 20px;">
                    平台已于稍早时刻，启动数据库备份计划，下面是执行结果报告：
                </div>
                <div style="height: 30px;text-indent:30px">
                    所属日期：${archive_date}
                </div>
                <div style="height: 30px;text-indent:30px">
                    开始时间：${start_date}
                </div>
                <div style="height: 30px;text-indent:30px">
                    结束时间：${end_date}
                </div>
                <div style="height: 30px;text-indent:30px">
                    执行结果：成功
                </div>
            </div>
            <div style="height: 30px;text-indent:30px;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                如果您看过上述信息，请忽略此电子邮件。
            </div>
            <div style=" height: 30px;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                此致！
            </div>
            <div style="height: 60px;display: flex;flex-direction: column;align-items: flex-end;justify-content: center;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                <div style="width: 200px;display: flex;flex-direction: column;align-items: center;justify-content: center;">
                    <div>极客印记·运营中心</div>
                    <div>${send_date}</div>
                </div>
            </div>
            <div style="height: 150px;display: flex;flex-direction: column;align-items: center;justify-content: center;font-family: 'Geneva', Helvetica, Arial, sans-serif;font-smooth: always;-webkit-font-smoothing: antialiased;font-size: 14px;color: #888;">
                Copyright &copy; <script>document.write(new Date().getFullYear())</script> saya.ac.cn, 极客印记 All Rights Reserved
            </div>
        </div>
    </div>
</body>
</html>"#;
        let now = DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()),&util::FORMAT_Y_M_D_H_M_S);
        let html = html_template.replace("${send_date}",now.unwrap().as_str())
            .replace("${archive_date}",archive_date)
            .replace("${start_date}",start_date)
            .replace("${end_date}",end_date);

        let mut email_builder = Message::builder();

        let email_from = format!("极客印记 <{}>",&CONTEXT.config.from_mail);
        // 发件人
        email_builder = email_builder.from(email_from.parse().unwrap());
        // 收件人
        for item in &CONTEXT.config.to_mail {
            email_builder = email_builder.to(format!("管理员 <{}>",item).parse().unwrap());
        }
        // 主题
        email_builder = email_builder.subject("【极客印记】应用通知");
        // 邮件内容
        let email_message = email_builder.multipart(
            MultiPart::alternative() // This is composed of two parts.
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(String::from(html)),
                ),
        )
        .unwrap();

        // 邮件服务器账号：
        let creds = Credentials::new(String::from(&CONTEXT.config.from_mail), String::from(&CONTEXT.config.mail_token));

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(&CONTEXT.config.mail_server).unwrap().credentials(creds).build();

        // Send the email
        match mailer.send(&email_message) {
            Ok(_) => log::debug!("Email sent successfully!"),
            Err(e) => log::error!("Could not send email: {:?}", e),
        }
    }

    /// 发送计划提醒邮件
    pub fn send_plan_massage(flag:bool,user:&User,plan_contents:Vec<String>) {

        let html_template = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>极客印记-邮件系统</title>
    <meta http-equiv="Content-Type" content="text/html; charset=UTF-8"/>
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="icon" href="https://saya.ac.cn/favicon.svg"/>
</head>
<body>
    <div style="width:available;display: flex;justify-content: center;align-items: center;">
        <div style="width: 700px;">
            <div style="height: 100px;display: flex;justify-content: flex-start;align-items: flex-end;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                ${plan_user}，您好：
            </div>
            <div style="display: flex;justify-content: flex-start;flex-direction: column;padding-bottom: 15px;padding-top: 15px;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                <div style="text-indent:30px; margin-bottom: 20px;">
                    ${plan_title}
                </div>
                ${plan_content}
            </div>
            <div style="height: 30px;text-indent:30px;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                如果您看过上述信息，请忽略此电子邮件。
            </div>
            <div style=" height: 30px;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                此致！
            </div>
            <div style="height: 60px;display: flex;flex-direction: column;align-items: flex-end;justify-content: center;line-height: inherit;font-family: 'Lucida Grande', Helvetica, Arial, sans-serif;font-size: 16px;color: #333;font-smooth: always;-webkit-font-smoothing: antialiased;">
                <div style="width: 200px;display: flex;flex-direction: column;align-items: center;justify-content: center;">
                    <div>极客印记·运营中心</div>
                    <div>${send_date}</div>
                </div>
            </div>
            <div style="height: 150px;display: flex;flex-direction: column;align-items: center;justify-content: center;font-family: 'Geneva', Helvetica, Arial, sans-serif;font-smooth: always;-webkit-font-smoothing: antialiased;font-size: 14px;color: #888;">
                Copyright &copy; <script>document.write(new Date().getFullYear())</script> saya.ac.cn, 极客印记 保留所有权利。
            </div>
        </div>
    </div>
</body>
</html>"#;


        // 拼凑提醒内容
        let mut title = String::new();
        if flag {
            // 执行正常的提醒
            title = String::from("以下是您今天的计划安排，请根据您的情况，合理安排：");
        } else {
            // 执行超期未完成的提醒
            title = String::from("以下是您截止昨天还未完成的计划安排，请根据您的情况，合理安排：");
        }

        // 拼凑提醒内容
        let mut content = String::new();
        let mut index:i32 = 1;
        for item in plan_contents {
            content = content.add(format!("<div style=\"height: 30px;text-indent:30px\">{}、{}</div>",index,item).as_str());
            index = index + 1;
        }
        let now = DateTimeUtil::naive_date_time_to_str(&Some(DateUtils::now()),&util::FORMAT_Y_M_D_H_M_S);
        let html = html_template.replace("${send_date}",now.unwrap().as_str())
            .replace("${plan_user}",user.name.clone().unwrap().as_str())
            .replace("${plan_content}", content.as_str())
            .replace("${plan_title}", title.as_str());
        // 准备收发件人
        let email_from = format!("极客印记 <{}>",&CONTEXT.config.from_mail);
        let to_user = user.name.clone().unwrap();
        let to_add = user.email.clone().unwrap();
        let to_mail = format!("{} <{}>",to_user,to_add);

        let email_builder = Message::builder()
        // 发件人
        .from(email_from.clone().parse().unwrap())
        // 收件人
        .to(to_mail.parse().unwrap())
        // 主题
        .subject("【极客印记】提醒事项");
        // 邮件内容
        let email_message = email_builder.multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(String::from(html)),
                ),
        )
            .unwrap();

        // 邮件服务器账号：
        let creds = Credentials::new(String::from(&CONTEXT.config.from_mail), String::from(&CONTEXT.config.mail_token));

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(&CONTEXT.config.mail_server).unwrap().credentials(creds).build();

        // Send the email
        match mailer.send(&email_message) {
            Ok(_) => log::debug!("Email sent successfully!"),
            Err(e) => log::error!("Could not send email: {:?}", e),
        }
    }


    pub fn send_example() {
        let email = Message::builder()
            // 发件人
            .from("极客印记 <504804540@qq.com>".parse().unwrap())
            // 收件人
            .to("管理员 <saya@saya.ac.cn>".parse().unwrap())
            .to("管理员 <228476495@qq.com>".parse().unwrap())
            // 主题
            .subject("【极客印记】系统通知")
            // 邮件内容
            .body(String::from("Be happy!"))
            .unwrap();

        // 邮件服务器账号：
        let creds = Credentials::new("504804540@qq.com".to_string(), "--------------".to_string());

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.qq.com").unwrap().credentials(creds).build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => log::debug!("Email sent successfully!"),
            Err(e) => log::error!("Could not send email: {:?}", e),
        }
    }
}