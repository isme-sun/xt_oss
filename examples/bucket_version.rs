use xt_oss::{
    oss::{self, entities::version::VersioningStatus},
    utils,
};

// * ex: * 调用PutBucketVersioning设置指定存储空间（Bucket）的版本控制状态
#[allow(unused)]
async fn ex_put_bucket_versioning() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let result = client
        .PutBucketVersioning()
        // .status(VersioningStatus::Suspended)
        .status(VersioningStatus::Enabled)
        .send()
        .await;
    match result {
        Ok(result) => {
            println!("status: {}", result.status);
            println!("headers: {:#?}", result.headers);
            println!("data: {:#?}", result.data);
        }
        Err(message) => {
            println!("{}", message)
        }
    }
}

// * ex: * GetBucketVersioning接口用于获取指定Bucket的版本控制状态
#[allow(unused)]
async fn ex_get_bucket_versioning() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let result = client.GetBucketVersioning().await;
    match result {
        Ok(result) => {
            println!("status: {}", result.status);
            println!("headers: {:#?}", result.headers);
            println!("data: {:#?}", result.data);
            println!("json: {}", serde_json::to_string(&result.data).unwrap());
        }
        Err(message) => {
            println!("{}", message)
        }
    }
}

// * ex: * ListObjectVersions（GetBucketVersions）接口用于列出Bucket中包括删除标记（Delete Marker）在内的所有Object的版本信息。
#[allow(unused)]
async fn ex_list_object_versions() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    // let result = client.ListObjectVersions().await;
    // match result {
    //     Ok(result) => {
    //         println!("result.status: {}", result.status);
    //         println!("result.headers: {:#?}", result.headers);
    //         println!("result.data: {:#?}", result.data);
    //     }
    //     Err(message) => {
    //         println!("{}", message)
    //     }
    // }
}

#[tokio::main]
async fn main() {
    // * ex: * 调用PutBucketVersioning设置指定存储空间（Bucket）的版本控制状态
    // ex_put_bucket_versioning().await;
    // * ex: * GetBucketVersioning接口用于获取指定Bucket的版本控制状态
    ex_get_bucket_versioning().await;
    // * ex: * ListObjectVersions（GetBucketVersions）接口用于列出Bucket中包括删除标记（Delete Marker）在内的所有Object的版本信息。
    // ex_list_object_versions().await;
}
