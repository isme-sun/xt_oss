# XT - Aliyun OSS SDK

> 参考官方其他SDK实现的阿里云OSS SDK

## 一、简介

内容

## 二、应用示例

 ```rust
use std::process;
use xt_oss::oss;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = oss::Options::new()
        .with_access_key_id("-- access key id --")
        .with_access_key_secret("-- access key secret --");

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
            oss_data
                .content()
                .region_info
                .into_iter()
                .for_each(|entry| {
                    println!("{:>20} | https://{}", entry.region, entry.internet_endpoint);
                });
        }
        Err(oss_error_message) => println!("oss error: {}", oss_error_message.content()),
    }
    Ok(())
}

 ```

## 三、配置说明

 概要说明 ...

 ```rust
 fn main() {
     println!("示例说明")
 }
 ```

## 四、参数与返回数据

内容

 ```rust
 fn main() {
     println!("示例说明")
 }
 ```

## 四、实现的API

简介

### 关于Service/Region

- [★ `ListBuckets（GetService）`](oss/struct.Client.html#method.ListBuckets) o
- [★ `DescribeRegions`](https://www.example.com) o

### Bucket - 基础操作

- [★ `PutBucket`](https://www.example.com) o
- [★ `DeleteBucket`](https://www.example.com) o
- [★ `ListObjects`](https://www.example.com)
- [★ `ListObjectsV2`](https://www.example.com)
- [★ `GetBucketInfo`](https://www.example.com) o
- [★ `GetBucketLocation`](https://www.example.com) o
- [★ `GetBucketStat`](https://www.example.com) o

### 合规保留策略（WORM）

- [★ `InitiateBucketWorm`](https://www.example.com)
- [★ `AbortBucketWorm`](https://www.example.com)
- [★ `CompleteBucketWorm`](https://www.example.com)
- [★ `ExtendBucketWorm`](https://www.example.com)
- [★ `GetBucketWorm`](https://www.example.com)

### Bucket 权限控制（ACL）

- [★ `PutBucketAcl`](https://www.example.com) o
- [★ `GetBucketAcl`](https://www.example.com) o

### Bucket 生命周期（Lifecycle）

- [★ `PutBucketLifecycle`](https://www.example.com)
- [★ `GetBucketLifecycle`](https://www.example.com)
- [★ `DeleteBucketLifecycle`](https://www.example.com)

### Bucket 传输加速（TransferAcceleration）

- [★ `PutBucketTransferAcceleration`](https://www.example.com) o
- [★ `GetBucketTransferAcceleration`](https://www.example.com) o

### Bucket 版本控制（Versioning）

- [★ `PutBucketVersioning`](https://www.example.com) o
- [★ `GetBucketVersioning`](https://www.example.com) o
- [☆ `ListObjectVersions（GetBucketVersions ）x`](https://www.example.com)

<!-- **Bucket 数据复制（Replication）**

- [ ] `PutBucketReplication`
- [ ] `PutBucketRTC`
- [ ] `GetBucketReplication`
- [ ] `GetBucketReplicationLocation`
- [ ] `GetBucketReplicationProgress`
- [ ] `DeleteBucketReplication` -->

### Bucket 授权策略（Policy）

- [☆ `PutBucketPolicy`](https://www.example.com)
- [☆ `GetBucketPolicy`](https://www.example.com)
- [☆ `DeleteBucketPolicy`](https://www.example.com)

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

- [★ `PutBucketWebsite`](https://www.example.com)
- [★ `GetBucketWebsite`](https://www.example.com)
- [★ `DeleteBucketWebsite`](https://www.example.com)

### Bucket 防盗链（Referer）

- [★ `PutBucketReferer`](https://www.example.com)
- [★ `GetBucketReferer`](https://www.example.com)

### Bucket 标签（Tags）

- [★ `PutBucketTags`](https://www.example.com) o
- [★ `GetBucketTags`](https://www.example.com) o
- [★ `DeleteBucketTags`](https://www.example.com) o

### Bucket 加密（Encryption）

- [★ `PutBucketEncryption`](https://www.example.com)
- [★ `GetBucketEncryption`](https://www.example.com)
- [★ `DeleteBucketEncryption`](https://www.example.com)

### Bucket 请求者付费（RequestPayment）

- [☆ `PutBucketRequestPayment`](https://www.example.com)
- [☆ `GetBucketRequestPayment`](https://www.example.com)

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
- [★ `PutCname`](https://www.example.com)
- [★ `ListCname`](`xt_oss::oss::Client.ListCname`)
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

- [☆ `PutObject`](https://www.example.com)
- [☆ `GetObject`](https://www.example.com)
- [☆ `CopyObject`](https://www.example.com)
- [☆ `AppendObject`](https://www.example.com)
- [☆ `DeleteObject`](https://www.example.com)
- [☆ `DeleteMultipleObjects`](https://www.example.com)
- [☆ `HeadObject`](https://www.example.com)
- [☆ `GetObjectMeta`](https://www.example.com)
- [☆ `PostObject`](https://www.example.com)
- [☆ `Callback`](https://www.example.com)
- [☆ `RestoreObject`](https://www.example.com)
- [☆ `SelectObject`](https://www.example.com)

### Object 分片上传（MultipartUpload）

- [☆ `InitiateMultipartUpload`](https://www.example.com)
- [☆ `UploadPart`](https://www.example.com)
- [☆ `UploadPartCopy`](https://www.example.com)
- [☆ `CompleteMultipartUpload`](https://www.example.com)
- [☆ `AbortMultipartUpload`](https://www.example.com)
- [☆ `ListMultipartUploads`](https://www.example.com)
- [☆ `ListParts`](https://www.example.com)

### Object 权限控制（ACL)

- [★ `PutObjectACL`](https://www.example.com) o
- [★ `GetObjectACL`](https://www.example.com) o

### Object 软链接（Symlink）

- [★ `PutSymlink`](https://www.example.com)
- [★ `GetSymlink`](https://www.example.com)

### Object 标签（Tagging）

- [★ `PutObjectTagging`](https://www.example.com) o
- [★ `GetObjectTagging`](https://www.example.com) o
- [★ `DeleteObjectTagging`](https://www.example.com) o

### 关于LiveChannel的操作

- [☆ `PutLiveChannel`](https://www.example.com)
- [☆ `ListLiveChannel`](https://www.example.com)
- [☆ `DeleteLiveChannel`](https://www.example.com)
- [☆ `PutLiveChannelStatus`](https://www.example.com)
- [☆ `GetLiveChannelInfo`](https://www.example.com)
- [☆ `GetLiveChannelStat`](https://www.example.com)
- [☆ `GetLiveChannelHistory`](https://www.example.com)
- [☆ `PostVodPlaylist`](https://www.example.com)
- [☆ `GetVodPlaylist`](https://www.example.com)

## 关于oss::Request

```rust
use std::env;

use xt_oss::oss::Request;
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let access_key_id = env::var("OSS_ACCESS_KEY_ID").unwrap();
    let access_key_secret = env::var("OSS_ACCESS_KEY_SECRET").unwrap();
    let url = "https://oss-cn-hangzhou.aliyuncs.com/?regions=oss-us-west-1";
    // let url = "https://oss-cn-hangzhou.aliyuncs.com/?regions";

    let resp = Request::new()
        .with_access_key_id(&access_key_id)
        .with_access_key_secret(&access_key_secret)
        .task()
        .with_url(&url)
        // default Method::GET
        // .with_method(http::Method::GET)
        .execute_timeout(30)
        // default timeout = 60
        // .execute()
        .await;

    match resp {
        Ok(resp) => {
            let bytes = resp.bytes().await.unwrap();
            let content = String::from_utf8_lossy(&bytes);
            println!("{}", content);
        }
        Err(error) => {
            println!("reqwest error: {}", error)
        }
    }
}
```

```rust
use std::env;
use xt_oss::oss::Request;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let access_key_id = env::var("OSS_ACCESS_KEY_ID").unwrap();
    let access_key_secret = env::var("OSS_ACCESS_KEY_SECRET").unwrap();
    let url = "https://oss-cn-hangzhou.aliyuncs.com";

    let resp = Request::new()
        .with_access_key_id(&access_key_id)
        .with_access_key_secret(&access_key_secret)
        .task()
        .with_url(&url)
        // default Method::GET
        // .with_method(http::Method::GET)
        .execute()
        .await;

    match resp {
        Ok(resp) => {
            let bytes = resp.bytes().await.unwrap();
            let content = String::from_utf8_lossy(&bytes);
            println!("{}", content);
        }
        Err(error) => {
            println!("reqwest error: {}", error)
        }
    }
}

```

```rust
use std::env;
use xt_oss::oss::{self, http, Request};
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let access_key_id = env::var("OSS_ACCESS_KEY_ID").unwrap();
    let access_key_secret = env::var("OSS_ACCESS_KEY_SECRET").unwrap();
    let url = "https://xtoss-t1.oss-cn-shanghai.aliyuncs.com/?cors";

    let cors_config = r#"<?xml version="1.0" encoding="UTF-8"?>
<CORSConfiguration>
    <CORSRule>
        <AllowedOrigin>*</AllowedOrigin>
        <AllowedMethod>PUT</AllowedMethod>
        <AllowedMethod>GET</AllowedMethod>
        <AllowedHeader>Authorization</AllowedHeader>
    </CORSRule>
    <CORSRule>
        <AllowedOrigin>http://example.com</AllowedOrigin>
        <AllowedOrigin>http://example.net</AllowedOrigin>
        <AllowedMethod>GET</AllowedMethod>
        <AllowedHeader> Authorization</AllowedHeader>
        <ExposeHeader>x-oss-test</ExposeHeader>
        <ExposeHeader>x-oss-test1</ExposeHeader>
        <MaxAgeSeconds>100</MaxAgeSeconds>
    </CORSRule>
    <ResponseVary>false</ResponseVary>
</CORSConfiguration >"#
        .to_string();

    let data = oss::Bytes::from(cors_config);

    let resp = Request::new()
        .with_access_key_id(&access_key_id)
        .with_access_key_secret(&access_key_secret)
        .task()
        .with_url(&url)
        .with_resource("/xtoss-t1/?cors")
        .with_method(http::Method::PUT)
        .with_body(data)
        .execute_timeout(30)
        .await;

    match resp {
        Ok(resp) => {
            println!("is success: {}", resp.status().is_success());
            let status = resp.status();
            let bytes = resp.bytes().await.unwrap();
            let content = String::from_utf8_lossy(&bytes);
            println!("{}", status);
            println!("{}", content);
        }
        Err(error) => {
            println!("{:#?}", error);
        }
    }
}

```

```rust
use std::env;
use xt_oss::oss::Request;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let access_key_id = env::var("OSS_ACCESS_KEY_ID").unwrap();
    let access_key_secret = env::var("OSS_ACCESS_KEY_SECRET").unwrap();
    let url = "https://xtoss-t1.oss-cn-shanghai.aliyuncs.com/?cors";
    let resp = Request::new()
        .with_access_key_id(&access_key_id)
        .with_access_key_secret(&access_key_secret)
        .task()
        .with_url(&url)
        .with_resource("/xtoss-t1/?cors")
        .execute_timeout(30)
        .await;

    match resp {
        Ok(resp) => {
            println!("is success: {}", resp.status().is_success());
            let status = resp.status();
            let bytes = resp.bytes().await.unwrap();
            let content = String::from_utf8_lossy(&bytes);
            println!("{}", status);
            println!("{}", content);
        }
        Err(error) => {
            println!("{:#?}", error);
        }
    }
}


```

## 五、其他

```rust
# use std::error::Error;
#
# fn main() -> Result<(), Box<dyn Error>> {
your;
example?;
code;
#
# Ok(())
# }
```

