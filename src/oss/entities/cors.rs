use serde::{Deserialize, Serialize};

pub mod builder {
    use super::*;
    use crate::{
        oss::http,
        util::{AllowedHeaderItem, AllowedMethodItem, AllowedOriginItem},
    };

    #[derive(Default, Debug, Clone)]
    pub struct CORSRuleBuilder {
        pub rule: CORSRule,
    }
    impl<'a> CORSRuleBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_allowed_origin(mut self, value: AllowedOriginItem) -> Self {
            match value {
                AllowedOriginItem::Any => self.rule.allowed_origin.push("*".to_string()),
                AllowedOriginItem::Urls(urls) => urls
                    .iter()
                    .for_each(|entry| self.rule.allowed_origin.push(entry.to_string())),
            }
            self
        }

        pub fn with_allowed_method(mut self, value: AllowedMethodItem) -> Self {
            match value {
                AllowedMethodItem::Any => {
                    [
                        http::Method::GET,
                        http::Method::PUT,
                        http::Method::POST,
                        http::Method::DELETE,
                        http::Method::HEAD,
                    ]
                    .into_iter()
                    .for_each(|entry| self.rule.allowed_method.push(entry.to_string()));
                }
                AllowedMethodItem::Methods(methods) => {
                    methods
                        .into_iter()
                        .for_each(|entry| self.rule.allowed_method.push(entry.to_string()));
                }
            }
            self
        }

        pub fn with_allowed_header(mut self, value: AllowedHeaderItem) -> Self {
            match value {
                AllowedHeaderItem::Any => {
                    self.rule.allowed_header = Some(vec!["*".to_string()]);
                }
                AllowedHeaderItem::Headers(headers) => {
                    let allowed_headers = headers
                        .into_iter()
                        .map(|entry| entry.to_string())
                        .collect::<Vec<String>>();
                    self.rule.allowed_header = Some(allowed_headers);
                }
            }
            self
        }

        pub fn with_expose_header(mut self, value: Vec<&'a str>) -> Self {
            self.rule.expose_header = Some(value.into_iter().map(|e| e.to_string()).collect());
            self
        }

        pub fn with_max_age_seconds(mut self, value: u32) -> Self {
            self.rule.max_age_seconds = Some(value);
            self
        }

        pub fn build(self) -> CORSRule {
            self.rule
        }
    }

    #[derive(Default, Debug, Clone)]
    pub struct CORSConfigurationBuilder {
        pub cors_configuration: CORSConfiguration,
    }

    impl CORSConfigurationBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn add_rule(mut self, value: CORSRule) -> Self {
            self.cors_configuration.cors_rule.push(value);
            self
        }

        pub fn with_response_vary(mut self, value: bool) -> Self {
            self.cors_configuration.response_vary = Some(value);
            self
        }

        pub fn build(self) -> CORSConfiguration {
            self.cors_configuration
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CORSRule {
    #[serde(rename = "AllowedOrigin")]
    pub allowed_origin: Vec<String>,
    #[serde(rename = "AllowedMethod")]
    pub allowed_method: Vec<String>,
    #[serde(rename = "AllowedHeader")]
    pub allowed_header: Option<Vec<String>>,
    #[serde(rename = "ExposeHeader")]
    pub expose_header: Option<Vec<String>>,
    #[serde(rename = "MaxAgeSeconds")]
    pub max_age_seconds: Option<u32>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct CORSConfiguration {
    #[serde(rename = "CORSRule")]
    pub cors_rule: Vec<CORSRule>,
    #[serde(rename = "ResponseVary", skip_serializing_if = "Option::is_none")]
    pub response_vary: Option<bool>,
}
#[cfg(test)]
pub mod tests {
    use super::builder::*;
    use super::*;
    use crate::oss::http;
    use crate::util::{AllowedHeaderItem, AllowedMethodItem, AllowedOriginItem};

    #[test]
    fn allowed_origin_item_1() {
        let value = AllowedMethodItem::Any;
        assert_eq!("*", value.to_string())
    }

    #[test]
    fn allowed_origin_item_2() {
        let value = AllowedOriginItem::Urls(vec!["http://localhost:3000", "http://localhost:3001"]);
        assert_eq!(
            "http://localhost:3000,http://localhost:3001",
            value.to_string()
        );
    }

    #[test]
    fn allowed_method_item_1() {
        let value = AllowedMethodItem::Any;
        assert_eq!("*", value.to_string())
    }

    #[test]
    fn allowed_method_item_2() {
        let value = AllowedMethodItem::Methods(vec![http::Method::GET, http::Method::POST]);
        assert_eq!("GET,POST", &value.to_string());
    }

    #[test]
    fn allowed_header_item_1() {
        let value = AllowedHeaderItem::Any;
        assert_eq!("*", value.to_string())
    }

    #[test]
    fn allowed_header_item_2() {
        let value = AllowedHeaderItem::Headers(vec![
            http::header::CONTENT_DISPOSITION,
            http::header::CONTENT_LANGUAGE,
        ]);
        assert_eq!("content-disposition,content-language", &value.to_string());
    }

    #[test]
    fn cors_configuration_1() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<CORSConfiguration>
  <CORSRule>
    <AllowedOrigin>*</AllowedOrigin>
    <AllowedMethod>PUT</AllowedMethod>
    <AllowedMethod>GET</AllowedMethod>
    <AllowedHeader>Authorization</AllowedHeader>
  </CORSRule>
  <CORSRule>
    <AllowedOrigin>http://example.com</AllowedOrigin>
    <AllowedOrigin>http://example.net</AllowedOrigin>
    <AllowedMethod>GET</AllowedMethod>
    <AllowedHeader> Authorization</AllowedHeader>
    <ExposeHeader>x-oss-test</ExposeHeader>
    <ExposeHeader>x-oss-test1</ExposeHeader>
    <MaxAgeSeconds>100</MaxAgeSeconds>
  </CORSRule>
  <ResponseVary>false</ResponseVary>
</CORSConfiguration>"#;

        let object = quick_xml::de::from_str::<CORSConfiguration>(xml_content).unwrap();
        assert_eq!(object.cors_rule[0].allowed_origin[0], "*");
    }

    #[test]
    fn cors_configuration_2() {
        let rule1 = CORSRuleBuilder::new()
            .with_allowed_origin(AllowedOriginItem::Any)
            .with_allowed_method(AllowedMethodItem::Any)
            .with_allowed_header(AllowedHeaderItem::Any)
            .build();

        let rule2 = CORSRuleBuilder::new()
            .with_allowed_origin(AllowedOriginItem::Urls(vec![
                "http://localhost:3000",
                "http://localhost:3001",
            ]))
            .with_allowed_method(AllowedMethodItem::Methods(vec![
                http::Method::GET,
                http::Method::POST,
            ]))
            .with_allowed_header(AllowedHeaderItem::Headers(vec![
                http::header::CACHE_CONTROL,
                http::header::CONTENT_ENCODING,
            ]))
            .with_expose_header(vec!["x-oss-test", "x-oss-test1"])
            .build();

        let config = CORSConfigurationBuilder::new()
            .add_rule(rule1)
            .add_rule(rule2)
            .with_response_vary(false)
            .build();

        let left = quick_xml::se::to_string(&config).unwrap().to_string();

        let right = r#"<CORSConfiguration><CORSRule><AllowedOrigin>*</AllowedOrigin><AllowedMethod>GET</AllowedMethod><AllowedMethod>PUT</AllowedMethod><AllowedMethod>POST</AllowedMethod><AllowedMethod>DELETE</AllowedMethod><AllowedMethod>HEAD</AllowedMethod><AllowedHeader>*</AllowedHeader><ExposeHeader/><MaxAgeSeconds/></CORSRule><CORSRule><AllowedOrigin>http://localhost:3000</AllowedOrigin><AllowedOrigin>http://localhost:3001</AllowedOrigin><AllowedMethod>GET</AllowedMethod><AllowedMethod>POST</AllowedMethod><AllowedHeader>cache-control</AllowedHeader><AllowedHeader>content-encoding</AllowedHeader><ExposeHeader>x-oss-test</ExposeHeader><ExposeHeader>x-oss-test1</ExposeHeader><MaxAgeSeconds/></CORSRule><ResponseVary>false</ResponseVary></CORSConfiguration>"#;

        assert_eq!(left, right)
    }
}
