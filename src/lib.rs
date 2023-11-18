//!
//! # 阿里云OSS SDK
//! # 阿里云OSS OssClient
//!

/// *阿里云OSS服务地址*
const OSS_BASE_URL: &'static str = "aliyuncs.com";
const DEFAULT_REGION: &'static str = "oss-cn-hangzhou";

pub mod client;
#[allow(unused_imports)]
pub mod common;
pub mod utls;
pub mod types;

pub use self::client::OssClient;
pub use self::common::Authorization;
pub use self::common::Bucket;
pub use self::common::Endpoint;
pub use self::common::OssError;
pub use self::common::OssOptions;
pub use self::common::Signature;

#[cfg(test)]
mod study {
    // use std::rc;

    #[allow(unused)]
    enum List {
        Coin(i32, Box<List>),
        Nil,
    }

    struct Str(String);

    impl Str {
        fn wrapper(content: String) -> Self {
            Str(content)
        }

        fn to_array(&self) -> Vec<char> {
            self.0.chars().collect::<Vec<char>>()
        }
    }

    fn rc_test() {
        // let a: Box<i32> = Box::new(100);
        // let b: Box<i32> = Box::new(200);
        let poem = Str::wrapper(String::from("相见时难别亦难,，😄🍰"));
        for (index, char) in poem.to_array().iter().enumerate()  {
            println!("{} : {}", index, char);
        }
        let poem = "东风无力百花残";
        let str_w = Str::wrapper(poem.to_string());
        println!("{:?}", str_w.to_array());
    }

    #[test]
    fn temp() {
        println!("{}", "=".repeat(80));
        println!();

        rc_test();

        println!();
        println!("{}", "=".repeat(80));
    }
}
