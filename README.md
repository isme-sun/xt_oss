# XT - Aliyun OSS SDK

![Crates.io MSRV](https://img.shields.io/crates/msrv/xt-oss)
![Crates.io Total Downloads](https://img.shields.io/crates/d/xt-oss)
![maintenance-status](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
![Crates.io License (version)](https://img.shields.io/crates/l/xt-oss/0.4.4)

`Rust`语言编写的阿里云OSS的SDK，依据官方文档并参考了其他语言的实现。

- 基于`tokio-rs`异步运行时与流行的`reqwest`库实现.
- 尽量完整的OSS数据结构描述（`struct`、`enum`）.
- `Builder`设计模式的传参风格.
- 实现常用的大部分API.
- 完整`Examples`演示.


```toml
[dependencies]
tokio = {version = "1.36.0", features = ["full"]}
xt-oss = "0.5.7"
#example 可选 dirs = "5.0.1" 
#example 可选 dotenv = "0.15.0"
#example 可选 serde_json = "1.0.114"
```

```rust ignore
//! `cargo run --example api_region_describe -q`
//! 调用DescribeRegions接口查询所有支持地域或者指定地域对应的Endpoint信息，
use dotenv;
use std::process;
use xt_oss::{
    oss::entities::region::RegionInfo,
    prelude::*,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv::dotenv().ok();
  // 从环境生成 oss::Options;
  let options = util::options_from_env();
  // builder oss::Options
  // let options = oss::Options::new()
  //     .with_access_key_id("-- your access_key_id --")
  //     .with_access_key_secret("-- your access_key_secret --");
  // 创建oss::Client
  let client = oss::Client::new(options);

  match client
    .DescribeRegions()
    // 可选参数
    // .with_region("oss-us-east-1")
    .execute()
    .await
    // 处理可能的reqwest错误
    .unwrap_or_else(|reqwest_error| {
        println!("reqweset error: {}", reqwest_error);
        process::exit(-1);
    }) {
    // 请求正常返回结果
    Ok(oss_data) => {
        let regions: Vec<RegionInfo> = oss_data.content().region_info;
        for e in regions {
            println!("{:>20} | {}", e.region, e.internet_endpoint);
        }
    }
    // 请求正常，返回oss错误消息
    Err(error_message) => {
        // let message = error_message.content();
        println!("request id: {}", &error_message.request_id());
        println!("oss error: {}", &error_message.content());
    }
  }
  Ok(())
}
```

## 一、 Example

- [批量创建oss buckets](https://github.com/isme-sun/xt_oss/tree/main/examples/aa_create_buckets.rs)
- [大文件分段下载](https://github.com/isme-sun/xt_oss/tree/main/examples/aa_get_big_object.rs)
- [Loop列出所有object](https://github.com/isme-sun/xt_oss/tree/main/examples/aa_list_object_v2_all.rs)
- [上传本地目录到bucket](https://github.com/isme-sun/xt_oss/tree/main/examples/aa_sync_sample_up.rs)
- [下载bucket内所有文件到本地](https://github.com/isme-sun/xt_oss/tree/main/examples/aa_sync_smaple_down.rs)
- [更多...](https://github.com/isme-sun/xt_oss/tree/main/examples)

## 二、 `Options` 配置

- `access_key_id` 通过阿里云控制台创建的AccessKey ID
- `access_key_secret` 通过阿里云控制台创建的AccessKey Secret
- `sts_token` 使用临时授权方式
- `bucket` 通过控制台或PutBucket创建的Bucket
- `endpoint` OSS访问域名。  
- `region` Bucket所在的区域,默认值为oss-cn-hangzhou
- `internal`  是否使用阿里云内网访问,默认值为false
- `cname`  是否支持上传自定义域名,默认值为false
- ~~`is_request_pay` Bucket是否开启请求者付费模,默认值为false~~
- `secure`  设置secure为true,则使用HTTPS;设置secure为false,则使用HTTP
- `timeout` 超时时间,默认值为60秒

> 当`cname`为true时,`endpoint`,`bucket`为必填,否则产生panic错误.
> 当internal为true时，忽略cname与endpoint
> 无论是否使用cname正确的设置region(location)与bucket

### 构建方式生成oss::Options

```rust ignore
use xt_oss::prelude::*;
// 构建方式
let options = oss::Options::new()
    .with_access_key_id("access_key_id")
    .with_access_key_secret("access_key_secret")
    .with_region("oss-cn-shanghai")
    .with_bucket("xtoss-ex")
    .with_secret(true)
    .with_internal(false);

let root_url = "https://oss-cn-hangzhou.aliyuncs.com";
let base_url = "https://xtoss-ex.oss-cn-shanghai.aliyuncs.com";

assert_eq!(options.root_url(), root_url);
assert_eq!(options.base_url(), base_url);

let client = oss::Client::new(options);
```

### 环境变量生成oss::Options,格式参见 [.env.example](https://github.com/isme-sun/xt_oss/blob/main/.env.example)

```rust ignore
use xt_oss::prelude::*;
// ...
dotenv::dotenv().ok();
let options = util::options_from_env();
let client = oss::Client::new(options);
// ...
```

## 三、 Api方法与参数构建

![xtoss-2](https://cdn.xuetube.com/upload/xtoss/xtoss-2.png)

api方法命名遵循官方文档，例如 `ListObjectsV2`,`DescribeRegions`,熟悉官方文档并结合
代码提示将给库的使用带来方便.

- 参数构建分为简单方式，直接在方法内传参数例如`.HeadObject("mp3/Audio_0.4mb.mp3")`
- with_ 传参数,例如:.  `PutSymlink("tmp/test.txt").with_symlink_target("target.txt")`
- 参数builder构建 例如:

```rust ignore
 // 构建参数
let index_document = IndexDocumentBuilder::new()
    .with_suffix("index.html")
    .with_support_sub_dir(true)
    .with_type(0)
    .build();
let error_document = ErrorDocumentBuilder::new()
    .with_http_status(StatusCode::NOT_FOUND)
    .with_key("error.html")
    .build();
let config = WebsiteConfigurationBuilder::new()
    .with_index_document(index_document)
    .with_error_document(error_document)
    // .with_routing_rules(rules)
    .build();
// 发出请求
let result = client
    .PutBucketWebsite()
    .with_config(config)
    .execute()
    .await
// ...
```

## 四、 返回与错误处理

```rust ignore
pub type ApiResponse<T> = Result<ApiData<T>, ApiData<ErrorMessage>>;
pub type ApiResult<T = ()> = Result<ApiResponse<T>, reqwest::Error>;
```

```rust ignore
//...
match client
    .GetObjectTagging("excel/Spreadsheet-1000-rows.xls")
    .execute()
    .await
{
    Ok(Ok(data)) => {
        // data:ApiData<Tagging>
        println!("{}", data.request_id());
        println!("{:#?}", data.headers());
        println!("{:#?}", data.content());
    }
    Ok(Err(message)) => {
        // message: ApiData<ErrorMessage>
        println!("{}", message.request_id());
        println!("{:#?}", message.headers());
        println!("{:#?}", message.content());
    }
    Err(reqwest_error) => println!("{}", reqwest_error),
}
//...
```

## 五、util提供一些工具方法

- `fn utc_to_gmt(datetime:DateTime<Utc>) -> String`
- `fn local_to_gmt(local_datetime: DateTime<Local>) -> String`
- `fn options_from_env() -> oss::Options<'static>`
- `fn oss_file_md5`
- `fn oss_md5`
- `struct ByteRange`

### 关于ByteRange 

提供了HTTP Range构造方法

```rust ignore
assert_eq!(ByteRange::new().to_string(), "bytes=0-");
assert_eq!(ByteRange::new().with_amount(500).to_string(), "bytes=0-499");
assert_eq!(ByteRange::new().with_amount(-500).to_string(), "bytes=-500");
assert_eq!(ByteRange::new().with_start(100).to_string(), "bytes=100-");
// 通过元组生成
assert_eq!(ByteRange::from((100, 500)).to_string(), "bytes=100-599");
assert_eq!(ByteRange::from((100, -500)).to_string(), "bytes=0-99");
assert_eq!(ByteRange::from((100, -50)).to_string(), "bytes=50-99");
```


## 六、 TODO

- 完成剩下的api,修复bug
- 逐步完善文档
- 提供一些开箱即用的utils功能

## 七、 欢迎提bug和需求

欢迎大家提出bug报告和功能需求。如果你在使用过程中遇到了任何问题或者有任何改进的建议，都可以在[Issues](https://github.com/isme-sun/xt_oss/issues)中告知。

- 邮箱：[`isme.sun@icloud.com`](mailto:isme.sun@icloud.com)
- 微信：`ismeSun`
- github: [`xt-oss`](https://github.com/isme-sun/xt_oss)

-----------------------------------------------------------------------------------

## 附录:实现的Api

下面是计划中要实现的Api,★ 已经实现 ☆ 未实现.

### 关于Service/Region

- [★ `ListBuckets（GetService）`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `DescribeRegions`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Bucket - 基础操作

- [★ `PutBucket`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `DeleteBucket`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `ListObjects`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `ListObjectsV2`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetBucketInfo`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetBucketLocation`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetBucketStat`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### 合规保留策略（WORM）

- [★ `InitiateBucketWorm`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `AbortBucketWorm`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `CompleteBucketWorm`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `ExtendBucketWorm`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetBucketWorm`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Bucket 权限控制（ACL）

- [★ `PutBucketAcl`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetBucketAcl`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Bucket 生命周期（Lifecycle）

- [★ `PutBucketLifecycle`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetBucketLifecycle`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `DeleteBucketLifecycle`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Bucket 传输加速（TransferAcceleration）

- [★ `PutBucketTransferAcceleration`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetBucketTransferAcceleration`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Bucket 版本控制（Versioning）

- [★ `PutBucketVersioning`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetBucketVersioning`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `ListObjectVersions(GetBucketVersions)`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Bucket 授权策略（Policy）

- [★ `PutBucketPolicy`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetBucketPolicy`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `DeleteBucketPolicy`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Bucket 日志管理（Logging）

- [★ `PutBucketLogging`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetBucketLogging`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `DeleteBucketLogging`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Bucket 静态网站（Website）

- [★ `PutBucketWebsite`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetBucketWebsite`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `DeleteBucketWebsite`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Bucket 防盗链（Referer）

- [★ `PutBucketReferer`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetBucketReferer`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Bucket 标签（Tags）

- [★ `PutBucketTags`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetBucketTags`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `DeleteBucketTags`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Bucket 加密（Encryption）

- [★ `PutBucketEncryption`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetBucketEncryption`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `DeleteBucketEncryption`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Bucket 请求者付费（RequestPayment）

- [★ `PutBucketRequestPayment`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetBucketRequestPayment`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Bucket 跨域资源共享（CORS）

- [★ `PutBucketCors`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetBucketCors`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `DeleteBucketCors`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `Options`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Bucket 自定义域名（CNAME）

- [★ `CreateCnameToken`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetCnameToken`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `PutCname`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `ListCname`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `DeleteCname`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Bucket 图片样式（Style）

- [★ `PutStyle`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetStyle`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `ListStyle`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `DeleteStyle`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Object 基础操作 Stand

- [★ `PutObject`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetObject`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `CopyObject`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `AppendObject`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `DeleteObject`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `DeleteMultipleObjects`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `HeadObject`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetObjectMeta`](https://github.com/isme-sun/xt_oss/tree/main/examples)
<!-- - [☆ `PostObject`](https://github.com/isme-sun/xt_oss/tree/main/examples) -->
- [☆ `Callback`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [☆ `RestoreObject`](https://github.com/isme-sun/xt_oss/tree/main/examples)
<!-- - [☆ `SelectObject`](https://github.com/isme-sun/xt_oss/tree/main/examples) -->

### Object 分片上传（MultipartUpload）

- [★ `InitiateMultipartUpload`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `UploadPart`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `UploadPartCopy`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `CompleteMultipartUpload`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `AbortMultipartUpload`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `ListMultipartUploads`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `ListParts`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Object 权限控制（ACL)

- [★ `PutObjectACL`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetObjectACL`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Object 软链接（Symlink）

- [★ `PutSymlink`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetSymlink`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### Object 标签（Tagging）

- [★ `PutObjectTagging`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `GetObjectTagging`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [★ `DeleteObjectTagging`](https://github.com/isme-sun/xt_oss/tree/main/examples)

### 关于LiveChannel的操作

- [☆ `PutLiveChannel`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [☆ `ListLiveChannel`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [☆ `DeleteLiveChannel`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [☆ `PutLiveChannelStatus`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [☆ `GetLiveChannelInfo`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [☆ `GetLiveChannelStat`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [☆ `GetLiveChannelHistory`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [☆ `PostVodPlaylist`](https://github.com/isme-sun/xt_oss/tree/main/examples)
- [☆ `GetVodPlaylist`](https://github.com/isme-sun/xt_oss/tree/main/examples)
