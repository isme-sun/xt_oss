use dotenv;
use std::process;
use xt_oss::{
    oss::entities::{
        lifecycle::builder::{ExpirationBuilder, LifecycleConfigurationBuilder, RuleBuilder},
        Status,
    },
    prelude::*,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let expire_180_day = ExpirationBuilder::new().with_days(180).build();
    let expire_30_day = ExpirationBuilder::new().with_days(30).build();

    match client
        .PutBucketLifecycle()
        .with_config(
            LifecycleConfigurationBuilder::new()
                .with_rule(
                    RuleBuilder::new()
                        .with_id("Rule1")
                        .with_prefix("mp3")
                        // Status default Diabled
                        // .with_status(Status::Enabled)
                        .with_expiration(expire_180_day)
                        .build(),
                )
                .with_rule(
                    RuleBuilder::new()
                        .with_id("Rule2")
                        .with_prefix("excel")
                        .with_status(Status::Enabled)
                        .with_expiration(expire_30_day)
                        .build(),
                )
                .build(),
        )
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("{}", error);
            process::exit(-1);
        }) {
        Ok(data) => {
            println!("{:#?}", data.headers())
        }
        Err(message) => {
            println!("{:#?}", message.content())
        }
    }
}
