use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt;

pub mod builder {

    use super::*;

    #[derive(Debug, Default)]
    pub struct MirrorHeadersBuilder<'a> {
        pass_all: bool,
        pass: Vec<&'a str>,
        remove: Vec<&'a str>,
        set: Vec<(&'a str, &'a str)>,
    }

    impl<'a> MirrorHeadersBuilder<'a> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_pass_all(mut self, value: bool) -> Self {
            self.pass_all = value;
            self
        }

        pub fn with_pass(mut self, value: Vec<&'a str>) -> Self {
            self.pass = value;
            self
        }

        pub fn with_remove(mut self, value: Vec<&'a str>) -> Self {
            self.remove = value;
            self
        }

        pub fn with_set(mut self, value: Vec<(&'a str, &'a str)>) -> Self {
            self.set = value;
            self
        }

        pub fn build(&self) -> MirrorHeaders {
            MirrorHeaders {
                pass_all: Some(self.pass_all),
                pass: if self.pass.is_empty() {
                    None
                } else {
                    Some(self.pass.iter().map(|value| value.to_string()).collect())
                },
                remove: if self.remove.is_empty() {
                    None
                } else {
                    Some(self.remove.iter().map(|value| value.to_string()).collect())
                },
                set: if self.set.is_empty() {
                    None
                } else {
                    Some({
                        self.set
                            .iter()
                            .map(|item| Set {
                                key: item.0.to_string(),
                                value: item.1.to_string(),
                            })
                            .collect()
                    })
                },
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct RedirectBuilder<'a> {
        redirect_type: RedirectType,
        protocol: Option<&'a str>,
        pass_query_string: Option<&'a str>,
        replace_key_with: Option<&'a str>,
        mirror_url: Option<&'a str>,
        mirror_pass_query_string: Option<bool>,
        mirror_follow_redirect: Option<bool>,
        mirror_check_md5: Option<bool>,
        mirror_headers: Option<MirrorHeaders>,
        enable_replace_prefix: Option<bool>,
        mirror_replace_prefix: Option<bool>,
        http_replace_code: Option<u16>,
        replace_key_prefix_with: Option<&'a str>,
        host_name: Option<&'a str>,
    }

    impl<'a> RedirectBuilder<'a> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_redirect_type(mut self, value: RedirectType) -> Self {
            self.redirect_type = value;
            self
        }

        pub fn with_protocol(mut self, value: &'a str) -> Self {
            self.protocol = Some(value);
            self
        }

        pub fn pass_query_string(mut self, value: &'a str) -> Self {
            self.pass_query_string = Some(value);
            self
        }

        pub fn with_replace_key_with(mut self, value: &'a str) -> Self {
            self.replace_key_prefix_with = Some(value);
            self
        }

        pub fn with_mirror_url(mut self, value: &'a str) -> Self {
            self.mirror_url = Some(value);
            self
        }

        pub fn with_mirror_pass_query_string(mut self, value: bool) -> Self {
            self.mirror_pass_query_string = Some(value);
            self
        }

        pub fn with_mirror_follow_redirect(mut self, value: bool) -> Self {
            self.mirror_follow_redirect = Some(value);
            self
        }
        pub fn with_mirror_check_md5(mut self, value: bool) -> Self {
            self.mirror_check_md5 = Some(value);
            self
        }

        pub fn with_mirror_headers(mut self, value: MirrorHeaders) -> Self {
            self.mirror_headers = Some(value);
            self
        }

        pub fn with_mirror_replace_prefix(mut self, value: bool) -> Self {
            self.mirror_replace_prefix = Some(value);
            self
        }

        pub fn with_http_replace_code(mut self, value: u16) -> Self {
            self.http_replace_code = Some(value);
            self
        }

        pub fn with_replace_key_prefix_with(mut self, value: &'a str) -> Self {
            self.replace_key_prefix_with = Some(value);
            self
        }

        pub fn with_host_name(mut self, value: &'a str) -> Self {
            self.host_name = Some(value);
            self
        }

        pub fn build(&self) -> Redirect {
            Redirect {
                redirect_type: self.redirect_type.clone(),
                protocol: self.protocol.map(|e| e.to_string()),
                pass_query_string: self.pass_query_string.map(|e| e.to_string()),
                replace_key_with: self.replace_key_with.map(|e| e.to_string()),
                mirror_url: self.mirror_url.map(|e| e.to_string()),
                mirror_pass_query_string: self.mirror_pass_query_string,
                mirror_follow_redirect: self.mirror_follow_redirect,
                mirror_check_md5: self.mirror_check_md5,
                mirror_headers: self.mirror_headers.clone(),
                enable_replace_prefix: self.enable_replace_prefix,
                http_redirect_code: self.http_replace_code,
                replace_key_prefix_with: self.replace_key_prefix_with.map(|e| e.to_string()),
                host_name: self.host_name.map(|e| e.to_string()),
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct ConditionBuilder<'a> {
        pub key_prefix_equals: Option<&'a str>,
        pub http_error_code_returned_equals: Option<u16>,
        pub key_suffix_equals: Option<&'a str>,
        pub include_header_key: Option<&'a str>,
        pub include_header_equals: Option<&'a str>,
    }

    impl<'a> ConditionBuilder<'a> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_key_prefix_equals(mut self, value: &'a str) -> Self {
            self.key_prefix_equals = Some(value);
            self
        }

        pub fn with_http_error_code_returned_equals(mut self, value: u16) -> Self {
            self.http_error_code_returned_equals = Some(value);
            self
        }

        pub fn with_key_suffix_equals(mut self, value: &'a str) -> Self {
            self.key_suffix_equals = Some(value);
            self
        }

        pub fn with_include_header_key(mut self, value: &'a str) -> Self {
            self.include_header_key = Some(value);
            self
        }

        pub fn with_include_header_equals(mut self, value: &'a str) -> Self {
            self.include_header_equals = Some(value);
            self
        }

        pub fn build(&self) -> Condition {
            Condition {
                include_header: if let Some(include_header_key) = self.include_header_key {
                    Some(IncludeHeader {
                        key: include_header_key.to_string(),
                        equals: self.include_header_equals.map(|e| e.to_string()),
                    })
                } else {
                    None
                },
                key_prefix_equals: self.key_prefix_equals.map(|e| e.to_string()),
                http_error_code_returned_equals: self.http_error_code_returned_equals,
                key_suffix_equals: self.key_suffix_equals.map(|e| e.to_string()),
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct IndexDocumentBuilder<'a> {
        suffix: &'a str,
        support_sub_dir: bool,
        r#type: u16,
    }

    impl<'a> IndexDocumentBuilder<'a> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_suffix(mut self, value: &'a str) -> Self {
            self.suffix = value;
            self
        }

        pub fn with_support_sub_dir(mut self, value: bool) -> Self {
            self.support_sub_dir = value;
            self
        }

        pub fn with_type(mut self, value: u16) -> Self {
            self.r#type = value;
            self
        }

        pub fn build(&self) -> IndexDocument {
            IndexDocument {
                suffix: self.suffix.to_string(),
                support_sub_dir: Some(self.support_sub_dir),
                r#type: Some(self.r#type),
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct ErrorDocumentBuilder<'a> {
        key: &'a str,
        http_status: StatusCode,
    }

    impl<'a> ErrorDocumentBuilder<'a> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_key(mut self, value: &'a str) -> Self {
            self.key = value;
            self
        }

        pub fn with_http_status(mut self, value: StatusCode) -> Self {
            self.http_status = value;
            self
        }

        pub fn build(&self) -> ErrorDocument {
            ErrorDocument {
                key: self.key.to_string(),
                http_status: Some(self.http_status.as_u16()),
            }
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct RoutingRuleBuilder {
        rule: RoutingRule,
    }

    impl RoutingRuleBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_rule_number(mut self, value: u32) -> Self {
            self.rule.rule_number = value;
            self
        }

        pub fn with_condition(mut self, value: Condition) -> Self {
            self.rule.condition = value;
            self
        }
        pub fn with_redirect(mut self, value: Redirect) -> Self {
            self.rule.redirect = value;
            self
        }

        pub fn build(&self) -> RoutingRule {
            self.rule.clone()
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct RoutingRulesBuilder {
        rules: Vec<RoutingRule>,
    }

    impl RoutingRulesBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_rule(mut self, value: RoutingRule) -> Self {
            self.rules.push(value);
            self
        }

        pub fn build(&self) -> RoutingRules {
            RoutingRules {
                routing_rule: if self.rules.is_empty() {
                    None
                } else {
                    Some(self.rules.clone())
                },
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct WebsiteConfigurationBuilder {
        pub index_document: Option<IndexDocument>,
        pub error_document: Option<ErrorDocument>,
        pub routing_rules: Option<RoutingRules>,
    }

    impl WebsiteConfigurationBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_index_document(mut self, value: IndexDocument) -> Self {
            self.index_document = Some(value);
            self
        }
        pub fn with_error_document(mut self, value: ErrorDocument) -> Self {
            self.error_document = Some(value);
            self
        }

        pub fn with_routing_rules(mut self, value: RoutingRules) -> Self {
            self.routing_rules = Some(value);
            self
        }

        pub fn build(&self) -> WebsiteConfiguration {
            WebsiteConfiguration {
                index_document: self.index_document.clone(),
                error_document: self.error_document.clone(),
                routing_rules: self.routing_rules.clone(),
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum RedirectType {
    #[default]
    /// 镜像回源
    Mirror,
    /// 外部跳转，即OSS会返回一个3xx请求，指定跳转到另外一个地址
    External,
    /// 阿里云CDN跳转，主要用于阿里云的CDN。与External不同的是，OSS会额外添加
    /// 一个Header。 阿里云CDN识别到此Header后会主动跳转到指定的地址，返回给用
    /// 户获取到的数据，而不是将3xx跳转请求返回给用户
    AliCDN,
}

impl fmt::Display for RedirectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Mirror => "Mirror",
                Self::External => "External",
                Self::AliCDN => "AliCDN",
            }
        )
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Set {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MirrorHeaders {
    #[serde(rename = "PassAll", skip_serializing_if = "Option::is_none")]
    pub pass_all: Option<bool>,
    #[serde(rename = "Pass", skip_serializing_if = "Option::is_none")]
    pub pass: Option<Vec<String>>,
    #[serde(rename = "Remove", skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
    #[serde(rename = "Set", skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<Set>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Redirect {
    #[serde(rename = "RedirectType")]
    pub redirect_type: RedirectType,
    #[serde(rename = "Protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "PassQueryString", skip_serializing_if = "Option::is_none")]
    pub pass_query_string: Option<String>,
    #[serde(rename = "ReplaceKeyWith", skip_serializing_if = "Option::is_none")]
    pub replace_key_with: Option<String>,
    #[serde(rename = "MirrorURL", skip_serializing_if = "Option::is_none")]
    pub mirror_url: Option<String>,
    #[serde(
        rename = "MirrorPassQueryString",
        skip_serializing_if = "Option::is_none"
    )]
    pub mirror_pass_query_string: Option<bool>,
    #[serde(
        rename = "MirrorFollowRedirect",
        skip_serializing_if = "Option::is_none"
    )]
    pub mirror_follow_redirect: Option<bool>,
    #[serde(rename = "MirrorCheckMd5", skip_serializing_if = "Option::is_none")]
    pub mirror_check_md5: Option<bool>,
    #[serde(rename = "MirrorHeaders", skip_serializing_if = "Option::is_none")]
    pub mirror_headers: Option<MirrorHeaders>,
    #[serde(
        rename = "EnableReplacePrefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_replace_prefix: Option<bool>,
    #[serde(rename = "HttpRedirectCode", skip_serializing_if = "Option::is_none")]
    pub http_redirect_code: Option<u16>,
    #[serde(
        rename = "ReplaceKeyPrefixWith",
        skip_serializing_if = "Option::is_none"
    )]
    pub replace_key_prefix_with: Option<String>,
    #[serde(rename = "HostName", skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IncludeHeader {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Equals", skip_serializing_if = "Option::is_none")]
    pub equals: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Condition {
    #[serde(rename = "KeyPrefixEquals", skip_serializing_if = "Option::is_none")]
    pub key_prefix_equals: Option<String>,
    #[serde(
        rename = "HttpErrorCodeReturnedEquals",
        skip_serializing_if = "Option::is_none"
    )]
    pub http_error_code_returned_equals: Option<u16>,
    #[serde(rename = "IncludeHeader", skip_serializing_if = "Option::is_none")]
    pub include_header: Option<IncludeHeader>,
    #[serde(rename = "KeySuffixEquals", skip_serializing_if = "Option::is_none")]
    pub key_suffix_equals: Option<String>,
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
    #[serde(rename = "RoutingRule", skip_serializing_if = "Option::is_none")]
    pub routing_rule: Option<Vec<RoutingRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 默认主页的容器
pub struct IndexDocument {
    #[serde(rename = "Suffix")]
    /// 设置默认主页后，如果访问以正斜线（/）结尾的Object，则OSS都会返回此默认主页。
    pub suffix: String,
    #[serde(rename = "SupportSubDir", skip_serializing_if = "Option::is_none")]
    /// 访问子目录时，是否支持跳转到子目录下的默认主页。取值范围如下：
    pub support_sub_dir: Option<bool>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    /// 设置默认主页后，访问以非正斜线（/）结尾的Object，且该Object不存在时的行为。
    pub r#type: Option<u16>,
}

impl Default for IndexDocument {
    fn default() -> Self {
        Self {
            suffix: "index.html".to_string(),
            support_sub_dir: Some(true),
            r#type: Some(0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDocument {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "HttpStatus", skip_serializing_if = "Option::is_none")]
    pub http_status: Option<u16>,
}

impl Default for ErrorDocument {
    fn default() -> Self {
        Self {
            key: "error.html".to_string(),
            http_status: Some(StatusCode::NOT_FOUND.as_u16()),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WebsiteConfiguration {
    // 默认主页的容器
    #[serde(rename = "IndexDocument", skip_serializing_if = "Option::is_none")]
    pub index_document: Option<IndexDocument>,
    #[serde(rename = "ErrorDocument", skip_serializing_if = "Option::is_none")]
    pub error_document: Option<ErrorDocument>,
    #[serde(rename = "RoutingRules", skip_serializing_if = "Option::is_none")]
    pub routing_rules: Option<RoutingRules>,
}

#[cfg(test)]
pub mod tests {
    use reqwest::StatusCode;

    use super::{
        builder::{
            ConditionBuilder, ErrorDocumentBuilder, IndexDocumentBuilder, MirrorHeadersBuilder,
            RedirectBuilder, RoutingRulesBuilder, WebsiteConfigurationBuilder,
        },
        RedirectType,
    };
    use crate::oss::entities::website::{RoutingRule, Set, WebsiteConfiguration};

    #[test]
    fn website_configuration_parse_1() {
        let xml_content = r#"<WebsiteConfiguration>
<IndexDocument>
    <Suffix>index.html</Suffix>
    <SupportSubDir>true</SupportSubDir>
    <Type>0</Type>
</IndexDocument>
<ErrorDocument>
    <Key>error.html</Key>
    <HttpStatus>404</HttpStatus>
</ErrorDocument>
<RoutingRules>
    <RoutingRule>
    <RuleNumber>1</RuleNumber>
    <Condition>
        <KeyPrefixEquals>abc/</KeyPrefixEquals>
        <HttpErrorCodeReturnedEquals>404</HttpErrorCodeReturnedEquals>
    </Condition>
    <Redirect>
        <RedirectType>Mirror</RedirectType>
        <PassQueryString>true</PassQueryString>
        <MirrorURL>http://example.com/</MirrorURL>   
        <MirrorPassQueryString>true</MirrorPassQueryString>
        <MirrorFollowRedirect>true</MirrorFollowRedirect>
        <MirrorCheckMd5>false</MirrorCheckMd5>
        <MirrorHeaders>
        <PassAll>true</PassAll>
        <Pass>myheader-key1</Pass>
        <Pass>myheader-key2</Pass>
        <Remove>myheader-key3</Remove>
        <Remove>myheader-key4</Remove>
        <Set>
            <Key>myheader-key5</Key>
            <Value>myheader-value5</Value>
        </Set>
        </MirrorHeaders>
    </Redirect>
    </RoutingRule>
    <RoutingRule>
    <RuleNumber>2</RuleNumber>
    <Condition>
        <KeyPrefixEquals>abc/</KeyPrefixEquals>
        <HttpErrorCodeReturnedEquals>404</HttpErrorCodeReturnedEquals>
        <IncludeHeader>
        <Key>host</Key>
        <Equals>test.oss-cn-beijing-internal.aliyuncs.com</Equals>
        </IncludeHeader>
    </Condition>
    <Redirect>
        <RedirectType>AliCDN</RedirectType>
        <Protocol>http</Protocol>
        <HostName>example.com</HostName>
        <PassQueryString>false</PassQueryString>
        <ReplaceKeyWith>prefix/${key}.suffix</ReplaceKeyWith>
        <HttpRedirectCode>301</HttpRedirectCode>
    </Redirect>
    </RoutingRule>
    <RoutingRule>
    <Condition>
        <HttpErrorCodeReturnedEquals>404</HttpErrorCodeReturnedEquals>
    </Condition>
    <RuleNumber>3</RuleNumber>
    <Redirect>
        <ReplaceKeyWith>prefix/${key}</ReplaceKeyWith>
        <HttpRedirectCode>302</HttpRedirectCode>
        <EnableReplacePrefix>false</EnableReplacePrefix>
        <PassQueryString>false</PassQueryString>
        <Protocol>http</Protocol>
        <HostName>example.com</HostName>
        <RedirectType>External</RedirectType>
    </Redirect>
    </RoutingRule>
</RoutingRules>
</WebsiteConfiguration>
"#;

        let object: WebsiteConfiguration = quick_xml::de::from_str(xml_content).unwrap();
        let left = "index.html";
        let right = object.index_document.unwrap().suffix;
        assert_eq!(left, right)
    }

    #[test]
    fn website_configuration_parse_2() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<WebsiteConfiguration>
  <IndexDocument>
    <Suffix>index.html</Suffix>
  </IndexDocument>
  <ErrorDocument>
    <Key>error.html</Key>
    <HttpStatus>404</HttpStatus>
  </ErrorDocument>
  <RoutingRules>
    <RoutingRule>
      <RuleNumber>1</RuleNumber>
      <Condition>
        <KeyPrefixEquals>abc/</KeyPrefixEquals>
        <HttpErrorCodeReturnedEquals>404</HttpErrorCodeReturnedEquals>
      </Condition>
      <Redirect>
        <RedirectType>Mirror</RedirectType>
        <PassQueryString>true</PassQueryString>
        <MirrorURL>http://example.com/</MirrorURL>  
        <MirrorPassQueryString>true</MirrorPassQueryString>
        <MirrorFollowRedirect>true</MirrorFollowRedirect>
        <MirrorCheckMd5>false</MirrorCheckMd5>
        <MirrorHeaders>
          <PassAll>true</PassAll>
          <Pass>myheader-key1</Pass>
          <Pass>myheader-key2</Pass>
          <Remove>myheader-key3</Remove>
          <Remove>myheader-key4</Remove>
          <Set>
            <Key>myheader-key5</Key>
            <Value>myheader-value5</Value>
          </Set>
        </MirrorHeaders>
      </Redirect>
    </RoutingRule>
    <RoutingRule>
      <RuleNumber>2</RuleNumber>
      <Condition>
        <IncludeHeader>
          <Key>host</Key>
          <Equals>test.oss-cn-beijing-internal.aliyuncs.com</Equals>
        </IncludeHeader>
        <KeyPrefixEquals>abc/</KeyPrefixEquals>
        <HttpErrorCodeReturnedEquals>404</HttpErrorCodeReturnedEquals>
      </Condition>
      <Redirect>
        <RedirectType>AliCDN</RedirectType>
        <Protocol>http</Protocol>
        <HostName>example.com</HostName>
        <PassQueryString>false</PassQueryString>
        <ReplaceKeyWith>prefix/${key}.suffix</ReplaceKeyWith>
        <HttpRedirectCode>301</HttpRedirectCode>
      </Redirect>
    </RoutingRule>
  </RoutingRules>
</WebsiteConfiguration>"#;

        let object: WebsiteConfiguration = quick_xml::de::from_str(xml_content).unwrap();
        let left = "index.html";
        let right = object.index_document.unwrap().suffix;
        assert_eq!(left, right)
    }

    #[test]
    fn mirror_headers_builder() {
        let obj = MirrorHeadersBuilder::new()
            .with_pass(["pass1", "pass2"].to_vec())
            .with_remove(["remove1", "remove2"].to_vec())
            .with_pass_all(true)
            .with_set([("lable", "value"), ("label1", "value1")].to_vec())
            .build();
        let left = Set {
            key: "label1".to_string(),
            value: "value1".to_string(),
        };
        let right = &obj.set.unwrap()[1];
        assert_eq!(left.key, right.key);
    }

    #[test]
    fn redirect_builder() {
        let redirect = RedirectBuilder::new()
            .with_host_name("xuetube.com")
            .with_http_replace_code(302)
            .with_mirror_check_md5(true)
            .with_mirror_follow_redirect(false)
            .with_mirror_headers(
                MirrorHeadersBuilder::new()
                    .with_pass(["pass1", "pass2"].to_vec())
                    .with_remove(["remove1", "remove2"].to_vec())
                    .with_pass_all(true)
                    .with_set([("name", "sjy"), ("age", "18")].to_vec())
                    .build(),
            )
            .with_mirror_pass_query_string(false)
            .with_mirror_replace_prefix(false)
            .with_mirror_url("http://example.com")
            .with_protocol("https")
            .with_redirect_type(RedirectType::AliCDN)
            .with_replace_key_with("test")
            .build();
        let left = "pass1";
        let right = &redirect.mirror_headers.unwrap().pass.unwrap()[0];
        assert_eq!(left, right);
    }

    #[test]
    fn condition_builder() {
        let condition = ConditionBuilder::new()
            .with_include_header_key("key")
            .with_include_header_equals("test")
            .with_http_error_code_returned_equals(203)
            .with_key_prefix_equals("prefix")
            .with_key_suffix_equals("suffix")
            .build();
        let left = "key";
        let right = condition.include_header.unwrap().key;
        assert_eq!(left, right);
    }

    #[test]
    fn web_config_builder() {
        let config = WebsiteConfigurationBuilder::new()
            .with_index_document(
                IndexDocumentBuilder::new()
                    .with_suffix("test")
                    .with_support_sub_dir(true)
                    .with_type(200)
                    .build(),
            )
            .with_error_document(
                ErrorDocumentBuilder::new()
                    .with_http_status(StatusCode::NOT_FOUND)
                    .with_key("abcd")
                    .build(),
            )
            .with_routing_rules(
                RoutingRulesBuilder::new()
                    .with_rule(RoutingRule {
                        rule_number: 100,
                        condition: ConditionBuilder::new().build(),
                        redirect: RedirectBuilder::new().build(),
                    })
                    .build(),
            )
            .build();
        let left = r#"<WebsiteConfiguration><IndexDocument><Suffix>test</Suffix><SupportSubDir>true</SupportSubDir><Type>200</Type></IndexDocument><ErrorDocument><Key>abcd</Key><HttpStatus>404</HttpStatus></ErrorDocument><RoutingRules><RoutingRule><RuleNumber>100</RuleNumber><Condition/><Redirect><RedirectType>Mirror</RedirectType></Redirect></RoutingRule></RoutingRules></WebsiteConfiguration>"#;
        let right = quick_xml::se::to_string(&config).unwrap();
        assert_eq!(left, right);
    }
}
