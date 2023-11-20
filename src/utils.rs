//! 常用工具方法合集

use chrono::prelude::*;
use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use hmacsha1;

/// 通用base64编码
pub fn base64_encode() {
    todo!()
}

/// base64解码
pub fn base64_decode() {
    todo!()
}

/// 给出字符串的md5值
pub fn md5(text: &String) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(&text[..]);
    let hex = hasher.result_str();
    hex
}

// 计算给出字符串的sha1加密值
pub fn sha1(text: &String) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(&text[..]);
    let hex = hasher.result_str();
    hex.to_string()
}

/// hmac sha1 计算
pub fn hmac_sha1(message: &String, key: &String) -> String {
    let key = key.as_bytes();
    let message = message.as_bytes();
    let hash = hmacsha1::hmac_sha1(key, message);
    hash.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

// 获取GMT时间格式
pub fn get_gmt_date(dt: &DateTime<Utc>) -> String {
    let fmt = "%a, %d %b %Y %H:%M:%S GMT";
    dt.format(fmt).to_string()
}

#[cfg(test)]
mod tests {

    const BODY_CONTENT: &'static str = r#"bodystring"#;
    const KEY: &'static str = "secret_key";
    const COMPUTED_HMAC: &'static str = "97049623b0e5d20bf6beb5313d80600e3d6abe56";

    #[test]
    fn sha1_test() {
        use crate::utils::sha1;
        let sha1_result = "87664ede859cdfe9d3fe93776a75089966067b66";
        let content = String::from("xuetube.com");
        let retval = sha1(&content);
        assert_eq!(sha1_result, retval);
    }

    #[test]
    fn gmt_test() {
        use crate::utils::get_gmt_date;
        use chrono::DateTime;
        use chrono::{TimeZone, Utc};

        let gmt_dt = "Fri, 15 Sep 2023 12:30:45 GMT";
        let date_time: DateTime<Utc> = Utc.with_ymd_and_hms(2023, 09, 15, 12, 30, 45).unwrap();

        println!("{}", gmt_dt);
        println!("{}", get_gmt_date(&date_time));
        assert_eq!(gmt_dt, get_gmt_date(&date_time).to_string());
    }

    #[test]
    fn md5_test() {
        use crate::utils::md5;

        let the_str = "xuetube.com".to_string();
        let the_md5_str = "1e836e01950a931cf446df1be70bb2f4".to_string();

        let result = md5(&the_str);
        println!("{}", result);
        assert_eq!(the_md5_str, result);
    }

    #[test]
    fn hmac_sha1_test() {
        use crate::utils::hmac_sha1;
        let content = BODY_CONTENT.to_string();
        let key = KEY.to_string();
        let sha1_str = hmac_sha1(&content, &key);
        println!("{}", sha1_str);
        assert_eq!(sha1_str, COMPUTED_HMAC.to_string());
    }

    #[test]
    fn base64_test() {
        let source = "孙健勇".to_string();
        let content = "5a2Z5YGl5YuH".to_string();
        let value = base64_url::encode(&source);
        println!("{}", value);
        // println!("SGVsbG8gV29ybGQu");
        assert_eq!(content, value);
    }

}
