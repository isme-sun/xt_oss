use crate::oss::{
    self,
    entities::cname::{BucketCnameConfiguration, Certificate, CnameToken, ListCnameResult},
};

#[allow(unused)]
pub struct PutCnameBuilder<'a> {
    client: &'a oss::Client<'a>,
    bucket_cname_configuration: BucketCnameConfiguration,
}

#[allow(unused)]
impl<'a> PutCnameBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client) -> Self {
        PutCnameBuilder {
            client,
            bucket_cname_configuration: BucketCnameConfiguration::default(),
        }
    }

    pub fn with_domain(mut self, value: &str) -> Self {
        self.bucket_cname_configuration.cname.domain = value.to_string();
        self
    }

    pub fn with_cert_id(mut self, value: &str) -> Self {
        let certificate =
            if let Some(mut certificate) = self.bucket_cname_configuration.cname.certificate {
                certificate.cert_id = value.to_string();
                certificate
            } else {
                let mut certificate = Certificate::default();
                certificate.cert_id = value.to_string();
                certificate
            };
        self.bucket_cname_configuration.cname.certificate = Some(certificate);
        self
    }

    pub fn config(&self) -> String {
        quick_xml::se::to_string(&self.bucket_cname_configuration).unwrap()
    }

    async fn send(&self) -> oss::Result<()> {
        let query = "cname&comp=token";
        let url = format!("{}/?{}", self.client.options.base_url(), query);

        let config = self.config();
        let data = oss::Bytes::from(config);
        let resp = self
            .client
            .request
            .task()
            .url(&url)
            .method(oss::Method::POST)
            .body(data)
            .resourse(query)
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

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 调用CreateCnameToken接口创建域名所有权验证所需的CnameToken
    pub async fn CreateCnameToken(&self, cname: &'a str) -> oss::Result<CnameToken> {
        let query = "cname&comp=token";
        let url = format!("{}/?{}", &self.options.base_url(), query);

        let mut config = BucketCnameConfiguration::default();
        config.cname.domain = cname.to_string();
        let data = oss::Bytes::from(quick_xml::se::to_string(&config).unwrap());

        let resp = self
            .request
            .task()
            .url(&url)
            .method(oss::Method::POST)
            .body(data)
            .send()
            .await?;

        let content = String::from_utf8_lossy(&resp.data);
        let cname_token = quick_xml::de::from_str::<CnameToken>(&content).unwrap();
        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: cname_token,
        };
        Ok(result)
    }

    /// 调用GetCnameToken接口获取已创建的CnameToken
    pub async fn GetCnameToken(&self, cname: &'a str) -> oss::Result<CnameToken> {
        let query = format!("comp=token&cname={}", cname);
        let url = format!("{}/?{}", self.options.base_url(), &query);

        let resp = self
            .request
            .task()
            .url(&url)
            .resourse(&query)
            .send()
            .await?;

        let content = String::from_utf8_lossy(&resp.data);
        let cname_token = quick_xml::de::from_str::<CnameToken>(&content).unwrap();
        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: cname_token,
        };
        Ok(result)
    }

    /// 调用PutCname接口为某个存储空间（Bucket）绑定自定义域名
    pub fn PutCname(&self) -> PutCnameBuilder {
        PutCnameBuilder::new(self)
    }

    /// 调用ListCname接口用于查询某个存储空间（Bucket）下绑定的所有的自定义域名（Cname）列表
    pub async fn ListCname(&self) -> oss::Result<ListCnameResult> {
        let res = "cname";
        let url = {
            let base_url = self.options.base_url();
            format!("{base_url}?{res}")
        };

        let resp = self.request.task().url(&url).resourse(res).send().await?;

        let content = String::from_utf8_lossy(&resp.data);

        println!("{}", content);

        let cnames: ListCnameResult = quick_xml::de::from_str(&content).unwrap();
        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: cnames.clone(),
        };
        Ok(result)
    }

    /// 调用DeleteCname接口删除某个存储空间（Bucket）已绑定的Cname
    pub async fn DeleteCname(&self, cname: &'a str) -> oss::Result<()> {
        let res = "cname";
        let url = {
            let base_url = &self.options.base_url();
            format!("{base_url}?{res}")
        };

        let mut config = BucketCnameConfiguration::default();
        config.cname.domain = cname.to_string();
        let data = oss::Bytes::from(quick_xml::se::to_string(&config).unwrap());

        let resp = self
            .request
            .task()
            .url(&url)
            .body(data)
            .resourse(res)
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
