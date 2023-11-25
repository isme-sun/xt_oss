//!
//! # 阿里云OSS SDK
//! # 阿里云OSS OssClient
//!

/// *阿里云OSS服务地址*
const OSS_BASE_URL: &'static str = "aliyuncs.com";

const DEFAULT_REGION: &'static str = "oss-cn-hangzhou";

pub mod client;
pub mod common;
pub mod params;

pub use self::client::OssClient;
pub use self::common::OssError;
pub use self::common::OssOptions;