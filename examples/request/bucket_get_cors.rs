use std::env;
use xt_oss::oss::Request;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  let access_key_id = env::var("OSS_ACCESS_KEY_ID").unwrap();
  let access_key_secret = env::var("OSS_ACCESS_KEY_SECRET").unwrap();
  let url = "https://xtoss-t1.oss-cn-shanghai.aliyuncs.com/?cors";
  let resp = Request::new()
    .with_access_key_id(&access_key_id)
    .with_access_key_secret(&access_key_secret)
    .task()
    .with_url(&url)
    .with_resource("/xtoss-t1/?cors")
    .execute_timeout(30)
    .await;

  match resp {
    Ok(resp) => {
      println!("is success: {}", resp.status().is_success());
      let status = resp.status();
      let bytes = resp.bytes().await.unwrap();
      let content = String::from_utf8_lossy(&bytes);
      println!("{}", status);
      println!("{}", content);
    }
    Err(error) => {
      println!("{:#?}", error);
    }
  }
}
