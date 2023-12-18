use xt_oss::{
    oss::{self, arguments::{OssAcl, StorageClass, DataRedundancyType}},
    utils,
};

#[allow(unused)]
async fn info_bucket() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client.GetBucketInfo().name("xtoss-t1").send().await;
    match result {
        Ok(_) => println!("{:#?}", result),
        Err(message) => {
            println!("{}", message)
        }
    }
}

#[allow(unused)]
async fn stat_bucket() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client.GetBucketStat().send().await;
    match result {
        Ok(_) => println!("{:#?}", result),
        Err(message) => {
            println!("{}", message)
        }
    }
}

#[allow(unused)]
async fn create_bucket() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client.PutBucket()
                        .name("xtoss-t1")
                        .acl(OssAcl::PublicRead)
                        .storage_class(StorageClass::Archive)
                        .data_redundancy_type(DataRedundancyType::LRS)
                        .send()
                        .await;
    match result {
        Ok(_) => println!("{:#?}", result),
        Err(message) => {
            println!("{}", message)
        }
    }
}

#[allow(unused)]
async fn delete_bucket() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client.DeleteBucket()
                    // .name("xuetube-t12")
                    .name("xtoss-t1")
                    .send()
                    .await;

    match result {
        Ok(_) => println!("{:#?}", result),
        Err(message) => {
            println!("{}", message)
        }
    }
}

#[allow(unused)]
async fn location_bucket() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client.GetBucketLocation().name("xtoss-dev1").send().await;

    match result {
        Ok(_) => println!("{:#?}", result),
        Err(message) => {
            println!("{}", message)
        }
    }
}

#[allow(unused)]
async fn get_acl_bucket() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client.GetBucketAcl().await;

    match result {
        Ok(_) => println!("{:#?}", result),
        Err(message) => {
            println!("{}", message)
        }
    }
}

#[allow(unused)]
async fn put_acl_bucket() {
    let options = utils::options_from_env();
    println!("{:#?}",&options);
    let client = oss::Client::new(options);
    let result = client.PutBucketAcl().acl(OssAcl::PublicRead).send().await;


    match result {
        Ok(_) => println!("{:#?}", result),
        Err(message) => {
            println!("{}", message)
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    info_bucket().await;
    // stat_bucket().await;
    // location_bucket().await;
    // get_acl_bucket().await;
    // put_acl_bucket().await;
    // create_bucket().await;
    // delete_bucket().await;
}
