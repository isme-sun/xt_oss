use crate::oss::{
    self,
    entities::referer::{inner, RefererConfiguration},
};

use self::builder::PutBucketRefererBuilder;

pub mod builder {
    use crate::oss::{self, entities::referer::RefererConfiguration};

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

        pub async fn send(&self) -> oss::Result<()> {
            let res = "referer";
            let url = { format!("{}?{}", self.client.options.base_url(), res) };
            let config = self.config();
            let data = oss::Bytes::from(config);
            let resp = self
                .client
                .request
                .task()
                .method(oss::Method::PUT)
                .url(&url)
                .resourse(res)
                .body(data)
                .send()
                .await?;

            let result = oss::Data {
                status: resp.status,
                headers: resp.headers,
                data: (),
            };
            Ok(result)
        }
    }
}

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// GetBucketReferer接口用于查看存储空间（Bucket）的防盗链（Referer）相关配置。
    pub async fn GetBucketReferer(&self) -> oss::Result<RefererConfiguration> {
        let res = "referer";
        let url = {
            let base_url = self.options.base_url();
            format!("{base_url}?{res}")
        };

        let resp = self
            .request
            .task()
            .url(&url)
            .resourse(res)
            .send()
            .await
            .unwrap();

        let content = String::from_utf8_lossy(&resp.data);

        let config_inner: inner::RefererConfiguration = quick_xml::de::from_str(&content).unwrap();

        let config = RefererConfiguration::from_inner(config_inner);

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: config,
        };
        Ok(result)
    }

    /// 调用PutBucketReferer接口设置存储空间（Bucket）级别的Referer访问白名单以及黑名单
    pub fn PutBucketReferer(&self) -> PutBucketRefererBuilder {
        PutBucketRefererBuilder::new(self)
    }
}
