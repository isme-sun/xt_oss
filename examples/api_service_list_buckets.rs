use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .ListBuckets()
        // .with_marker("marker")
        // .with_max_keys(100)
        // .with_prefix("xtoss")
        // .with_resource_group_id("group_id")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });

    match result {
        Ok(data) => {
            let all_buckets = data.content();
            // println!("{:#?}", all_buckets);
            if let Some(buckets) = &all_buckets.buckets.bucket {
                for bucket in buckets {
                    println!("{}", bucket.name);
                    println!("{}", "=".repeat(bucket.name.len()));
                    println!(" - storage_class: {}", bucket.storage_class);
                    println!(" - creation_date): {}", bucket.creation_date);
                    println!()
                }
            } else {
                // println!("{:#?}", all_buckets);
                println!("no buckets");
            }
        }
        Err(message) => println!("oss error: {}", message.content()),
    }
    Ok(())
}
