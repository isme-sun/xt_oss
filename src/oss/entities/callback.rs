use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub enum CallbackBodyType {
    FormUrlEncoded,
    #[default]
    JSON,
}

impl fmt::Display for CallbackBodyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FormUrlEncoded => write!(f, "{}", "a"),
            Self::JSON => write!(f, "{}", "b"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CallbackBody {
    /// 存储空间名称。
    pub bucket: String,
    ///对象（文件）的完整路径。
    pub object: String,
    /// 文件的ETag,即返回给用户的ETag字段。
    pub etag: String,
    /// Object大小。调用CompleteMultipartUpload时,size为整个Object的大小。
    pub size: String,
    /// 资源类型,例如jpeg图片的资源类型为image/jpeg。
    pub mime_type: String,
    /// 图片高度。该变量仅适用于图片格式，对于非图片格式，该变量的值为空。
    pub imageinfo_height: String,
    /// 图片宽度。该变量仅适用于图片格式，对于非图片格式，该变量的值为空。
    pub imageinfo_width: String,
    /// 图片格式,例如JPG、PNG等。该变量仅适用于图片格式,对于非图片格式,该变量的值为空。
    pub imageinfo_format: String,
    /// 与上传文件后返回的x-oss-hash-crc64ecma头内容一致。
    pub crc64: String,
    /// 与上传文件后返回的Content-MD5头内容一致。 仅在调用PutObject和PostObject接口上传文件时,该变量的值不为空。
    pub content_md5: String,
    /// 发起请求的客户端所在的VpcId。如果不是通过VPC发起请求,则该变量的值为空。
    pub vpc_id: String,
    /// 发起请求的客户端IP地址。
    pub client_ip: String,
    /// 发起请求的RequestId。
    pub req_id: String,
    /// 发起请求的接口名称,例如PutObject、PostObject等。
    pub operation: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Callback {
    pub callback_url: String,
    pub callback_host: Option<String>,
    pub callback_body: CallbackBody,
    pub callback_sni: Option<String>,
    pub callback_body_type: Option<CallbackBodyType>,
}
