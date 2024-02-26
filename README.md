# XT - Aliyun OSS SDK


![Crates.io MSRV](https://img.shields.io/crates/msrv/xt-oss)
![Crates.io Total Downloads](https://img.shields.io/crates/d/xt-oss)
![maintenance-status](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
![Crates.io License (version)](https://img.shields.io/crates/l/xt-oss/0.4.4)



这是一个`Rust`语言编写的阿里云OSS的SDK，依据官网文档并参考了其他语言的实现。

- 基于`tokio-rs`异步运行时与流行的`reqwest`库实现.
- 尽量完整的OSS数据结构描述（`struct`、`enum`）.
- `Builder`设计模式的传参风格.
- 实现常用的大部分API.
- 完整`Examples`演示.

```toml
[dependencies]
tokio = {version = "1.36.0", features = ["full"]}
xt-oss = "0.4.4"
```

 ```rust no_run
//! `cargo run --example api_region_describe -q`
//!
//! 调用DescribeRegions接口查询所有支持地域或者指定地域对应的Endpoint信息，
//! 包括外网Endpoint、内网Endpoint和传输加速Endpoint。
use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    // let options = oss::Options::new()
    //     .with_access_key_id("-- your access_key_id --")
    //     .with_access_key_secret("-- your access_key_secret --");

    let client = oss::Client::new(options);

    match client
        .DescribeRegions()
        // .with_region("oss-us-east-1")
        .execute()
        .await
        .unwrap_or_else(|reqwest_error| {
            println!("reqweset error: {}", reqwest_error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            oss_data.content().region_info.iter().for_each(|entry| {
                println!("{:>20} | {}", entry.region, entry.internet_endpoint);
            });
        }
        Err(error_message) => {
            // let message = error_message.content();
            println!("request id: {}", &error_message.request_id());
            println!("oss error: {}", &error_message.content());
        }
    }
    Ok(())
}
```

## `Options` 配置

- `access_key_id` 通过阿里云控制台创建的AccessKey ID
- `access_key_secret` 通过阿里云控制台创建的AccessKey Secret
- `sts_token` 使用临时授权方式
- `bucket` 通过控制台或PutBucket创建的Bucket
- `endpoint` OSS访问域名。
- `region` Bucket所在的区域,默认值为oss-cn-hangzhou
- `internal`  是否使用阿里云内网访问,默认值为false
- `cname`  是否支持上传自定义域名,默认值为false
- `is_request_pay` Bucket是否开启请求者付费模,默认值为false
- `secure`  设置secure为true,则使用HTTPS;设置secure为false,则使用HTTP
- `timeout` 超时时间,默认值为60秒

### 构建方式

```rust no_run
use xt_oss::prelude::*;
// 构建方式
let options = oss::Options::new()
    .with_access_key_id("access_key_id")
    .with_access_key_secret("access_key_secret")
    .with_bucket("xtoss-ex1")
    .with_cname(true)
    .with_endpoint("http://cdn-dev.xuetube.com")
    .with_internal(false)
    .with_region("oss-cn-shanghai")
    .with_secret(true)
    // .with_sts_token("sts token")
    .with_timeout(60);

let client = oss::Client::new(options);
```

### 从.env加载,格式参见 .env.example.

```rust no_run
use xt_oss::prelude::*;
// ...
dotenv::dotenv().ok();
let options = util::options_from_env();
let client = oss::Client::new(options);
// ...
```

## 参数构建

## 错误处理

## 其他

## 实现的Api

### 关于Service/Region

- [★ `ListBuckets（GetService）`](oss/struct.Client.html#method.ListBuckets)
- [★ `DescribeRegions`](https://www.example.com) 

### Bucket - 基础操作

- [★ `PutBucket`](https://www.example.com)
- [★ `DeleteBucket`](https://www.example.com)
- [★ `ListObjects`](https://www.example.com)
- [★ `ListObjectsV2`](https://www.example.com)
- [★ `GetBucketInfo`](https://www.example.com) 
- [★ `GetBucketLocation`](https://www.example.com) 
- [★ `GetBucketStat`](https://www.example.com) 

### 合规保留策略（WORM）

- [★ `InitiateBucketWorm`](https://www.example.com)
- [★ `AbortBucketWorm`](https://www.example.com)
- [★ `CompleteBucketWorm`](https://www.example.com)
- [★ `ExtendBucketWorm`](https://www.example.com)
- [★ `GetBucketWorm`](https://www.example.com)

### Bucket 权限控制（ACL）

- [★ `PutBucketAcl`](https://www.example.com)
- [★ `GetBucketAcl`](https://www.example.com)

### Bucket 生命周期（Lifecycle）

- [★ `PutBucketLifecycle`](https://www.example.com)
- [★ `GetBucketLifecycle`](https://www.example.com)
- [★ `DeleteBucketLifecycle`](https://www.example.com)

### Bucket 传输加速（TransferAcceleration）

- [★ `PutBucketTransferAcceleration`](https://www.example.com)
- [★ `GetBucketTransferAcceleration`](https://www.example.com)

### Bucket 版本控制（Versioning）

- [★ `PutBucketVersioning`](https://www.example.com)
- [★ `GetBucketVersioning`](https://www.example.com)
- [☆ `ListObjectVersions（GetBucketVersions ）x`](https://www.example.com)

<!-- **Bucket 数据复制（Replication）**

- [ ] `PutBucketReplication`
- [ ] `PutBucketRTC`
- [ ] `GetBucketReplication`
- [ ] `GetBucketReplicationLocation`
- [ ] `GetBucketReplicationProgress`
- [ ] `DeleteBucketReplication` -->

### Bucket 授权策略（Policy）

- [★ `PutBucketPolicy`](https://www.example.com)
- [★ `GetBucketPolicy`](https://www.example.com)
- [★ `DeleteBucketPolicy`](https://www.example.com)

<!-- ###### Bucket 清单（Inventory）

- [ ] `PutBucketInventory`
- [ ] `GetBucketInventory`
- [ ] `ListBucketInventory`
- [ ] `DeleteBucketInventory` -->

### Bucket 日志管理（Logging）

- [★ `PutBucketLogging`](https://www.example.com)
- [★ `GetBucketLogging`](https://www.example.com)
- [★ `DeleteBucketLogging`](https://www.example.com)

### Bucket 静态网站（Website）

- [★ `PutBucketWebsite`](https://www.example.com)?
- [★ `GetBucketWebsite`](https://www.example.com)
- [★ `DeleteBucketWebsite`](https://www.example.com)

### Bucket 防盗链（Referer）

- [★ `PutBucketReferer`](https://www.example.com)
- [★ `GetBucketReferer`](https://www.example.com)

### Bucket 标签（Tags）

- [★ `PutBucketTags`](https://www.example.com)
- [★ `GetBucketTags`](https://www.example.com)
- [★ `DeleteBucketTags`](https://www.example.com)

### Bucket 加密（Encryption）

- [★ `PutBucketEncryption`](https://www.example.com)
- [★ `GetBucketEncryption`](https://www.example.com)
- [★ `DeleteBucketEncryption`](https://www.example.com)

<!-- ### Bucket 请求者付费（RequestPayment）

- [☆ `PutBucketRequestPayment`](https://www.example.com)
- [☆ `GetBucketRequestPayment`](https://www.example.com) -->

### Bucket 跨域资源共享（CORS）

- [★ `PutBucketCors`](https://www.example.com)
- [★ `GetBucketCors`](https://www.example.com)
- [★ `DeleteBucketCors`](https://www.example.com)
- [★ `Options`](https://www.example.com)

<!-- ##### Bucket 访问跟踪（AccessMonitor）

- [ ] `PutBucketAccessMonitor`
- [ ] `GetBucketAccessMonitor`

##### Bucket 数据索引（Data Indexing）

- [ ] `OpenMetaQuery`
- [ ] `GetMetaQueryStatus`
- [ ] `DoMetaQuery`
- [ ] `CloseMetaQuery`

##### Bucket 资源组（Resource Group）

- [ ] `PutBucketResourceGroup`
- [ ] `GetBucketResourceGroup` -->

### Bucket 自定义域名（CNAME）

- [★ `CreateCnameToken`](https://www.example.com)
- [★ `GetCnameToken`](https://www.example.com)
- [★ `PutCname`](https://www.example.com)?
- [★ `ListCname`](https://www.example.com)
- [★ `DeleteCname`](https://www.example.com)

### Bucket 图片样式（Style）

- [★ `PutStyle`](https://www.example.com)
- [★ `GetStyle`](https://www.example.com)
- [★ `ListStyle`](https://www.example.com)
- [★ `DeleteStyle`](https://www.example.com)

<!-- ##### Bucket  安全传输层协议（TLS）

- [ ] `PutBucketHttpsConfig`
- [ ] `GetBucketHttpsConfig`

### Bucket  存储冗余转换（RedundancyTransition）

- [ ] `CreateBucketDataRedundancyTransition`
- [ ] `GetBucketDataRedundancyTransition`
- [ ] `DeleteBucketDataRedundancyTransition`
- [ ] `ListUserDataRedundancyTransition`
- [ ] `ListBucketDataRedundancyTransition`

##### Bucket  接入点（AccessPoint）

- [ ] `CreateAccessPoint`
- [ ] `GetAccessPoint`
- [ ] `DeleteAccessPoint`
- [ ] `ListAccessPoints`
- [ ] `PutAccessPointPolicy`
- [ ] `GetAccessPointPolicy`
- [ ] `DeleteAccessPointPolicy`

##### Bucket  对象FC接入点（Object FC AccessPoint）

- [ ] `CreateAccessPointForObjectProcess`
- [ ] `GetAccessPointForObjectProcess`
- [ ] `DeleteAccessPointForObjectProcess`
- [ ] `ListAccessPointsForObjectProcess`
- [ ] `PutAccessPointConfigForObjectProcess`
- [ ] `GetAccessPointConfigForObjectProcess`
- [ ] `PutAccessPointPolicyForObjectProcess`
- [ ] `GetAccessPointPolicyForObjectProcess`
- [ ] `DeleteAccessPointPolicyForObjectProcess`
- [ ] `WriteGetObjectResponse` -->

### Object 基础操作 Stand

- [★ `PutObject`](https://www.example.com)
- [★ `GetObject`](https://www.example.com)
- [★ `CopyObject`](https://www.example.com)
- [★ `AppendObject`](https://www.example.com)
- [★ `DeleteObject`](https://www.example.com)
- [★ `DeleteMultipleObjects`](https://www.example.com)
- [★ `HeadObject`](https://www.example.com)
- [★ `GetObjectMeta`](https://www.example.com)
<!-- - [☆ `PostObject`](https://www.example.com) -->
<!-- - [☆ `Callback`](https://www.example.com) -->
<!-- - [☆ `RestoreObject`](https://www.example.com) -->
<!-- - [☆ `SelectObject`](https://www.example.com) -->

### Object 分片上传（MultipartUpload）

- [★ `InitiateMultipartUpload`](https://www.example.com)
- [★ `UploadPart`](https://www.example.com)
- [★ `UploadPartCopy`](https://www.example.com)
- [★ `CompleteMultipartUpload`](https://www.example.com)
- [★ `AbortMultipartUpload`](https://www.example.com)
- [★ `ListMultipartUploads`](https://www.example.com)
- [★ `ListParts`](https://www.example.com)

### Object 权限控制（ACL)

- [★ `PutObjectACL`](https://www.example.com)
- [★ `GetObjectACL`](https://www.example.com)

### Object 软链接（Symlink）

- [★ `PutSymlink`](https://www.example.com)
- [★ `GetSymlink`](https://www.example.com)

### Object 标签（Tagging）

- [★ `PutObjectTagging`](https://www.example.com)
- [★ `GetObjectTagging`](https://www.example.com)
- [★ `DeleteObjectTagging`](https://www.example.com)

<!-- ### 关于LiveChannel的操作

- [☆ `PutLiveChannel`](https://www.example.com)
- [☆ `ListLiveChannel`](https://www.example.com)
- [☆ `DeleteLiveChannel`](https://www.example.com)
- [☆ `PutLiveChannelStatus`](https://www.example.com)
- [☆ `GetLiveChannelInfo`](https://www.example.com)
- [☆ `GetLiveChannelStat`](https://www.example.com)
- [☆ `GetLiveChannelHistory`](https://www.example.com)
- [☆ `PostVodPlaylist`](https://www.example.com)
- [☆ `GetVodPlaylist`](https://www.example.com) -->