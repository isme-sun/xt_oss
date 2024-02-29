use serde::{Deserialize, Serialize};

pub mod builder {
    use super::{RefererBlacklist, RefererConfiguration, RefererList};

    #[derive(Debug, Clone)]
    pub struct RefererConfigurationBuilder<'a> {
        allow_empty_referer: bool,
        allow_truncate_query_string: bool,
        truncate_path: bool,
        referer_list: Vec<&'a str>,
        referer_blacklist: Vec<&'a str>,
    }

    impl<'a> Default for RefererConfigurationBuilder<'a> {
        fn default() -> Self {
            Self {
                allow_empty_referer: true,
                allow_truncate_query_string: true,
                truncate_path: true,
                referer_list: Vec::new(),
                referer_blacklist: Vec::new(),
            }
        }
    }

    impl<'a> RefererConfigurationBuilder<'a> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_allow_empty_referer(mut self, value: bool) -> Self {
            self.allow_empty_referer = value;
            self
        }

        pub fn with_allow_truncate_query_string(mut self, value: bool) -> Self {
            self.allow_truncate_query_string = value;
            self
        }

        pub fn with_truncate_path(mut self, value: bool) -> Self {
            self.truncate_path = value;
            self
        }

        pub fn pust_referer(mut self, value: &'a str) -> Self {
            self.referer_list.push(value);
            self
        }

        pub fn with_referer_list(mut self, value: Vec<&'a str>) -> Self {
            self.referer_list = value;
            self
        }

        pub fn with_referer_blacklist(mut self, value: Vec<&'a str>) -> Self {
            self.referer_blacklist = value;
            self
        }

        pub fn build(&self) -> RefererConfiguration {
            RefererConfiguration {
                allow_empty_referer: self.allow_empty_referer,
                allow_truncate_query_string: self.allow_truncate_query_string,
                truncate_path: self.truncate_path,
                referer_list: if self.referer_list.is_empty() {
                    None
                } else {
                    Some(RefererList {
                        referer: self
                            .referer_list
                            .iter()
                            .map(|referer| Some(referer.to_string()))
                            .collect(),
                    })
                },
                referer_blacklist: if self.referer_blacklist.is_empty() {
                    None
                } else {
                    Some(RefererBlacklist {
                        referer: self
                            .referer_blacklist
                            .iter()
                            .map(|referer| Some(referer.to_string()))
                            .collect(),
                    })
                },
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RefererList {
    #[serde(rename = "Referer", skip_serializing_if = "Option::is_none")]
    referer: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RefererBlacklist {
    #[serde(rename = "Referer", skip_serializing_if = "Option::is_none")]
    referer: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RefererConfiguration {
    #[serde(rename = "AllowEmptyReferer")]
    pub allow_empty_referer: bool,
    #[serde(rename = "AllowTruncateQueryString")]
    pub allow_truncate_query_string: bool,
    #[serde(rename = "TruncatePath")]
    pub truncate_path: bool,
    #[serde(rename = "RefererList")]
    pub referer_list: Option<RefererList>,
    #[serde(rename = "RefererBlacklist")]
    pub referer_blacklist: Option<RefererBlacklist>,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn referer_configuration() {
        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<RefererConfiguration>
  <AllowEmptyReferer>false</AllowEmptyReferer>
  <AllowTruncateQueryString>true</AllowTruncateQueryString>
  <TruncatePath>true</TruncatePath>
  <RefererList>
    <Referer>http://www.aliyun.com</Referer>
    <Referer>https://www.aliyun.com</Referer>
    <Referer>http://www.*.com</Referer>
    <Referer>https://www.?.aliyuncs.com</Referer>
  </RefererList>
  <RefererBlacklist>
    <Referer>http://www.refuse.com</Referer>
    <Referer>https://*.hack.com</Referer>
    <Referer>http://ban.*.com</Referer>
    <Referer>https://www.?.deny.com</Referer>
  </RefererBlacklist>
</RefererConfiguration>"#;

        let object: RefererConfiguration = quick_xml::de::from_str(&content).unwrap();
        let left = "http://www.aliyun.com";
        let right = &object.referer_list.unwrap().referer.unwrap()[0];
        assert_eq!(left, right);
    }
}
