use itertools::Itertools;
use regex::Regex;

pub struct Editor {}

impl Editor {
    pub fn get_content(val: &str) -> String {
        let regex_html_tag = Regex::new(r"<[^>]+>").unwrap();
        // 第一次移除html标签
        let post_regex_html_1 = regex_html_tag.replace_all(val, "");

        // 转markdown 转换成 html
        let html: String = markdown::to_html(&*post_regex_html_1); //markdown::to_html("__我是 *markdown*__");

        // 第二次移除html标签
        let post_regex_html_2 = regex_html_tag.replace_all(html.as_str(), "");

        let regex_space = Regex::new(r"\\s*|\t|\r|\n").unwrap();
        // 移除换行等制表符
        let post_tab = regex_space.replace_all(&post_regex_html_2, "");

        // 移除转义后的空格字符
        let post_space1 = post_tab.replace("&amp;nbsp;", "");
        let post_space2 = post_space1.replace("&amp;", "");

        // 彻底移除空格
        let result = post_space2.split_whitespace().format("").to_string();

        if result.is_empty() {
            return String::from("......");
        }
        return if result.chars().count() <= 150 {
            result
        } else {
            // 因为字符串的编码不同。不能直接通过下标访问
            let mut i = 0;
            let mut sub_string = String::new();
            for item in result.chars() {
                if i < 150 {
                    sub_string.push(item);
                    i = i + 1;
                }
            }
            format!("{}......(更多)", sub_string)
        };
    }
}
