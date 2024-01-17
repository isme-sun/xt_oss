use xt_oss::{
    oss::{
        self,
        entities::{
            lifecycle::builder::{
                ExpirationBuilder, LifecycleConfigurationBuilder, RuleBuilder, TransitionBuilder,
            },
            StorageClass,
        },
    },
    utils,
};

#[allow(unused)]
async fn put_bucket_lifecycle() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let rule = RuleBuilder::new()
        .with_id("RuleID")
        .with_prefix("Prefix")
        .with_status("Enabled")
        .with_expiration(ExpirationBuilder::new().with_days(4).build())
        .with_transition(
            TransitionBuilder::new()
                .with_days(3)
                .with_torage_class(StorageClass::Archive)
                .build(),
        )
        .with_abort_multipart_upload(3)
        .build();

    let config = LifecycleConfigurationBuilder::new().with_rule(rule).build();

    let resp = client.PutBucketLifecycle(config).await;

    match resp {
        Ok(logging_status) => println!("{:#?}", logging_status),
        Err(message) => println!("{:#?}", message),
    }
}

#[allow(unused)]
async fn get_bucket_lifecycle() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let resp = client.GetBucketLifecycle().await;

    match resp {
        Ok(result) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&result.data).unwrap()
            );
        }
        Err(message) => println!("{:#?}", message),
    }
}

#[allow(unused)]
async fn delete_bucket_lifecycle() {
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let resp = client
        .DeleteBucketLifecycle()
        .await;
    println!("{:#?}", resp);
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // put_bucket_lifecycle().await;
    // get_bucket_lifecycle().await;
    delete_bucket_lifecycle().await;
}
