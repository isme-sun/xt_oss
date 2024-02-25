use super::http;
use super::http::header::CONTENT_TYPE;
use super::DEFAULT_CONTENT_TYPE;
use base64::{engine::general_purpose, Engine as _};
pub(super) struct SingerV1<'a> {
    pub(super) access_key_id: &'a str,
    pub(super) access_key_secret: &'a str,
    #[allow(unused)]
    pub(super) sts_token: Option<&'a str>,
    pub(super) headers: &'a http::HeaderMap,
    pub(super) method: &'a http::Method,
    pub(super) date: &'a String,
    pub(super) resourse: Option<&'a str>,
}
impl<'a> SingerV1<'a> {
    pub(super) fn complute(&self) -> String {
        format!("OSS {}:{}", self.access_key_id, self.signature())
    }

    fn headers_str(&self) -> String {
        // dbg!(self.headers);
        let mut oss_key_name: Vec<&str> = self
            .headers
            .keys()
            .filter_map(|k| {
                let key = k.as_str();
                if key.starts_with("x-oss") {
                    Some(key)
                } else {
                    None
                }
            })
            .collect();
        oss_key_name.sort();
        // dbg!(&oss_key_name);
        let mut value: Vec<String> = Vec::new();
        for key_name in oss_key_name {
            if let Some(key_value) = self.headers.get(key_name).and_then(|v| v.to_str().ok()) {
                value.push(format!("{}:{}\n", key_name, key_value));
            }
        }

        value.join("")
    }

    fn signature(&self) -> String {
        let header_str = self.headers_str();
        let content_type = match self.headers.get(CONTENT_TYPE) {
            Some(content_type) => content_type.to_str().unwrap(),
            None => DEFAULT_CONTENT_TYPE,
        };
        let content_md5 = match self.headers.get("content-md5") {
            Some(content_type) => content_type.to_str().unwrap().to_string(),
            None => "".to_string(),
        };

        // !! ? :(
        let resource = urlencoding::decode(self.resourse.unwrap_or("/"))
            .unwrap()
            .replace("+", " ");

        let value = format!(
            "{VERB}\n{ContentMD5}\n{ContentType}\n{Date}\n{Header}{Resource}",
            VERB = self.method,
            ContentMD5 = content_md5,
            ContentType = content_type,
            Date = self.date,
            Header = header_str,
            Resource = resource
        );
        // dbg!(&value);
        let key = self.access_key_secret.as_bytes();
        let message = value.as_bytes();
        let value = hmac_sha1::hmac_sha1(key, message);
        let encoded = general_purpose::STANDARD.encode(value.as_slice());
        encoded
    }
}
