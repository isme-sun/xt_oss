use serde::{Deserialize, Serialize};

pub(crate) mod inner {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RefererList {
        #[serde(rename = "Referer")]
        pub referer: Option<Vec<String>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RefererBlacklist {
        #[serde(rename = "Referer")]
        pub referer: Option<Vec<String>>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefererConfiguration {
    #[serde(rename = "AllowEmptyReferer")]
    pub allow_empty_referer: bool,
    #[serde(rename = "AllowTruncateQueryString")]
    pub allow_truncate_query_string: bool,
    #[serde(rename = "TruncatePath")]
    pub truncate_path: bool,
    #[serde(rename = "RefererList")]
    pub referer_list: Vec<String>,
    #[serde(rename = "RefererBlacklist")]
    pub referer_blacklist: Vec<String>,
}

impl Default for RefererConfiguration {
    fn default() -> Self {
        Self {
            allow_empty_referer: false,
            allow_truncate_query_string: true,
            truncate_path: true,
            referer_list: Default::default(),
            referer_blacklist: Default::default(),
        }
    }
}

#[allow(unused)]
impl RefererConfiguration {
    pub(crate) fn from_inner(config: inner::RefererConfiguration) -> Self {
        let mut referer_list: Vec<String> = Vec::new();
        let mut referer_blacklist: Vec<String> = Vec::new();

        if let Some(inner_referer_list) = config.referer_list {
            if let Some(referer) = inner_referer_list.referer {
                for url in referer {
                    referer_list.push(url);
                }
            }
        }

        if let Some(inner_referer_blacklist) = config.referer_blacklist {
            if let Some(referer) = inner_referer_blacklist.referer {
                for url in referer {
                    referer_blacklist.push(url);
                }
            }
        }

        let config = RefererConfiguration {
            allow_empty_referer: config.allow_empty_referer,
            allow_truncate_query_string: config.allow_truncate_query_string,
            truncate_path: config.truncate_path,
            referer_list,
            referer_blacklist,
        };
        config
    }

    pub(crate) fn to_inner(&self) -> inner::RefererConfiguration {
        let referer_list = {
            if self.referer_list.len() > 0 {
                Some(inner::RefererList {
                    referer: Some({
                        let mut referer: Vec<String> = Vec::new();
                        for url in &self.referer_list {
                            referer.push(url.to_string())
                        }
                        referer
                    }),
                })
            } else {
                None
            }
        };
        let referer_blacklist = {
            if self.referer_blacklist.len() > 0 {
                Some(inner::RefererBlacklist {
                    referer: Some({
                        let mut referer: Vec<String> = Vec::new();
                        for url in &self.referer_blacklist {
                            referer.push(url.to_string())
                        }
                        referer
                    }),
                })
            } else {
                None
            }
        };
        let config = inner::RefererConfiguration {
            allow_empty_referer: self.allow_empty_referer,
            allow_truncate_query_string: self.allow_truncate_query_string,
            truncate_path: self.truncate_path,
            referer_list,
            referer_blacklist,
        };
        config
    }
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

        let object: inner::RefererConfiguration = quick_xml::de::from_str(&content).unwrap();
        println!("{:#?}", object);
    }
}
