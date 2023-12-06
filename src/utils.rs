use std::env;

pub fn options_from_env() -> crate::oss::Options<'static> {
    crate::oss::Options::new()
        .access_key_id(env::var("OSS_ACCESS_KEY_ID").unwrap_or_default().leak())
        .access_key_secret(env::var("OSS_ACCESS_KEY_SECRET").unwrap_or_default().leak())
        .region(env::var("OSS_REGION").unwrap_or_default().leak())
        .bucket(env::var("OSS_BUCKET").unwrap_or_default().leak())
        .sts_token(env::var("OSS_STS_TOKEN").unwrap_or_default().leak())
        .internal(
            env::var("OSS_INTERNAL")
                .unwrap_or_default()
                .parse::<bool>()
                .unwrap_or(false),
        )
        .cname(
            env::var("OSS_CNAME")
                .unwrap_or_default()
                .parse::<bool>()
                .unwrap_or(false),
        )
        .is_request_pay(
            env::var("OSS_IS_REQUEST_PAY")
                .unwrap_or_default()
                .parse::<bool>()
                .unwrap_or(false),
        )
        .secret(
            env::var("OSS_SECRET")
                .unwrap_or_default()
                .parse::<bool>()
                .unwrap_or(false),
        )
        .timeout(
            env::var("OSS_TIMEOUT")
                .unwrap_or_default()
                .parse::<u64>()
                .unwrap_or(60),
        )
}

#[cfg(test)]
mod tests {
    use crate::oss::Options;
    use crate::utils::options_from_env;

    #[test]
    fn options_env() {
        dotenv::dotenv().ok();
        let options: Options = options_from_env();
        let base_url = "https://xuetube-dev.oss-cn-shanghai.aliyuncs.com";
        assert_eq!(base_url, options.base_url());
    }
}
