use xt_oss::{oss, utils};

#[allow(unused)]
async fn get_referer<'a>(client: &'a oss::Client<'_>) {
    let result = client.GetBucketReferer().await;

    match result {
        Ok(result) => {
            println!("{:#?}", result);
            println!("-----------------------");
            println!("referer_list");
            for url in result.data.referer_list {
                println!("{}", url);
            }
            println!("-----------------------");
            println!("referer_blacklist");
            for url in result.data.referer_blacklist {
                println!("{}", url);
            }
            println!("-----------------------");
        }
        Err(message) => {
            println!("{}", message);
        }
    }
}

#[allow(unused)]
async fn put_referer<'a>(client: &'a oss::Client<'_>) {
    let result = client
        .PutBucketReferer()
        .push_to_referer_list("https://www.xuetube.com")
        .push_to_referer_blacklist("https://localhost:3000")
        .push_to_referer_list("https://abc.com")
        .push_to_referer_list("https://123.com")
        .send()
        .await;

    println!("{:#?}", result);
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options).cname(false);
    println!("{:#?}", client.options());
    // put_referer(&client).await;
    get_referer(&client).await;
}
