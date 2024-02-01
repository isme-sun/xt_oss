use crate::oss;

pub fn get_env(key: &str, default: &str) -> String {
  std::env::var(key).unwrap_or_else(|_| default.to_owned())
}

pub fn get_env_bool(key: &str, default: bool) -> bool {
  std::env::var(key)
    .unwrap_or(default.to_string())
    .parse()
    .unwrap_or(default)
}

pub fn options_from_env() -> oss::Options<'static> {
  oss::Options::new()
    .with_access_key_id(get_env("OSS_ACCESS_KEY_ID", "").leak())
    .with_access_key_secret(get_env("OSS_ACCESS_KEY_SECRET", "").leak())
    .with_region(get_env("OSS_REGION", "").leak())
    .with_bucket(get_env("OSS_BUCKET", "").leak())
    .with_sts_token(get_env("OSS_STS_TOKEN", "").leak())
    .with_internal(get_env_bool("OSS_INTERNAL", false))
    .with_cname(get_env_bool("OSS_CNAME", false))
    .with_is_request_pay(get_env_bool("OSS_IS_REQUEST_PAY", false))
    .with_secret(get_env_bool("OSS_SECURE", false))
    .with_timeout(
      get_env("OSS_TIMEOUT", "60")
        .parse::<u64>()
        .unwrap_or(oss::DEFAULT_TIMEOUT),
    )
}
