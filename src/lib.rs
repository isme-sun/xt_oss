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
pub mod types;
pub mod utls;

pub use self::client::OssClient;
pub use self::common::Endpoint;
pub use self::common::OssError;
pub use self::common::OssOptions;
pub use self::common::Signature;