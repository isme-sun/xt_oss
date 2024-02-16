use std::process;

use xt_oss::{oss::{self, entities::{ContentDisposition, ObjectACL, StorageClass}}, utils};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    match client
        .PutSymlink("tmp/test.xls")
        .with_symlink_target("excel/Spreadsheet-1000-rows.xls")
        // .with_forbid_overwrite(false)
        .with_content_type("application/vnd.ms-excel")
        .with_object_acl(ObjectACL::Default)
        .with_storage_class(StorageClass::Archive)
        .with_content_disposition(ContentDisposition::ATTACHMENT(Some("测试.xml".to_string())))
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