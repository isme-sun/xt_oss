# XT - Aliyun OSS SDK

> 参考官方其他SDK实现的阿里云OSS SDK

概要说明 ...

### 关于Service/Region操作

- [X] 01 ★ `ListBuckets（GetService）`
- [X] 02 ★ `DescribeRegions` 

### Bucket - 基础操作

- [X] 03 ★ `PutBucket` 
- [X] 04 ★ `DeleteBucket`
- [ ] 05 ★ `ListObjects`
- [X] 06 ★ `ListObjectsV2`
- [X] 07 ★ `GetBucketInfo`
- [X] 08 ★ `GetBucketLocation`
- [X] 09 ★ `GetBucketStat`

### Bucket 合规保留策略（WORM）

- [ ] 10 ★ `InitiateBucketWorm`
- [ ] 11 ★ `AbortBucketWorm`
- [ ] 12 ★ `CompleteBucketWorm`
- [ ] 13 ★ `ExtendBucketWorm`
- [ ] 14 ★ `GetBucketWorm`

### Bucket 权限控制（ACL）

- [ ] 15 ★ `PutBucketAcl`
- [ ] 16 ★ `GetBucketAcl`

### Bucket 生命周期（Lifecycle）

- [ ] 17 ★ `PutBucketLifecycle`
- [ ] 18 ★ `GetBucketLifecycle`
- [ ] 19 ★ `DeleteBucketLifecycle`

### Bucket 传输加速（TransferAcceleration）

- [ ] 20 ★ `PutBucketTransferAcceleration`
- [ ] 21 ★ `GetBucketTransferAcceleration`

### Bucket 版本控制（Versioning）

- [ ] 22 ★ `PutBucketVersioning`
- [ ] 23 ★ `GetBucketVersioning`
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

- [ ] 28 ★ `PutBucketLogging`
- [ ] 29 ★ `GetBucketLogging`
- [ ] 30 ★ `DeleteBucketLogging`

### Bucket 静态网站（Website）

- [ ] 31 ★ `PutBucketWebsite`
- [ ] 32 ★ `GetBucketWebsite`
- [ ] 33 ★ `DeleteBucketWebsite`

### Bucket 防盗链（Referer）

- [ ] 34 ★ `PutBucketReferer`
- [ ] 35 ★ `GetBucketReferer`

### Bucket 标签（Tags）

- [ ] 36 ★ `PutBucketTags`
- [ ] 37 ★ `GetBucketTags`
- [ ] 38 ★ `DeleteBucketTags`

### Bucket 加密（Encryption）

- [ ] 39 ★ `PutBucketEncryption`
- [ ] 40 ★ `GetBucketEncryption`
- [ ] 41 ★ `DeleteBucketEncryption`

### Bucket 请求者付费（RequestPayment）

- [ ] 42 ★ `PutBucketRequestPayment`
- [ ] 43 ★ `GetBucketRequestPayment`

### Bucket 跨域资源共享（CORS）

- [ ] 43 ★ `PutBucketCors`
- [ ] 44 ★ `GetBucketCors`
- [ ] 45 ★ `DeleteBucketCors`
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

- [ ] 46 ★ `CreateCnameToken`
- [ ] 47 ★ `GetCnameToken`
- [ ] 48 ★ `PutCname`
- [X] 49 ★ `ListCname`
- [ ] 50 ★ `DeleteCname`

### Bucket 图片样式（Style）

- [ ] 51 ★ `PutStyle`
- [ ] 52 ★ `GetStyle`
- [ ] 53 ★ `ListStyle`
- [ ] 54 ★ `DeleteStyle`

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

- [ ] 55 ★ `PutObject`
- [ ] 56 ★ `GetObject`
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

- [ ] 71 ★ `PutObjectACL`
- [ ] 72 ★ `GetObjectACL`

### Object 软链接（Symlink）

- [ ] 73 ★ `PutSymlink`
- [ ] 74 ★ `GetSymlink`

### Object 标签（Tagging）

- [ ] 75 ★ `PutObjectTagging`
- [ ] 76 ★ `GetObjectTagging`
- [ ] 77 ★ `DeleteObjectTagging`

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
