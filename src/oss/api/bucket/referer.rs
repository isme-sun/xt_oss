use crate::oss;

use self::builders::{GetBucketRefererBuilder, PutBucketRefererBuilder};

pub mod builders {
    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
        entities::referer::RefererConfiguration,
        http,
    };

    #[derive(Debug)]
    pub struct PutBucketRefererBuilder<'a> {
        client: &'a oss::Client<'a>,
        // config: RefererConfiguration,
        config: RefererConfiguration,
    }

    impl<'a> PutBucketRefererBuilder<'a> {
        pub fn new(cilent: &'a oss::Client) -> Self {
            Self {
                client: cilent,
                config: RefererConfiguration::default(),
            }
        }

        pub fn allow_empty_referer(mut self, value: bool) -> Self {
            self.config.allow_empty_referer = value;
            self
        }

        pub fn allow_truncate_query_string(mut self, value: bool) -> Self {
            self.config.allow_truncate_query_string = value;
            self
        }

        pub fn truncate_path(mut self, value: bool) -> Self {
            self.config.truncate_path = value;
            self
        }

        pub fn referer_list(mut self, value: Vec<String>) -> Self {
            self.config.referer_list = value;
            self
        }

        pub fn referer_blacklist(mut self, value: Vec<String>) -> Self {
            self.config.referer_blacklist = value;
            self
        }

        pub fn get_referer_list(self) -> Vec<String> {
            self.config.referer_list
        }

        pub fn get_referer_blacklist(self) -> Vec<String> {
            self.config.referer_blacklist
        }

        pub fn push_to_referer_list(mut self, value: &'a str) -> Self {
            let mut index: Option<usize> = None;
            for (i, item) in self.config.referer_list.iter().enumerate() {
                if value == *item {
                    index = Some(i);
                    break;
                }
            }
            if index.is_none() {
                self.config.referer_list.push(value.to_string());
            }
            self
        }

        pub fn remove_from_referer_list(mut self, value: &'a str) -> Self {
            let mut index: Option<usize> = None;
            for (i, item) in self.config.referer_list.iter().enumerate() {
                if value == *item {
                    index = Some(i);
                    break;
                }
            }
            if let Some(index) = index {
                self.config.referer_list.remove(index);
            }
            self
        }

        pub fn push_to_referer_blacklist(mut self, value: &'static str) -> Self {
            let mut index: Option<usize> = None;
            for (i, item) in self.config.referer_blacklist.iter().enumerate() {
                if value == *item {
                    index = Some(i);
                    break;
                }
            }
            if index.is_none() {
                self.config.referer_blacklist.push(value.into());
            }
            self
        }

        pub fn remove_from_referer_backlist(mut self, value: String) -> Self {
            let mut index: Option<usize> = None;
            for (i, item) in self.config.referer_blacklist.iter().enumerate() {
                if value == *item {
                    index = Some(i);
                    break;
                }
            }
            if let Some(index) = index {
                self.config.referer_blacklist.remove(index);
            }
            self
        }

        fn config(&self) -> String {
            let config = self.config.to_inner();
            quick_xml::se::to_string(&config).unwrap()
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}", self.client.options.bucket, "referer");
            let url = { format!("{}?{}", self.client.options.base_url(), "referer") };
            let config = self.config();
            let data = oss::Bytes::from(config);

            let resp = self
                .client
                .request
                .task()
                .with_method(http::Method::PUT)
                .with_url(&url)
                .with_resource(&res)
                .with_body(data)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).as_empty().await)
        }
    }

    pub struct GetBucketRefererBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> GetBucketRefererBuilder<'a> {
        pub fn new(cilent: &'a oss::Client) -> Self {
            Self { client: cilent }
        }

        pub async fn execute(&self) -> api::ApiResult<RefererConfiguration> {
            let res = format!("/{}/?{}", self.client.options.bucket, "referer");
            let url = format!("{}?{}", self.client.options.base_url(), "referer");

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).as_type().await)
        }
    }
}

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 调用PutBucketReferer接口设置存储空间（Bucket）级别的Referer访问白名单以及黑名单
    pub fn PutBucketReferer(&self) -> PutBucketRefererBuilder {
        PutBucketRefererBuilder::new(self)
    }

    /// GetBucketReferer接口用于查看存储空间（Bucket）的防盗链（Referer）相关配置。
    pub fn GetBucketReferer(&self) -> GetBucketRefererBuilder {
        GetBucketRefererBuilder::new(self)
    }
}
