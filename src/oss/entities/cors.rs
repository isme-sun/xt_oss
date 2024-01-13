use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
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
    pub max_age_seconds: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CORSConfiguration {
    #[serde(rename = "CORSRule")]
    pub cors_rule: Vec<CORSRule>,
    #[serde(rename = "ResponseVary", skip_serializing_if = "Option::is_none")]
    pub response_vary: Option<bool>,
}

pub mod builder {
    use super::*;
    use crate::oss;

    #[derive(Default, Debug)]
    #[allow(unused)]
    pub struct CORSRuleBuilder {
        pub rule: CORSRule,
    }
    impl<'a> CORSRuleBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn allowed_origin(mut self, value: &'a str) -> Self {
            self.rule.allowed_origin.push(value.to_string());
            self
        }

        pub fn allowed_method(mut self, value: oss::Method) -> Self {
            self.rule.allowed_method.push(value.to_string());
            self
        }

        pub fn allowed_header(mut self, value: oss::header::HeaderName) -> Self {
            if let Some(mut header_list) = self.rule.allowed_header {
                header_list.push(value.to_string());
                self.rule.allowed_header = Some(header_list)
            } else {
                let header_list = vec![value.to_string()];
                self.rule.allowed_header = Some(header_list);
            }
            self
        }

        pub fn expose_header(mut self, value: &'a str) -> Self {
            if let Some(mut expose_header_list) = self.rule.expose_header {
                expose_header_list.push(value.to_string());
                self.rule.expose_header = Some(expose_header_list)
            } else {
                let expose_header_list = vec![value.to_string()];
                self.rule.expose_header = Some(expose_header_list);
            }
            self
        }

        pub fn max_age_seconds(mut self, value: i32) -> Self {
            self.rule.max_age_seconds = Some(value);
            self
        }

        pub fn builder(self) -> CORSRule {
            self.rule
        }
    }

    pub struct CORSConfigurationBuilder {
        pub cors_configuration: CORSConfiguration,
    }

    impl CORSConfigurationBuilder {
        pub fn new() -> Self {
            Self {
                cors_configuration: CORSConfiguration::default(),
            }
        }

        pub fn add_rule(mut self, value: CORSRule) -> Self {
            self.cors_configuration.cors_rule.push(value);
            self
        }

        pub fn response_vary(mut self, value: bool) -> Self {
            self.cors_configuration.response_vary = Some(value);
            self
        }

        pub fn builder(self) -> CORSConfiguration {
            self.cors_configuration
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::builder::*;
    use super::*;
    use crate::oss;
    #[test]
    fn cors_configuration1() {
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

        let object = quick_xml::de::from_str::<CORSConfiguration>(&xml_content).unwrap();
        assert_eq!(object.cors_rule[0].allowed_origin[0], "*");
    }

    #[test]
    fn cors_configuration2() {
        let rule1 = CORSRuleBuilder::new()
            .allowed_origin("*")
            .allowed_method(oss::Method::PUT)
            .allowed_method(oss::Method::GET)
            .allowed_header(oss::header::AUTHORIZATION)
            .builder();

        let rule2 = CORSRuleBuilder::new()
            .allowed_origin("http://example.com")
            .allowed_origin("http://example.net")
            .allowed_method(oss::Method::GET)
            .allowed_header(oss::header::AUTHORIZATION)
            .expose_header("x-oss-test")
            .expose_header("x-oss-test1")
            .builder();

        let config = CORSConfigurationBuilder::new()
            .add_rule(rule1)
            .add_rule(rule2)
            .response_vary(false)
            .builder();

        let left = format!("{}", quick_xml::se::to_string(&config).unwrap());

        let right = r#"<CORSConfiguration><CORSRule><AllowedOrigin>*</AllowedOrigin><AllowedMethod>PUT</AllowedMethod><AllowedMethod>GET</AllowedMethod><AllowedHeader>authorization</AllowedHeader><ExposeHeader/><MaxAgeSeconds/></CORSRule><CORSRule><AllowedOrigin>http://example.com</AllowedOrigin><AllowedOrigin>http://example.net</AllowedOrigin><AllowedMethod>GET</AllowedMethod><AllowedHeader>authorization</AllowedHeader><ExposeHeader>x-oss-test</ExposeHeader><ExposeHeader>x-oss-test1</ExposeHeader><MaxAgeSeconds/></CORSRule><ResponseVary>false</ResponseVary></CORSConfiguration>"#;

        assert_eq!(left, right)
        // println!("{}", quick_xml::se::to_string(&config).unwrap())
    }
}
