
use super::{StorageClass, Tag};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Not {
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Tag")]
    pub tag: Tag,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Filter {
    #[serde(rename = "Not")]
    pub not: Not,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AbortMultipartUpload {
    #[serde(rename = "Days")]
    pub days: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoncurrentVersionTransition {
    #[serde(rename = "NoncurrentDays", skip_serializing_if = "Option::is_none")]
    pub noncurrent_days: Option<bool>,
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Transition {
    #[serde(rename = "Days")]
    pub days: Option<i32>,
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
    #[serde(rename = "IsAccessTime", skip_serializing_if = "Option::is_none")]
    pub is_access_time: Option<bool>,
    #[serde(
        rename = "ReturnToStdWhenVisit",
        skip_serializing_if = "Option::is_none"
    )]
    pub return_to_std_when_visit: Option<bool>,
    #[serde(rename = "AllowSmallFile", skip_serializing_if = "Option::is_none")]
    pub allow_small_file: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Expiration {
    #[serde(rename = "Days", skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    #[serde(rename = "CreatedBeforeDate", skip_serializing_if = "Option::is_none")]
    pub created_before_date: Option<String>,
    #[serde(
        rename = "ExpiredObjectDeleteMarker",
        skip_serializing_if = "Option::is_none"
    )]
    pub expired_object_delete_marker: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NoncurrentVersionExpiration {
    #[serde(rename = "NoncurrentDays")]
    pub noncurrent_days: i32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Rule {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Transition", skip_serializing_if = "Option::is_none")]
    pub transition: Option<Vec<Transition>>,
    #[serde(rename = "Filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
    #[serde(rename = "Expiration", skip_serializing_if = "Option::is_none")]
    pub expiration: Option<Expiration>,
    #[serde(
        rename = "NoncurrentVersionExpiration",
        skip_serializing_if = "Option::is_none"
    )]
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    #[serde(
        rename = "AbortMultipartUpload",
        skip_serializing_if = "Option::is_none"
    )]
    pub abort_multipart_upload: Option<AbortMultipartUpload>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LifecycleConfiguration {
    #[serde(rename = "Rule")]
    pub rule: Vec<Rule>,
}

pub mod builder {
    use crate::oss::entities::StorageClass;

    use super::{
        AbortMultipartUpload, Expiration, Filter, LifecycleConfiguration,
        NoncurrentVersionExpiration, Rule, Transition,
    };

    #[derive(Default)]
    #[allow(unused)]
    pub struct ExpirationBuilder {
        days: Option<i32>,
        created_before_date: Option<String>,
        expired_object_delete_marker: Option<bool>,
    }

    #[allow(unused)]
    impl ExpirationBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn days(mut self, value: i32) -> Self {
            self.days = Some(value);
            self
        }

        pub fn created_before_date(mut self, value: String) -> Self {
            self.created_before_date = Some(value);
            self
        }

        pub fn expired_object_delete_marker(mut self, value: bool) -> Self {
            self.expired_object_delete_marker = Some(value);
            self
        }

        pub fn builder(&self) -> Expiration {
            Expiration {
                days: self.days,
                created_before_date: self.created_before_date.clone(),
                expired_object_delete_marker: self.expired_object_delete_marker,
            }
        }
    }

    #[derive(Default)]
    #[allow(unused)]
    pub struct TransitionBuilder {
        days: Option<i32>,
        storage_class: StorageClass,
        is_access_time: Option<bool>,
        return_to_std_when_visit: Option<bool>,
        allow_small_file: Option<bool>,
    }

    impl TransitionBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn days(mut self, value: i32) -> Self {
            self.days = Some(value);
            self
        }

        pub fn standard_storage(mut self, value: StorageClass) -> Self {
            self.storage_class = value;
            self
        }

        pub fn is_access_time(mut self, value: bool) -> Self {
            self.is_access_time = Some(value);
            self
        }

        pub fn return_to_std_when_visit(mut self, value: bool) -> Self {
            self.return_to_std_when_visit = Some(value);
            self
        }

        pub fn allow_small_file(mut self, value: bool) -> Self {
            self.allow_small_file = Some(value);
            self
        }

        pub fn builder(&self) -> Transition {
            Transition {
                days: self.days,
                storage_class: self.storage_class.clone(),
                is_access_time: self.is_access_time,
                return_to_std_when_visit: self.return_to_std_when_visit,
                allow_small_file: self.allow_small_file,
            }
        }
    }

    #[derive(Default)]
    #[allow(unused)]
    pub struct RuleBuilder<'a> {
        id: &'a str,
        prefix: &'a str,
        status: &'a str,
        transition: Option<Vec<Transition>>,
        filter: Option<Filter>,
        expiration: Option<Expiration>,
        noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
        abort_multipart_upload: Option<AbortMultipartUpload>,
    }

    #[allow(unused)]
    impl<'a> RuleBuilder<'a> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn id(mut self, value: &'a str) -> Self {
            self.id = value;
            self
        }

        pub fn prefix(mut self, value: &'a str) -> Self {
            self.prefix = value;
            self
        }

        pub fn status(mut self, value: &'a str) -> Self {
            self.status = value;
            self
        }

        pub fn add_transition(mut self, value: Transition) -> Self {
            let transitions = if let Some(mut transitions) = self.transition {
                transitions.push(value);
                transitions
            } else {
                vec![value]
            };
            self.transition = Some(transitions);
            self
        }

        pub fn filter(mut self, value: Filter) -> Self {
            self.filter = Some(value);
            self
        }

        pub fn expiration(mut self, value: Expiration) -> Self {
            self.expiration = Some(value);
            self
        }

        pub fn noncurrent_version_expiration(mut self, value: NoncurrentVersionExpiration) -> Self {
            self.noncurrent_version_expiration = Some(value);
            self
        }

        pub fn abort_multipart_upload(mut self, value: i32) -> Self {
            self.abort_multipart_upload = Some(AbortMultipartUpload { days: value });
            self
        }

        pub fn builder(&self) -> Rule {
            Rule {
                id: self.id.to_string(),
                prefix: self.prefix.to_string(),
                status: self.status.to_string(),
                transition: self.transition.clone(),
                filter: self.filter.clone(),
                expiration: self.expiration.clone(),
                noncurrent_version_expiration: self.noncurrent_version_expiration.clone(),
                abort_multipart_upload: self.abort_multipart_upload.clone(),
            }
        }
    }

    #[derive(Default)]
    #[allow(unused)]
    pub struct LifecycleConfigurationBuilder {
        rules: Vec<Rule>,
    }

    #[allow(unused)]
    impl LifecycleConfigurationBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn add_rule(mut self, value: Rule) -> Self {
            self.rules.push(value);
            self
        }

        pub fn builder(&self) -> LifecycleConfiguration {
            LifecycleConfiguration {
                rule: self.rules.clone(),
            }
        }
    }
}
