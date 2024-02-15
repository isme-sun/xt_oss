use std::process;

use xt_oss::{oss::{self, entities::ObjectACL}, utils};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    match client
        .PutObjectACL("excel/Spreadsheet-1000-rows.xls")
        .with_acl(ObjectACL::PublicReadWrite)
        .with_version_id("CAEQmgEYgYDA9I_smO0YIiBhOGJmMTczNzY0ZmM0NTE1YTA5MDJlOWE1YmI1ZTZlNQ--")
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
