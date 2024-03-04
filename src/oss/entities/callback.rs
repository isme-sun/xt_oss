use serde_json;
use std::{collections::HashMap, fmt};

pub mod builder {

    use super::{Callback, CallbackBody, CallbackBodyType};

    #[derive(Debug, Default, Clone)]
    pub struct CallbackBuilder<'a> {
        // callback: Callback<'a>,
        url: String,
        host: Option<String>,
        body: CallbackBody<'a>,
        sni: Option<bool>,
        body_type: Option<CallbackBodyType>,
    }

    impl<'a> CallbackBuilder<'a> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_url(mut self, value: Vec<&'a str>) -> Self {
            self.url = value.join(",");
            self
        }

        pub fn with_host(mut self, value: &'a str) -> Self {
            self.host = Some(value.to_string());
            self
        }

        pub fn with_body(mut self, value: CallbackBody<'a>) -> Self {
            self.body = value;
            self
        }

        pub fn with_sni(mut self, value: bool) -> Self {
            self.sni = Some(value);
            self
        }

        pub fn with_body_type(mut self, value: CallbackBodyType) -> Self {
            self.body_type = Some(value);
            self
        }

        pub fn build(self) -> Callback<'a> {
            Callback {
                callback_url: self.url,
                callback_host: self.host,
                callback_body: self.body,
                callback_sni: self.sni,
                callback_body_type: self.body_type,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum CallbackBodyType {
    FormUrlEncoded,
    #[default]
    JSON,
}

impl fmt::Display for CallbackBodyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FormUrlEncoded => write!(f, "{}", "application/x-www-form-urlencoded"),
            Self::JSON => write!(f, "{}", "application/json"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CallbackBodyItem {
    Bucket,
    //对象（文件）的完整路径。
    Object,
    // 文件的ETag,即返回给用户的ETag字段。
    Etag,
    // Object大小。调用CompleteMultipartUpload时,size为整个Object的大小。
    Size,
    // 资源类型,例如jpeg图片的资源类型为image/jpeg。
    MimeType,
    // 图片高度。该变量仅适用于图片格式，对于非图片格式，该变量的值为空。
    ImageInfoHeight,
    // 图片宽度。该变量仅适用于图片格式，对于非图片格式，该变量的值为空。
    ImageInfoWidth,
    // 图片格式,例如JPG、PNG等。该变量仅适用于图片格式,对于非图片格式,该变量的值为空。
    ImageInfoFormat,
    // 与上传文件后返回的x-oss-hash-crc64ecma头内容一致。
    Crc64,
    // 与上传文件后返回的Content-MD5头内容一致。 仅在调用PutObject和PostObject接口上传文件时,该变量的值不为空。
    ContentMD5,
    // 发起请求的客户端所在的VpcId。如果不是通过VPC发起请求,则该变量的值为空。
    VpcId,
    // 发起请求的客户端IP地址。
    ClientIp,
    // 发起请求的RequestId。
    ReqId,
    // 发起请求的接口名称,例如PutObject、PostObject等。
    Operation,
}

impl fmt::Display for CallbackBodyItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallbackBodyItem::Bucket => write!(f, "{}", "bucket=${bucket}"),
            CallbackBodyItem::Object => write!(f, "{}", "object=${object}"),
            CallbackBodyItem::Etag => write!(f, "{}", "etag=${etag}"),
            CallbackBodyItem::Size => write!(f, "{}", "size=${size}"),
            CallbackBodyItem::MimeType => write!(f, "{}", "mimeType=${mimeType}"),
            CallbackBodyItem::ImageInfoHeight => {
                write!(f, "{}", "imageInfo.height=${imageInfo.height}")
            }
            CallbackBodyItem::ImageInfoWidth => {
                write!(f, "{}", "imageInfo.width=${imageInfo.width}")
            }
            CallbackBodyItem::ImageInfoFormat => {
                write!(f, "{}", "imageInfo.format=${imageInfo.format}")
            }
            CallbackBodyItem::Crc64 => write!(f, "{}", "crc64=${crc64}"),
            CallbackBodyItem::ContentMD5 => write!(f, "{}", "contentMd5=${contentMd5}"),
            CallbackBodyItem::VpcId => write!(f, "{}", "vpcId=${vpcId}"),
            CallbackBodyItem::ClientIp => write!(f, "{}", "clientIp=${clientIp}"),
            CallbackBodyItem::ReqId => write!(f, "{}", "reqId=${reqId}"),
            CallbackBodyItem::Operation => write!(f, "{}", "operation=${operation}"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CallbackBody<'a> {
    pub items: Vec<CallbackBodyItem>,
    pub custom_items: Vec<(&'a str, &'a str)>,
}

impl<'a> CallbackBody<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_items(mut self, value: Vec<CallbackBodyItem>) -> Self {
        self.items = value;
        self
    }

    pub fn with_custom_items(mut self, value: Vec<(&'a str, &'a str)>) -> Self {
        self.custom_items = value;
        self
    }

    pub fn to_json(&self) -> String {
        let mut items: HashMap<String, String> = HashMap::new();
        for item in &self.items {
            let value = item.to_string();
            let group = value.split_once("=").unwrap();
            items.insert(group.0.into(), group.1.into());
        }
        for (key, value) in &self.custom_items {
            items.insert(format!("x:{}", key), format!("x:{}", value));
        }
        serde_json::to_string(&items).unwrap()
    }

    pub fn to_form_url_encoded(&self) -> String {
        let sec1 = self.items.iter().map(|e| e.to_string());
        let sec2 = self.custom_items.iter().map(|e| format!("x:{}=x:{}", e.0, e.1));
        sec1.chain(sec2).collect::<Vec<String>>().join("&")
    }
    
}

#[derive(Debug, Default, Clone)]
pub struct Callback<'a> {
    pub callback_url: String,
    pub callback_host: Option<String>,
    pub callback_body: CallbackBody<'a>,
    pub callback_sni: Option<bool>,
    pub callback_body_type: Option<CallbackBodyType>,
}

impl<'a> Callback<'a> {
    pub fn to_json(&self) -> String {
        let mut data = HashMap::new();
        data.insert("callbackUrl".to_string(), self.callback_url.to_owned());
        if let Some(host) = self.callback_host.to_owned() {
            data.insert("callbackHost".to_string(), host);
        }
        data.insert("callbackBody".to_string(), self.callback_body.to_json());
        let body_type = if self.callback_body_type.is_some() {
            self.callback_body_type.unwrap()
        } else {
            CallbackBodyType::JSON
        };
        data.insert("callbackBodyType".to_string(), body_type.to_string());
        data.insert(
            "callbackBody".to_string(),
            match body_type {
                CallbackBodyType::FormUrlEncoded => self.callback_body.to_form_url_encoded(),
                CallbackBodyType::JSON => self.callback_body.to_json(),
            },
        );
        serde_json::to_string(&data).unwrap()
    }
}

#[cfg(test)]
pub mod test {

    use percent_encoding::utf8_percent_encode;

    use crate::oss::entities::callback::builder::CallbackBuilder;

    use super::{CallbackBody, CallbackBodyItem};

    #[test]
    fn urlencode_1() {
        let url = "http://example.com/中文.php?key=value&中文名称=中文值";
        let left = utf8_percent_encode(url, percent_encoding::CONTROLS).to_string();
        let right = "http://example.com/%E4%B8%AD%E6%96%87.php?key=value&%E4%B8%AD%E6%96%87%E5%90%8D%E7%A7%B0=%E4%B8%AD%E6%96%87%E5%80%BC";
        assert_eq!(left, right);
    }

    #[test]
    fn callback_body_1() {
        let items = vec![
            CallbackBodyItem::Bucket,
            CallbackBodyItem::Object,
            CallbackBodyItem::MimeType,
            CallbackBodyItem::Size,
        ];
        let callback_body = CallbackBody::new()
            .with_items(items)
            .with_custom_items(vec![("name", "ktoss"), ("version", "3.1415")]);
        let binding = CallbackBuilder::new()
            .with_url(vec!["https://dev-service.xuetube.com/system/test/xtoss/cb"])
            .with_body(callback_body);
        let callback = binding.build();
        // let callback = binding.build();
        println!("{}", callback.to_json());
    }
}
