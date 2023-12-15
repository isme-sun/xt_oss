use xt_oss::{
    oss::{self, arguments::OssAcl},
    utils,
};

#[allow(unused)]
async fn info_bucket() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client.GetBucketInfo().name("xuetube-t12").send().await;
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
    let result = client.PutBucket().name("xuetube-t12").send().await;
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
    let result = client.DeleteBucket().name("xuetube-t12").send().await;

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
    let result = client.GetBucketLocation().name("xuetube-dev1").send().await;

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
    let result = client.PutBucketAcl().acl(OssAcl::Private).send().await;


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
    // create_bucket().await;
    // stat_bucket().await;
    // location_bucket().await;
    // get_acl_bucket().await;
    put_acl_bucket().await;


}
