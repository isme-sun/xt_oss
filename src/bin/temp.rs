use reqwest::Client;
use xt_oss::{DEFAULT_REGION, OSS_BASE_URL};

#[allow(unused)]
#[derive(Debug)]
struct Options<'a> {
    pub access_key_id: &'a str,
    pub access_key_secret: &'a str,
    pub sts_token: &'a str,
    pub bucket: &'a str,
    pub endpoint: &'a str,
    pub region: &'a str,
    pub internal: bool,
    pub cname: bool,
    pub is_request_pay: bool,
    pub secure: bool,
    pub timeout: u64,
}

impl<'a> Default for Options<'a> {
    fn default() -> Self {
        Self {
            access_key_id: Default::default(),
            access_key_secret: Default::default(),
            sts_token: Default::default(),
            bucket: Default::default(),
            endpoint: Default::default(),
            region: DEFAULT_REGION,
            internal: false,
            cname: true,
            is_request_pay: false,
            secure: true,
            timeout: 60,
        }
    }
}

#[allow(unused)]
impl<'a> Options<'a> {
    fn root_url(&self) -> String {
        format!("{}://{}.{}", self.schema(), self.region, OSS_BASE_URL)
    }

    fn base_url(&self) -> String {
        format!(
            "{}://{}.{}.{}",
            self.schema(),
            self.bucket,
            self.region,
            OSS_BASE_URL
        )
    }

    fn schema(&self) -> &'a str {
        if self.secure == true {
            "https"
        } else {
            "http"
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
struct OssClient<'a> {
    options: Options<'a>,
    client: Client,
}

impl<'a> OssClient<'a> {
    #[allow(unused_mut)]
    fn new(option: &'a Options) -> Self {
        let mut opt = Options { ..*option };
        let client = OssClient {
            options: opt,
            client: reqwest::Client::new(),
        };
        client
    }

}

#[tokio::main]
async fn main() {
    let option = Options::default();
    let mut client = OssClient::new(&option);

    client.options.access_key_id = "aaaaaaaa";
    client.options.access_key_secret = "bbbbbbb";

    println!("{:#?}", client);
    // println!("{:#?}", option);
    // println!("{}", option.root_url());
    // println!("{}", option.base_url());
    // println!("{}", option.schema());
}
