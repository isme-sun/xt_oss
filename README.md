# XT - Aliyun OSS SDK

> 参考官方其他SDK实现的阿里云OSS SDK

概要说明 ...

### 关于Service/Region操作

| Item         | Price     | # In stock |
|--------------|-----------|------------|
| Juicy Apples | 1.99      | *7*        |
| Bananas      | **1.89**  | 5234       |


### 关于Service/Region操作

- [X] 01 ★ `ListBuckets（GetService）`
- [X] 02 ★ `DescribeRegions`

### Bucket - 基础操作

- [X] 03 ★ `PutBucket`
- [X] 04 ★ `DeleteBucket`
- [X] 05 ★ `ListObjects`
- [X] 06 ★ `ListObjectsV2`
- [X] 07 ★ `GetBucketInfo`
- [X] 08 ★ `GetBucketLocation`
- [X] 09 ★ `GetBucketStat`

### Bucket 合规保留策略（WORM）

- [X] 10 ★ `InitiateBucketWorm`
- [X] 11 ★ `AbortBucketWorm`
- [X] 12 ★ `CompleteBucketWorm`
- [X] 13 ★ `ExtendBucketWorm`
- [X] 14 ★ `GetBucketWorm`

### Bucket 权限控制（ACL）

- [X] 15 ★ `PutBucketAcl`
- [X] 16 ★ `GetBucketAcl`

### Bucket 生命周期（Lifecycle）

- [X] 17 ★ `PutBucketLifecycle`
- [X] 18 ★ `GetBucketLifecycle`
- [X] 19 ★ `DeleteBucketLifecycle`

### Bucket 传输加速（TransferAcceleration）

- [X] 20 ★ `PutBucketTransferAcceleration`
- [X] 21 ★ `GetBucketTransferAcceleration`

### Bucket 版本控制（Versioning）

- [X] 22 ★ `PutBucketVersioning`
- [X] 23 ★ `GetBucketVersioning`
- [ ] 24 ★ `ListObjectVersions（GetBucketVersions）`

### Bucket 数据复制（Replication）

- [ ] `PutBucketReplication`
- [ ] `PutBucketRTC`
- [ ] `GetBucketReplication`
- [ ] `GetBucketReplicationLocation`
- [ ] `GetBucketReplicationProgress`
- [ ] `DeleteBucketReplication`

### Bucket 授权策略（Policy）

- [ ] 25 ★ `PutBucketPolicy`
- [ ] 26 ★ `GetBucketPolicy`
- [ ] 27 ★ `DeleteBucketPolicy`

### Bucket 清单（Inventory）

- [ ] `PutBucketInventory`
- [ ] `GetBucketInventory`
- [ ] `ListBucketInventory`
- [ ] `DeleteBucketInventory`

### Bucket 日志管理（Logging）

- [X] 28 ★ `PutBucketLogging`
- [X] 29 ★ `GetBucketLogging`
- [X] 30 ★ `DeleteBucketLogging`

### Bucket 静态网站（Website）

- [X] 31 ★ `PutBucketWebsite`
- [X] 32 ★ `GetBucketWebsite`
- [X] 33 ★ `DeleteBucketWebsite`

### Bucket 防盗链（Referer）

- [X] 34 ★ `PutBucketReferer`
- [X] 35 ★ `GetBucketReferer`

### Bucket 标签（Tags）

- [X] 36 ★ `PutBucketTags`
- [X] 37 ★ `GetBucketTags`
- [X] 38 ★ `DeleteBucketTags`

### Bucket 加密（Encryption）

- [X] 39 ★ `PutBucketEncryption`
- [X] 40 ★ `GetBucketEncryption`
- [X] 41 ★ `DeleteBucketEncryption`

### Bucket 请求者付费（RequestPayment）

- [ ] 42 ★ `PutBucketRequestPayment`
- [ ] 43 ★ `GetBucketRequestPayment`

### Bucket 跨域资源共享（CORS）

- [X] 43 ★ `PutBucketCors`
- [X] 44 ★ `GetBucketCors`
- [X] 45 ★ `DeleteBucketCors`
- [ ] `Options`

### Bucket 访问跟踪（AccessMonitor）

- [ ] `PutBucketAccessMonitor`
- [ ] `GetBucketAccessMonitor`

### Bucket 数据索引（Data Indexing）

- [ ] `OpenMetaQuery`
- [ ] `GetMetaQueryStatus`
- [ ] `DoMetaQuery`
- [ ] `CloseMetaQuery`

### Bucket 资源组（Resource Group）

- [ ] `PutBucketResourceGroup`
- [ ] `GetBucketResourceGroup`

### Bucket 自定义域名（CNAME）

- [X] 46 ★ `CreateCnameToken`
- [X] 47 ★ `GetCnameToken`
- [X] 48 ★ `PutCname`
- [X] 49 ★ `ListCname`
- [X] 50 ★ `DeleteCname`

### Bucket 图片样式（Style）

- [X] 51 ★ `PutStyle`
- [X] 52 ★ `GetStyle`
- [X] 53 ★ `ListStyle`
- [X] 54 ★ `DeleteStyle`

### Bucket  安全传输层协议（TLS）

- [ ] `PutBucketHttpsConfig`
- [ ] `GetBucketHttpsConfig`

### Bucket  存储冗余转换（RedundancyTransition）

- [ ] `CreateBucketDataRedundancyTransition`
- [ ] `GetBucketDataRedundancyTransition`
- [ ] `DeleteBucketDataRedundancyTransition`
- [ ] `ListUserDataRedundancyTransition`
- [ ] `ListBucketDataRedundancyTransition`

### Bucket  接入点（AccessPoint）

- [ ] `CreateAccessPoint`
- [ ] `GetAccessPoint`
- [ ] `DeleteAccessPoint`
- [ ] `ListAccessPoints`
- [ ] `PutAccessPointPolicy`
- [ ] `GetAccessPointPolicy`
- [ ] `DeleteAccessPointPolicy`

### Bucket  对象FC接入点（Object FC AccessPoint）

- [ ] `CreateAccessPointForObjectProcess`
- [ ] `GetAccessPointForObjectProcess`
- [ ] `DeleteAccessPointForObjectProcess`
- [ ] `ListAccessPointsForObjectProcess`
- [ ] `PutAccessPointConfigForObjectProcess`
- [ ] `GetAccessPointConfigForObjectProcess`
- [ ] `PutAccessPointPolicyForObjectProcess`
- [ ] `GetAccessPointPolicyForObjectProcess`
- [ ] `DeleteAccessPointPolicyForObjectProcess`
- [ ] `WriteGetObjectResponse`

### Object 基础操作 Stand

- [X] 55 ★ `PutObject`
- [X] 56 ★ `GetObject`
- [ ] 57 ★ `CopyObject`
- [ ] 58 ★ `AppendObject`
- [ ] 59 ★ `DeleteObject`
- [ ] 60 ★ `DeleteMultipleObjects`
- [X] 61 ★ `HeadObject`
- [X] 62 ★ `GetObjectMeta`
- [ ] `PostObject`
- [ ] `Callback`
- [ ] 63 ★ `RestoreObject`
- [ ] `SelectObject`

### Object 分片上传（MultipartUpload）

- [ ] 64 ★ `InitiateMultipartUpload`
- [ ] 65 ★ `UploadPart`
- [ ] 66 ★ `UploadPartCopy`
- [ ] 67 ★ `CompleteMultipartUpload`
- [ ] 68 ★ `AbortMultipartUpload`
- [ ] 69 ★ `ListMultipartUploads`
- [ ] 70 ★ `ListParts`

### Object 权限控制（ACL)

- [X] 71 ★ `PutObjectACL`
- [X] 72 ★ `GetObjectACL`

### Object 软链接（Symlink）

- [X] 73 ★ `PutSymlink`
- [X] 74 ★ `GetSymlink`

### Object 标签（Tagging）

- [X] 75 ★ `PutObjectTagging`
- [X] 76 ★ `GetObjectTagging`
- [X] 77 ★ `DeleteObjectTagging`

### 关于LiveChannel的操作

- [ ] 78 ★ `PutLiveChannel`
- [ ] 79 ★ `ListLiveChannel`
- [ ] 80 ★ `DeleteLiveChannel`
- [ ] 81 ★ `PutLiveChannelStatus`
- [ ] 82 ★ `GetLiveChannelInfo`
- [ ] 83 ★ `GetLiveChannelStat`
- [ ] 84 ★ `GetLiveChannelHistory`
- [ ] 85 ★ `PostVodPlaylist`
- [ ] `GetVodPlaylist`

 ```rust
 fn main() {
     println!("示例说明")
 }
  //
 ```

## 配置说明

 概要说明 ...

 ```rust
 fn main() {
     println!("示例说明")
 }
 ```

## 参数与返回数据

 概要说明 ...

 ```rust
 fn main() {
     println!("示例说明")
 }
 ```

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
