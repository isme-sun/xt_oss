use xt_oss::{oss::entities::bucket::ListAllMyBucketsResult, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    // 列出所有以xtoss-开头的bucket
    let all_buckets: ListAllMyBucketsResult = client
        .ListBuckets()
        .with_prefix("xtoss-")
        .execute()
        .await?
        .unwrap()
        .content();

    if let Some(bucktes) = all_buckets.buckets.bucket {
        for bucket in bucktes {
            let result = client
                .DeleteBucket()
                .with_bucket(&bucket.name)
                .with_region(&bucket.location)
                .execute()
                .await
                .unwrap_or_else(|error| {
                    println!("reqwest error: {}", error);
                    std::process::exit(-1);
                });
            match result {
                Ok(_) => println!("delete {} {}", &bucket.location, &bucket.name),
                Err(error) => println!("{}", error.content()),
            }
        }
    } else {
        println!("no match bucket!");
    }
    Ok(())
}
