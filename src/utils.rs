use std::env;

use crate::oss;

pub fn options_from_env() -> oss::Options<'static> {
  oss::Options::new()
    .with_access_key_id(env::var("OSS_ACCESS_KEY_ID").unwrap_or_default().leak())
    .with_access_key_secret(env::var("OSS_ACCESS_KEY_SECRET").unwrap_or_default().leak())
    .with_region(env::var("OSS_REGION").unwrap_or_default().leak())
    .with_bucket(env::var("OSS_BUCKET").unwrap_or_default().leak())
    .with_sts_token(env::var("OSS_STS_TOKEN").unwrap_or_default().leak())
    .with_internal(
      env::var("OSS_INTERNAL")
        .unwrap_or_default()
        .parse::<bool>()
        .unwrap_or(false),
    )
    .with_cname(
      env::var("OSS_CNAME")
        .unwrap_or_default()
        .parse::<bool>()
        .unwrap_or(false),
    )
    .with_is_request_pay(
      env::var("OSS_IS_REQUEST_PAY")
        .unwrap_or_default()
        .parse::<bool>()
        .unwrap_or(false),
    )
    .with_secret(
      env::var("OSS_SECRET")
        .unwrap_or_default()
        .parse::<bool>()
        .unwrap_or(false),
    )
    .with_timeout(
      env::var("OSS_TIMEOUT")
        .unwrap_or(oss::DEFAULT_TIMEOUT.to_string())
        .parse::<u64>()
        .unwrap_or(60),
    )
}
