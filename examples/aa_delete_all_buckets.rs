use std::process;
use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    // 列出所有以xtoss-开头的bucket
    let result = client
        .ListBuckets()
        .with_prefix("xtoss-")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });

    let all_buckets = match result {
        Ok(data) => data.content(),
        Err(error) => {
            println!("oss error: {}", error.content());
            process::exit(-1);
        }
    };

    // 删除这些bucket
    // TODO 改有join模式
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
}