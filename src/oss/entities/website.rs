use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt;

pub mod builder {

	use super::*;

	#[derive(Debug, Default)]
	#[allow(unused)]
	struct MirrorHeadersBuilder<'a> {
		pass_all: bool,
		pass: Vec<&'a str>,
		remove: Vec<&'a str>,
		set: Vec<(&'a str, &'a str)>,
	}

	#[allow(unused)]
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

	#[test]
	fn test_mirror_headers_builder() {
		let obj = MirrorHeadersBuilder::new()
			.with_pass(["abcd", "edf"].to_vec())
			.with_remove(["addf", "asd"].to_vec())
			.with_pass_all(true)
			.with_set([("name", "sjy"), ("age", "18")].to_vec())
			.build();
		println!("{:#?}", obj);
	}

	#[allow(unused)]
	struct RedirectBuilder {}

	#[allow(unused)]
	struct ConditionBuilder {}

	#[allow(unused)]
	#[derive(Debug, Default)]
	struct IndexDocumentBuilder<'a> {
		suffix: &'a str,
		support_sub_dir: bool,
		r#type: u16,
	}

	#[allow(unused)]
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

	#[allow(unused)]
	#[derive(Debug, Default)]
	struct ErrorDocumentBuilder<'a> {
		key: &'a str,
		http_status: StatusCode,
	}

	#[allow(unused)]
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
	///
	/// - true：转到子目录下的默认主页。
	/// - false（默认）：不转到子目录下的默认主页，而是转到根目录下的默认主页。
	///
	/// 假设默认主页为index.html，
	/// 要访问bucket.oss-cn-hangzhou.aliyuncs.com/subdir/，如果设置SupportSubDir为false，则转
	/// 到bucket.oss-cn-hangzhou.aliyuncs.com/index.html；如果设置SupportSubDir为true，
	/// 则转到bucket.oss-cn-hangzhou.aliyuncs.com/subdir/index.html。
	pub support_sub_dir: Option<bool>,
	#[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
	/// 设置默认主页后，访问以非正斜线（/）结尾的Object，且该Object不存在时的行为。
	/// 只有设置SupportSubDir为true时才生效，且生效的顺序在RoutingRule之后、ErrorFile之前。
	/// 假设默认主页为index.html，要访问的文件路径为bucket.oss-cn-hangzhou.aliyuncs.com/abc，
	/// 且abc这个Object不存在，此时Type的不同取值对应的行为如下：
	/// - `0`（默认）：检查abc/index.html是否存在（即Object + 正斜线（/）+ 主页的形式），如果存在则返回302，Location头为/abc/的URL编码（即正斜线（/） + Object + 正斜线（/）的形式），如果不存在则返回404，继续检查ErrorFile。
	/// - `1`：直接返回404，报错NoSuchKey，继续检查ErrorFile。
	/// - `2`：检查abc/index.html是否存在，如果存在则返回该Object的内容；如果不存在则返回404，继续检查ErrorFile。
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
	use crate::oss::entities::website::WebsiteConfiguration;

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

		// let json_str = serde_json::to_string_pretty(&object).unwrap();
		// println!("{}", json_str);
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

		let json_str = serde_json::to_string_pretty(&object).unwrap();
		println!("{}", json_str);
		// let left = "index.html";
		// let right = object.index_document.unwrap().suffix;
		// assert_eq!(left, right)
	}
}
