use std::process;

use xt_oss::{
    oss::entities::{ObjectACL, StorageClass},
    prelude::*,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let content_disposition = http::ContentDisposition::ATTACHMENT(Some("测试.xml".to_string()));
    match client
        .PutSymlink("tmp/test.xls")
        .with_symlink_target("excel/Spreadsheet-1000-rows.xls")
        // .with_forbid_overwrite(false)
        .with_content_type("application/vnd.ms-excel")
        .with_object_acl(ObjectACL::Default)
        .with_storage_class(StorageClass::Archive)
        .with_content_disposition(content_disposition)
        .with_oss_meta("name1", "sjy")
        .with_oss_meta("name2", "sun")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("{}", error);
            process::exit(-1);
        }) {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers())
        }
        Err(error_message) => {
            println!("{:#?}", error_message.content())
        }
    }
}
