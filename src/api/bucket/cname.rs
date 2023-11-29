// use crate::{entities::ListCnameResult, util::Authorization, OssClient, OssData, OssResult};

// #[allow(non_snake_case)]
// impl OssClient {
//     /// 调用CreateCnameToken接口创建域名所有权验证所需的CnameToken
//     pub fn CreateCnameToken() {
//         todo!()
//     }

//     /// 调用GetCnameToken接口获取已创建的CnameToken
//     pub fn GetCnameToken() {
//         todo!()
//     }

//     /// 调用PutCname接口为某个存储空间（Bucket）绑定自定义域名
//     pub fn PutCname() {
//         todo!()
//     }

//     /// 调用ListCname接口用于查询某个存储空间（Bucket）下绑定的所有的自定义域名（Cname）列表
//     pub async fn ListCname(&self) -> OssResult<ListCnameResult> {
//         let url = {
//             let base_url = self.options.base_url();
//             let query_str = "cname";
//             format!("{base_url}?{query_str}")
//         };
//         let auth = Authorization {
//             bucket: Some(self.options.bucket.to_owned()),
//             sub_res: Some("cname".to_string()),
//             ..Authorization::default()
//         };

//         let (_status, headers, data) = self.request(url, auth).await?;

//         let content = String::from_utf8_lossy(&data);
//         let bucket: ListCnameResult = serde_xml_rs::from_str(&content).unwrap();
//         let result = OssData {
//             headers,
//             data: bucket,
//         };
//         Ok(result)
//     }

//     /// 调用DeleteCname接口删除某个存储空间（Bucket）已绑定的Cname
//     pub fn DeleteCname() {
//         todo!()
//     }
// }
