// use ::{entities::ListCnameResult, util::Authorization, OssClient, OssData, OssResult};
use crate::oss::{self, entities::ListCnameResult, Client};

#[allow(non_snake_case)]
impl<'a> Client<'a> {
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
    pub fn DeleteCname() {
        todo!()
    }
}
