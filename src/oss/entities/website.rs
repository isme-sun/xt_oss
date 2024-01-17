use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Set {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MirrorHeaders {
    #[serde(rename = "PassAll")]
    pub pass_all: bool,
    #[serde(rename = "Pass")]
    pub pass: String,
    #[serde(rename = "Remove")]
    pub remove: String,
    #[serde(rename = "Set")]
    pub set: Set,
    #[serde(rename = "Protocol")]
    pub protocol: String,
    #[serde(rename = "HostName")]
    pub host_name: String,
    #[serde(rename = "ReplaceKeyPrefixWith")]
    pub replace_key_prefix_with: String,
    #[serde(rename = "EnableReplacePrefix")]
    pub enable_replace_prefix: bool,
    #[serde(rename = "ReplaceKeyWith")]
    pub replace_key_with: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Redirect {
    #[serde(rename = "RedirectType")]
    pub redirect_type: String,
    #[serde(rename = "PassQueryString")]
    pub pass_query_string: String,
    #[serde(rename = "MirrorURL")]
    pub mirror_url: String,
    #[serde(rename = "MirrorPassQueryString")]
    pub mirror_pass_query_string: bool,
    #[serde(rename = "MirrorFollowRedirect")]
    pub mirror_follow_redirect: bool,
    #[serde(rename = "MirrorCheckMd5")]
    pub mirror_check_md5: bool,
    #[serde(rename = "MirrorHeaders")]
    pub mirror_headers: MirrorHeaders,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IncludeHeader {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Equals")]
    pub equals: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Condition {
    #[serde(rename = "KeyPrefixEquals")]
    pub key_prefix_equals: String,
    #[serde(rename = "HttpErrorCodeReturnedEquals")]
    pub http_error_code_returned_equals: String,
    #[serde(rename = "IncludeHeader")]
    pub include_header: IncludeHeader,
    #[serde(rename = "KeySuffixEquals")]
    pub key_suffix_equals: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    #[serde(rename = "RuleNumber")]
    pub rule_number: u32,
    #[serde(rename = "Condition")]
    pub condition: Condition,
    #[serde(rename = "Redirect")]
    pub redirect: Redirect,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RoutingRules {
    #[serde(rename = "RoutingRule")]
    pub routing_rule: Option<Vec<RoutingRule>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IndexDocument {
    #[serde(rename = "Suffix")]
    // 设置默认主页后，如果访问以正斜线（/）结尾的Object，则OSS都会返回此默认主页
    pub suffix: String,
    #[serde(rename = "SupportSubDir")]
    pub support_sub_dir: Option<String>,
    #[serde(rename = "Type")]
    pub r#type: Option<u8>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ErrorDocument {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "HttpStatus")]
    pub http_status: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WebsiteConfiguration {
    // 默认主页的容器
    #[serde(rename = "IndexDocument")]
    pub index_document: Option<IndexDocument>,
    #[serde(rename = "ErrorDocument")]
    pub error_document: Option<ErrorDocument>,
    #[serde(rename = "RoutingRules")]
		pub routing_rules: Option<RoutingRules>
}

#[cfg(test)]
pub mod tests {}
