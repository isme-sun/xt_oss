// use crate::{
//     arguments::{ListBucketsQuery, OSSQuery},
//     entities::ListAllMyBucketsResult,
//     util::Authorization,
//     OssClient, OssData, OssResult,
// };

// #[allow(non_snake_case)]
// /// 关于Region操作
// impl OssClient {
//     /// 调用ListBuckets（GetService）接口列举请求者拥有的所有存储空间（Bucket）。
//     /// 您还可以通过设置prefix、marker或者max-keys参数列举满足指定条件的存储空间。
//     pub async fn ListBuckets(&self, query: ListBucketsQuery) -> OssResult<ListAllMyBucketsResult> {
//         // 生成uri地址
//         let url = {
//             let base_url = self.options.root_url();
//             format!("{}?{}", base_url, query.to_query())
//         };
//         let auth = Authorization::default();
//         let (_status, headers, data) = self.request(url, auth).await?;
//         // println!("{}", content);

//         let content = String::from_utf8_lossy(&data);
//         let bucket: ListAllMyBucketsResult = serde_xml_rs::from_str(&content).unwrap();
//         let result = OssData {
//             headers,
//             data: bucket,
//         };
//         Ok(result)
//     }
// }
