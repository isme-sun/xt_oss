use crate::{
    arguments::ListObject2Query,
    entities::{BucketInfo, BucketStat, ListBucketResult, ListCnameResult},
    util::Authorization,
    OssClient, OssData, OssResult,
};
#[allow(non_snake_case)]
impl OssClient {
    /// 调用PutBucket接口创建存储空间（Bucket）。
    pub fn PutBucket(&self) {
        todo!()
    }

    /// 调用DeleteBucket删除某个存储空间（Bucket）。
    /// - 只有Bucket的拥有者才有权限删除该Bucket。
    /// - 为了防止误删除的发生，OSS不允许删除一个非空的Bucket。
    pub fn DeleteBucket(&self) {
        todo!()
    }

    /// GetBucket (ListObjects)接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
    pub fn GetBucket(&self) {
        todo!()
    }

    /// ListObjectsV2（GetBucketV2）接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
    #[allow(unused)]
    pub async fn ListObjectsV2(&self, qurey: ListObject2Query) -> OssResult<ListBucketResult> {
        let url = {
            let base_url = self.options.base_url();
            let query_str = serde_qs::to_string(&qurey).unwrap();
            format!("{base_url}?{query_str}")
        };
        let auth = Authorization {
            bucket: Some(self.options.bucket.to_owned()),
            ..Authorization::default()
        };

        let (_status, headers, data) = self.request(url, auth).await?;
				let content = String::from_utf8_lossy(&data);
        let bucket: ListBucketResult = serde_xml_rs::from_str(&content).unwrap();
        let result = OssData {
            headers,
            data: bucket,
        };
        Ok(result)
    }

    /// 调用GetBucketInfo接口查看存储空间（Bucket）的相关信息。
    pub async fn GetBucketInfo(&self) -> OssResult<BucketInfo> {
        let url = {
            let base_url = self.options.base_url();
            let query_str = "bucketInfo".to_string();
            format!("{base_url}?{query_str}")
        };
        let auth = Authorization {
            bucket: Some(self.options.bucket.to_owned()),
            sub_res: Some("bucketInfo".to_string()),
            ..Authorization::default()
        };

        let (_status, headers, data) = self.request(url, auth).await?;

        let content = String::from_utf8_lossy(&data);
        let bucket: BucketInfo = serde_xml_rs::from_str(&content).unwrap();
        let result = OssData {
            headers,
            data: bucket,
        };
        Ok(result)
    }

    /// GetBucketLocation接口用于查看存储空间（Bucket）的位置信息。
    /// 只有Bucket的拥有者才能查看Bucket的位置信息。
    pub fn GetBucketLocation(&self) {
        todo!()
    }

    pub async fn GetBucketStat(&self) -> OssResult<BucketStat> {
        let url = {
            let base_url = self.options.base_url();
            let query_str = "stat";
            format!("{base_url}?{query_str}")
        };
        let auth = Authorization {
            bucket: Some(self.options.bucket.to_owned()),
            sub_res: Some("stat".to_string()),
            ..Authorization::default()
        };

        let (_status, headers, data) = self.request(url, auth).await?;

				let content = String::from_utf8_lossy(&data);
        let bucket: BucketStat = serde_xml_rs::from_str(&content).unwrap();
        let result = OssData {
            headers,
            data: bucket,
        };
        Ok(result)
    }

    /// 调用InitiateBucketWorm接口新建一条合规保留策略。
    #[allow(non_snake_case)]
    pub fn InitiateBucketWorm() {
        todo!()
    }

    /// AbortBucketWorm用于删除未锁定的合规保留策略。
    #[allow(non_snake_case)]
    pub fn AbortBucketWorm() {
        todo!()
    }

    /// CompleteBucketWorm用于锁定合规保留策略。
    #[allow(non_snake_case)]
    pub fn CompleteBucketWorm() {
        todo!()
    }

    /// ExtendBucketWorm用于延长已锁定的合规保留策略对应Bucket中Object的保留天数。
    #[allow(non_snake_case)]
    pub fn ExtendBucketWorm() {
        todo!()
    }

    /// GetBucketWorm用于获取指定存储空间（Bucket）的合规保留策略信息。
    #[allow(non_snake_case)]
    pub fn GetBucketWorm() {
        todo!()
    }

    /// PutBucketAcl接口用于设置或修改存储空间（Bucket）的访问权限（ACL）
    #[allow(non_snake_case)]
    pub fn PutBucketAcl() {
        todo!()
    }

    /// GetBucketAcl接口用于获取某个存储空间（Bucket）的访问权限（ACL）。只有Bucket的拥有者才能获取Bucket的访问权限。
    #[allow(non_snake_case)]
    pub fn GetBucketAcl() {
        todo!()
    }
    /// 调用PutBucketLifecycle接口为存储空间（Bucket）设置生命周期规则。生命周期规则开启后，OSS将按照规则中指
    /// 定的过期时间，自动转换与规则相匹配文件（Object）的存储类型或将其删除。
    #[allow(non_snake_case)]
    pub fn PutBucketLifecycle() {
        todo!()
    }

    /// 调用GetBucketLifecycle接口查看存储空间（Bucket）的生命周期规则（Lifecycle）。
    #[allow(non_snake_case)]
    pub fn GetBucketLifecycle() {
        todo!()
    }

    /// DeleteBucketLifecycle接口用于删除指定存储空间（Bucket）的生命周期规则。使用DeleteBucketLifecycle
    /// 接口删除指定Bucket所有的生命周期规则后，该Bucket中的文件（Object）不会被自动删除。只有Bucket的拥有者
    /// 才能删除该Bucket的生命周期规则。
    #[allow(non_snake_case)]
    pub fn DeleteBucketLifecycle() {
        todo!()
    }

    /// # PutBucketTransferAcceleration
    /// PutBucketTransferAcceleration接口用于为存储空间（Bucket）配置传输加速。开启传输加速后，可提升全球各
    /// 地用户对OSS的访问速度，适用于远距离数据传输、GB或TB级大文件上传和下载的场景。
    #[allow(non_snake_case)]
    pub fn PutBucketTransferAcceleration() {
        todo!()
    }

    /// GetBucketTransferAcceleration接口用于获取目标存储空间（Bucket）的传输加速配置。
    #[allow(non_snake_case)]
    pub fn GetBucketTransferAcceleration() {
        todo!()
    }

    /// 调用CreateCnameToken接口创建域名所有权验证所需的CnameToken
    pub fn CreateCnameToken() {
        todo!()
    }

    /// 调用GetCnameToken接口获取已创建的CnameToken
    pub fn GetCnameToken() {
        todo!()
    }

    /// 调用PutCname接口为某个存储空间（Bucket）绑定自定义域名
    pub fn PutCname() {
        todo!()
    }

    /// 调用ListCname接口用于查询某个存储空间（Bucket）下绑定的所有的自定义域名（Cname）列表
    pub async fn ListCname(&self) -> OssResult<ListCnameResult> {
        let url = {
            let base_url = self.options.base_url();
            let query_str = "cname";
            format!("{base_url}?{query_str}")
        };
        let auth = Authorization {
            bucket: Some(self.options.bucket.to_owned()),
            sub_res: Some("cname".to_string()),
            ..Authorization::default()
        };

        let (_status, headers, data) = self.request(url, auth).await?;

				let content = String::from_utf8_lossy(&data);
        let bucket: ListCnameResult = serde_xml_rs::from_str(&content).unwrap();
        let result = OssData {
            headers,
            data: bucket,
        };
        Ok(result)
    }

    /// 调用DeleteCname接口删除某个存储空间（Bucket）已绑定的Cname
    pub fn DeleteCname() {
        todo!()
    }
}
