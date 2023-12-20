use crate::oss::{self, entities::RefererConfiguration, Client};

use super::builders::PutBucketRefererBuilder;
#[allow(non_snake_case)]
impl<'a> Client<'a> {
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

        let config_inner: oss::entities::inner::RefererConfiguration =
            quick_xml::de::from_str(&content).unwrap();

        let mut referer_list: Vec<String> = Vec::new();
        let mut referer_blacklist: Vec<String> = Vec::new();

        if let Some(inner_referer_list) = config_inner.referer_list {
            if let Some(referer) = inner_referer_list.referer {
                for url in referer {
                    referer_list.push(url);
                }
            }
        }

        if let Some(inner_referer_blacklist) = config_inner.referer_blacklist {
            if let Some(referer) = inner_referer_blacklist.referer {
                for url in referer {
                    referer_blacklist.push(url);
                }
            }
        }

        let config = RefererConfiguration {
            allow_empty_referer: config_inner.allow_empty_referer,
            allow_truncate_query_string: config_inner.allow_truncate_query_string,
            truncate_path: config_inner.truncate_path,
            referer_list,
            referer_blacklist,
        };

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: config,
        };
        Ok(result)
    }

    /// 调用PutBucketReferer接口设置存储空间（Bucket）级别的Referer访问白名单以及黑名单
    pub fn PutBucketReferer(&self) -> PutBucketRefererBuilder {
        PutBucketRefererBuilder::new(&self)
    }
}
