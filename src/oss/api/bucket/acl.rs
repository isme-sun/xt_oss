use crate::oss::{Client, self, entities::AccessControlPolicy};

use super::builders::PutBucketAclBuilder;
#[allow(non_snake_case)]
impl<'a> Client<'a> {

	/// PutBucketAcl接口用于设置或修改存储空间（Bucket）的访问权限（ACL）。
	pub fn PutBucketAcl(&self) -> PutBucketAclBuilder {
        PutBucketAclBuilder::new(&self)
	}

	/// GetBucketAcl接口用于获取某个存储空间（Bucket）的访问权限（ACL）。只有Bucket的拥有者才能获取Bucket的访问权限。
	pub async fn GetBucketAcl(&self) -> oss::Result<AccessControlPolicy> {
        let res = "acl";
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
        let cnames: AccessControlPolicy = serde_xml_rs::from_str(&content).unwrap();
        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: cnames,
        };
        Ok(result)
	}

}
