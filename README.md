# XT - Aliyun OSS SDK

> 参考官方其他SDK实现的阿里云OSS SDK

概要说明 ...

☆ 未实现 ★ 已实现

##### 关于Service/Region操作

- [X] `ListBuckets（GetService）`  
- [X] `DescribeRegions`

##### Bucket - 基础操作

- [X] `PutBucket`
- [X] `DeleteBucket`
- [X] `ListObjects`
- [X] `ListObjectsV2`
- [X] `GetBucketInfo`
- [X] `GetBucketLocation`
- [X] `GetBucketStat`

##### Bucket 合规保留策略（WORM）

- [X] `InitiateBucketWorm`
- [X] `AbortBucketWorm`
- [X] `CompleteBucketWorm`
- [X] `ExtendBucketWorm`
- [X] `GetBucketWorm`

##### Bucket 权限控制（ACL）

- [X] `PutBucketAcl`
- [X] `GetBucketAcl`

##### Bucket 生命周期（Lifecycle）

- [X] `PutBucketLifecycle`
- [X] `GetBucketLifecycle`
- [X] `DeleteBucketLifecycle`

##### Bucket 传输加速（TransferAcceleration）

- [X] `PutBucketTransferAcceleration`
- [X] `GetBucketTransferAcceleration`

##### Bucket 版本控制（Versioning）

- [X] `PutBucketVersioning`
- [X] `GetBucketVersioning`
- [ ] `ListObjectVersions（GetBucketVersions）` x

##### Bucket 数据复制（Replication）

- [ ] `PutBucketReplication`
- [ ] `PutBucketRTC`
- [ ] `GetBucketReplication`
- [ ] `GetBucketReplicationLocation`
- [ ] `GetBucketReplicationProgress`
- [ ] `DeleteBucketReplication`

##### Bucket 授权策略（Policy）

- [ ] `PutBucketPolicy` x
- [ ] `GetBucketPolicy` x
- [ ] `DeleteBucketPolicy`x

##### Bucket 清单（Inventory）

- [ ] `PutBucketInventory`
- [ ] `GetBucketInventory`
- [ ] `ListBucketInventory`
- [ ] `DeleteBucketInventory`

##### Bucket 日志管理（Logging）

- [X] `PutBucketLogging`
- [X] `GetBucketLogging`
- [X] `DeleteBucketLogging`

##### Bucket 静态网站（Website）

- [X] `PutBucketWebsite`
- [X] `GetBucketWebsite`
- [X] `DeleteBucketWebsite`

##### Bucket 防盗链（Referer）

- [X] `PutBucketReferer`
- [X] `GetBucketReferer`

##### Bucket 标签（Tags）

- [X] `PutBucketTags`
- [X] `GetBucketTags`
- [X] `DeleteBucketTags`

##### Bucket 加密（Encryption）

- [X] `PutBucketEncryption`
- [X] `GetBucketEncryption`
- [X] `DeleteBucketEncryption`

##### Bucket 请求者付费（RequestPayment）

- [ ] `PutBucketRequestPayment` x
- [ ] `GetBucketRequestPayment` x

##### Bucket 跨域资源共享（CORS）

- [X] `PutBucketCors`
- [X] `GetBucketCors`
- [X] `DeleteBucketCors`
- [ ] `Options` x

##### Bucket 访问跟踪（AccessMonitor）

- [ ] `PutBucketAccessMonitor`
- [ ] `GetBucketAccessMonitor`

##### Bucket 数据索引（Data Indexing）

- [ ] `OpenMetaQuery`
- [ ] `GetMetaQueryStatus`
- [ ] `DoMetaQuery`
- [ ] `CloseMetaQuery`

##### Bucket 资源组（Resource Group）

- [ ] `PutBucketResourceGroup`
- [ ] `GetBucketResourceGroup`

##### Bucket 自定义域名（CNAME）

- [X] `CreateCnameToken`
- [X] `GetCnameToken`
- [X] `PutCname`
- [X] `ListCname`
- [X] `DeleteCname`

##### Bucket 图片样式（Style）

- [X] `PutStyle`
- [X] `GetStyle`
- [X] `ListStyle`
- [X] `DeleteStyle`

##### Bucket  安全传输层协议（TLS）

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
- [ ] `WriteGetObjectResponse`

##### Object 基础操作 Stand

- [X] `PutObject`
- [X] `GetObject`
- [ ] `CopyObject` x
- [ ] `AppendObject` x
- [ ] `DeleteObject` x
- [ ] `DeleteMultipleObjects` x
- [X] `HeadObject`
- [X] `GetObjectMeta`
- [ ] `PostObject`
- [ ] `Callback`
- [ ] `RestoreObject` x
- [ ] `SelectObject`

##### Object 分片上传（MultipartUpload）

- [ ] `InitiateMultipartUpload` x
- [ ] `UploadPart` x
- [ ] `UploadPartCopy` x
- [ ] `CompleteMultipartUpload` x
- [ ] `AbortMultipartUpload` x
- [ ] `ListMultipartUploads` x
- [ ] `ListParts` x

##### Object 权限控制（ACL)

- [X] `PutObjectACL`
- [X] `GetObjectACL`

##### Object 软链接（Symlink）

- [X] `PutSymlink`
- [X] `GetSymlink`

##### Object 标签（Tagging）

- [X] `PutObjectTagging`
- [X] `GetObjectTagging`
- [X] `DeleteObjectTagging`

##### 关于LiveChannel的操作

- [ ] `PutLiveChannel` x
- [ ] `ListLiveChannel` x
- [ ] `DeleteLiveChannel` x
- [ ] `PutLiveChannelStatus` x
- [ ] `GetLiveChannelInfo` x
- [ ] `GetLiveChannelStat` x
- [ ] `GetLiveChannelHistory` x
- [ ] `PostVodPlaylist` x
- [ ] `GetVodPlaylist` x

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
