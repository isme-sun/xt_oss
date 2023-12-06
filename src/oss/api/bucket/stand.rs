#[allow(unused)]
use crate::oss::{
    self,
    arguments::{CreateBucketConfiguration, CreateBucketParams, ListObject2Query},
    entities::{BucketInfo, BucketStat, ListBucketResult},
    Client, Data, Method, Result,
};

#[allow(non_snake_case)]
impl<'a> Client<'a> {
    /// 调用PutBucket接口创建存储空间（Bucket）。
    pub async fn PutBucket(&self, params: CreateBucketParams<'_>) -> Result<oss::Bytes> {
        let bucket = params.name;
        let url = {
            format!(
                "{}://{}.{}",
                self.options.schema(),
                bucket,
                self.options.host()
            )
        };

        let headers = params.headers();
        let data = params.config();

        let resp = self
            .request
            .task()
            .url(&url)
            .method(Method::PUT)
            .headers(headers)
            .body(data)
            .send()
            .await?;

        let result = Data {
            status: resp.status,
            headers: resp.headers,
            data: resp.data,
        };
        Ok(result)
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

    // ListObjectsV2（GetBucketV2）接口用于列举存储空间（Bucket）中所有文件（Object）的信息。
    pub async fn ListObjectsV2(&self, qurey: ListObject2Query<'a>) -> Result<ListBucketResult> {
        let url = {
            let base_url = self.options.base_url();
            format!("{}?{}", base_url, qurey)
        };

        let resp = self.request.task().url(&url).send().await.unwrap();

        let content = String::from_utf8_lossy(&resp.data);
        let buckets: ListBucketResult = serde_xml_rs::from_str(&content).unwrap();
        let result = Data {
            status: resp.status,
            headers: resp.headers,
            data: buckets,
        };
        Ok(result)
    }

    // 调用GetBucketInfo接口查看存储空间（Bucket）的相关信息。
    pub async fn GetBucketInfo(&self) -> Result<BucketInfo> {
        let res = "bucketInfo";
        let url = {
            let base_url = self.options.base_url();
            format!("{base_url}?{res}")
        };

        let resp = self
            .request
            .task()
            .url(&url)
            .resourse(&res)
            .send()
            .await
            .unwrap();

        let content = String::from_utf8_lossy(&resp.data);
        let bucket_info: BucketInfo = serde_xml_rs::from_str(&content).unwrap();
        let result = Data {
            status: resp.status,
            headers: resp.headers,
            data: bucket_info,
        };
        Ok(result)
    }

    /// GetBucketLocation接口用于查看存储空间（Bucket）的位置信息。
    /// 只有Bucket的拥有者才能查看Bucket的位置信息。
    pub fn GetBucketLocation(&self) {
        todo!()
    }

    pub async fn GetBucketStat(&self) -> Result<BucketStat> {
        let res = "stat";
        let url = {
            let base_url = self.options.base_url();
            format!("{base_url}?{res}")
        };
        let resp = self
            .request
            .task()
            .url(&url)
            .resourse(&res)
            .send()
            .await
            .unwrap();

        let content = String::from_utf8_lossy(&resp.data);
        let bucket_stat: BucketStat = serde_xml_rs::from_str(&content).unwrap();
        let result = Data {
            status: resp.status,
            headers: resp.headers,
            data: bucket_stat,
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
}
