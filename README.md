# XT - Aliyun OSS SDK

> 参考官方其他SDK实现的阿里云OSS SDK

概要说明 ...

### 关于Service/Region操作

- [X] `ListBuckets（GetService）`
- [X] `DescribeRegions`

### Bucket - 基础操作

- [ ] `PutBucket`
- [ ] `DeleteBucket`
- [ ] `ListObjects`
- [X] `ListObjectsV2`
- [X] `GetBucketInfo`
- [ ] `GetBucketLocation`
- [X] `GetBucketStat`

### Bucket 合规保留策略（WORM）

- [ ] `InitiateBucketWorm`
- [ ] `AbortBucketWorm`
- [ ] `CompleteBucketWorm`
- [ ] `ExtendBucketWorm`
- [ ] `GetBucketWorm`

### Bucket 权限控制（ACL）

- [ ] `PutBucketAcl`
- [ ] `GetBucketAcl`

### Bucket 生命周期（Lifecycle）

- [ ] `PutBucketLifecycle`
- [ ] `GetBucketLifecycle`
- [ ] `DeleteBucketLifecycle`

### Bucket 传输加速（TransferAcceleration）

- [ ] `PutBucketTransferAcceleration`
- [ ] `GetBucketTransferAcceleration`

### Bucket 版本控制（Versioning）

- [ ] `PutBucketVersioning`
- [ ] `GetBucketVersioning`
- [ ] `ListObjectVersions（GetBucketVersions）`

### Bucket 数据复制（Replication）

- [ ] `PutBucketReplication`
- [ ] `PutBucketRTC`
- [ ] `GetBucketReplication`
- [ ] `GetBucketReplicationLocation`
- [ ] `GetBucketReplicationProgress`
- [ ] `DeleteBucketReplication`

### Bucket 授权策略（Policy）

- [ ] `PutBucketPolicy`
- [ ] `GetBucketPolicy`
- [ ] `DeleteBucketPolicy`

### Bucket 清单（Inventory）

- [ ] `PutBucketInventory`
- [ ] `GetBucketInventory`
- [ ] `ListBucketInventory`
- [ ] `DeleteBucketInventory`

### Bucket 日志管理（Logging）

- [ ] `PutBucketLogging`
- [ ] `GetBucketLogging`
- [ ] `DeleteBucketLogging`

### Bucket 静态网站（Website）

- [ ] `PutBucketWebsite`
- [ ] `GetBucketWebsite`
- [ ] `DeleteBucketWebsite`

### Bucket 防盗链（Referer）

- [ ] `PutBucketReferer`
- [ ] `GetBucketReferer`

### Bucket 标签（Tags）

- [ ] `PutBucketTags`
- [ ] `GetBucketTags`
- [ ] `DeleteBucketTags`

### Bucket 加密（Encryption）

- [ ] `PutBucketEncryption`
- [ ] `GetBucketEncryption`
- [ ] `DeleteBucketEncryption`

### Bucket 请求者付费（RequestPayment）

- [ ] `PutBucketRequestPayment`
- [ ] `GetBucketRequestPayment`

### Bucket 跨域资源共享（CORS）

- [ ] `PutBucketCors`
- [ ] `GetBucketCors`
- [ ] `DeleteBucketCors`
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

- [ ] `CreateCnameToken`
- [ ] `GetCnameToken`
- [ ] `PutCname`
- [X] `ListCname`
- [ ] `DeleteCname`

### Bucket 图片样式（Style）

- [ ] `PutStyle`
- [ ] `GetStyle`
- [ ] `ListStyle`
- [ ] `DeleteStyle`

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

- [ ] `PutObject`
- [ ] `GetObject`
- [ ] `CopyObject`
- [ ] `AppendObject`
- [ ] `DeleteObject`
- [ ] `DeleteMultipleObjects`
- [X] `HeadObject`
- [X] `GetObjectMeta`
- [ ] `PostObject`
- [ ] `Callback`
- [ ] `RestoreObject`
- [ ] `SelectObject`

### Object 分片上传（MultipartUpload）

- [ ] `InitiateMultipartUpload`
- [ ] `UploadPart`
- [ ] `UploadPartCopy`
- [ ] `CompleteMultipartUpload`
- [ ] `AbortMultipartUpload`
- [ ] `ListMultipartUploads`
- [ ] `ListParts`

### Object 权限控制（ACL)

- [ ] `PutObjectACL`
- [ ] `GetObjectACL`

### Object 软链接（Symlink）

- [ ] `PutSymlink`
- [ ] `GetSymlink`

### Object 标签（Tagging）

- [ ] `PutObjectTagging`
- [ ] `GetObjectTagging`
- [ ] `DeleteObjectTagging`

### 关于LiveChannel的操作

- [ ] `PutLiveChannel`
- [ ] `ListLiveChannel`
- [ ] `DeleteLiveChannel`
- [ ] `PutLiveChannelStatus`
- [ ] `GetLiveChannelInfo`
- [ ] `GetLiveChannelStat`
- [ ] `GetLiveChannelHistory`
- [ ] `PostVodPlaylist`
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
