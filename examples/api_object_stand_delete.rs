use std::process;

use xt_oss::{
    oss::{self},
    utils,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let resp = client
        .DeleteObject("tmp/test.txt")
        // .with_version_id("CAEQ2AEYgYCA1v6ot.sYIiBmZjU2NTQwOGEwZDc0MTMyYTU5ZjhlMmUyNGYwMjc3NA--")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("{}", error);
            process::exit(-1);
        });

    // CAEQ2AEYgYDA_66nt.sYIiA5NTUzMjE0YzcwZGE0N2MyYTUxY2QxNmY1MGIxNjgzMQ--
    // CAEQ2AEYgYCAur2ot.sYIiBmM2M5MDBjNDE0OWE0OGVmYTYwN2Q1OWIyMGNlZDQ3Ng--
    // CAEQ2AEYgYCA1v6ot.sYIiBmZjU2NTQwOGEwZDc0MTMyYTU5ZjhlMmUyNGYwMjc3NA--
    match resp {
        Ok(data) => {
            println!("{:#?}", data.headers())
        }
        Err(message) => {
            println!("{:#?}", message.content())
        }
    }
}
