use super::{tag::Tag, StorageClass};
use serde::{Deserialize, Serialize};

// todo FilterBuild
pub mod builder {
	use crate::oss::entities::StorageClass;

	use super::{
		AbortMultipartUpload, Expiration, Filter, LifecycleConfiguration,
		NoncurrentVersionExpiration, Rule, Transition,
	};

	pub struct FilterBuilder {}

	impl FilterBuilder {
		pub fn new() -> Self {
			Self {}
		}

		pub fn build() -> Filter {
			Filter::default()
		}
	}

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

		pub fn with_days(mut self, value: i32) -> Self {
			self.days = Some(value);
			self
		}

		pub fn with_created_before_date(mut self, value: String) -> Self {
			self.created_before_date = Some(value);
			self
		}

		pub fn with_expired_object_delete_marker(mut self, value: bool) -> Self {
			self.expired_object_delete_marker = Some(value);
			self
		}

		pub fn build(&self) -> Expiration {
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

		pub fn with_days(mut self, value: i32) -> Self {
			self.days = Some(value);
			self
		}

		pub fn with_torage_class(mut self, value: StorageClass) -> Self {
			self.storage_class = value;
			self
		}

		pub fn with_is_access_time(mut self, value: bool) -> Self {
			self.is_access_time = Some(value);
			self
		}

		pub fn with_return_to_std_when_visit(mut self, value: bool) -> Self {
			self.return_to_std_when_visit = Some(value);
			self
		}

		pub fn with_allow_small_file(mut self, value: bool) -> Self {
			self.allow_small_file = Some(value);
			self
		}

		pub fn build(&self) -> Transition {
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

		pub fn with_id(mut self, value: &'a str) -> Self {
			self.id = value;
			self
		}

		pub fn with_prefix(mut self, value: &'a str) -> Self {
			self.prefix = value;
			self
		}

		pub fn with_status(mut self, value: &'a str) -> Self {
			self.status = value;
			self
		}

		pub fn with_transition(mut self, value: Transition) -> Self {
			let transitions = if let Some(mut transitions) = self.transition {
				transitions.push(value);
				transitions
			} else {
				vec![value]
			};
			self.transition = Some(transitions);
			self
		}

		pub fn with_filter(mut self, value: Filter) -> Self {
			self.filter = Some(value);
			self
		}

		pub fn with_expiration(mut self, value: Expiration) -> Self {
			self.expiration = Some(value);
			self
		}

		pub fn with_noncurrent_version_expiration(mut self, days: i32) -> Self {
			self.noncurrent_version_expiration = Some(NoncurrentVersionExpiration {
				noncurrent_days: days,
			});
			self
		}

		pub fn with_abort_multipart_upload(mut self, days: i32) -> Self {
			self.abort_multipart_upload = Some(AbortMultipartUpload { days });
			self
		}

		pub fn build(&self) -> Rule {
			Rule {
				// id: self.id.to_string(),
				id: self.id.into(),
				prefix: self.prefix.into(),
				status: self.status.into(),
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

		pub fn with_rule(mut self, value: Rule) -> Self {
			self.rules.push(value);
			self
		}

		pub fn build(&self) -> LifecycleConfiguration {
			LifecycleConfiguration {
				rule: self.rules.clone(),
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Not {
	#[serde(rename = "Prefix")]
	pub prefix: String,
	#[serde(rename = "Tag")]
	pub tag: Tag,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Filter {
	#[serde(rename = "Not", skip_serializing_if = "Option::is_none")]
	pub not: Option<Not>,
	#[serde(
		rename = "ObjectSizeGreaterThan",
		skip_serializing_if = "Option::is_none"
	)]
	pub object_size_greater_than: Option<i32>,
	#[serde(rename = "ObjectSizeLessThan", skip_serializing_if = "Option::is_none")]
	pub object_size_less_than: Option<i32>,
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

#[cfg(test)]
// https://help.aliyun.com/zh/oss/developer-reference/getbucketlifecycle?spm=a2c4g.11186623.0.0.738c1e73D0wqnM
mod tests {
	use super::builder::*;
	use super::*;

	#[test]
	/// 返回内容解析测试
	fn lifecycle_configuration_return_parse1() {
		let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<LifecycleConfiguration>
  <Rule>
    <ID>delete after one day</ID>
    <Prefix>logs1/</Prefix>
    <Status>Enabled</Status>
    <Expiration>
      <Days>1</Days>
    </Expiration>
  </Rule>
  <Rule>
  <ID>mtime transition1</ID>
  <Prefix>logs2/</Prefix>
  <Status>Enabled</Status>
  <Transition>
     <Days>30</Days>
     <StorageClass>IA</StorageClass>
  </Transition>
 </Rule>
 <Rule>
   <ID>mtime transition2</ID>
   <Prefix>logs3/</Prefix>
   <Status>Enabled</Status>
   <Transition>
     <Days>30</Days>
     <StorageClass>IA</StorageClass>
     <IsAccessTime>false</IsAccessTime>
  </Transition>
 </Rule>
</LifecycleConfiguration>"#;
		let object: LifecycleConfiguration = quick_xml::de::from_str(&xml_content).unwrap();

		// let json_str = serde_json::to_string_pretty(&object).unwrap();
		// println!("{}", json_str);
		let left = "mtime transition1";
		let right = &object.rule[1].id;
		assert_eq!(left, right);
	}

	#[test]
	// 返回结果分析测试
	fn lifecycle_configuration_return_parse2() {
		let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<LifecycleConfiguration>
  <Rule>
    <ID>atime transition1</ID>
    <Prefix>logs1/</Prefix>
    <Status>Enabled</Status>
      <Transition>
         <Days>30</Days>
         <StorageClass>IA</StorageClass>
         <IsAccessTime>true</IsAccessTime>
         <ReturnToStdWhenVisit>false</ReturnToStdWhenVisit>
      </Transition>
    <AtimeBase>1631698332</AtimeBase>
   </Rule>
   <Rule>
      <ID>atime transition2</ID>
      <Prefix>logs2/</Prefix>
      <Status>Enabled</Status>
      <NoncurrentVersionTransition>
         <NoncurrentDays>10</NoncurrentDays>
         <StorageClass>IA</StorageClass>
         <IsAccessTime>true</IsAccessTime>
         <ReturnToStdWhenVisit>false</ReturnToStdWhenVisit>
      </NoncurrentVersionTransition>
     <AtimeBase>1631698332</AtimeBase>
  </Rule>
</LifecycleConfiguration>"#;
		let object: LifecycleConfiguration = quick_xml::de::from_str(&xml_content).unwrap();

		let json_str = serde_json::to_string_pretty(&object).unwrap();
		println!("{}", json_str);

		let left = "atime transition2";
		let right = &object.rule[1].id;
		assert_eq!(left, right);
	}

	// xml转换
	#[test]
	// 示例1：基于最后一次修改时间策略仅执行转换文件存储类型操作
	fn lifecycle_configuration_builder_1() {
		let config = LifecycleConfigurationBuilder::new()
			.with_rule(
				RuleBuilder::new()
					.with_id("rule")
					.with_prefix("log")
					.with_status("Enabled")
					.with_transition(
						TransitionBuilder::new()
							.with_days(30)
							.with_torage_class(StorageClass::IA)
							.build(),
					)
					.build(),
			)
			.build();

		let json_str = serde_json::to_string_pretty(&config).unwrap();
		println!("{}", json_str);

		let left = "<LifecycleConfiguration><Rule><ID>rule</ID><Prefix>log</Prefix><Status>Enabled</Status><Transition><Days>30</Days><StorageClass>IA</StorageClass></Transition></Rule></LifecycleConfiguration>";
		let right = quick_xml::se::to_string(&config).unwrap();

		assert_eq!(left, right);
	}

	// xml转换
	#[test]
	// 示例2：基于最后一次修改时间策略仅执行删除文件操作
	fn lifecycle_configuration_builder_2() {
		let config = LifecycleConfigurationBuilder::new()
			.with_rule(
				RuleBuilder::new()
					.with_id("rule")
					.with_prefix("log")
					.with_status("Enabled")
					.with_expiration(ExpirationBuilder::new().with_days(90).build())
					.build(),
			)
			.build();

		let json_str = serde_json::to_string_pretty(&config).unwrap();
		println!("{}", json_str);

		let left = "<LifecycleConfiguration><Rule><ID>rule</ID><Prefix>log</Prefix><Status>Enabled</Status><Expiration><Days>90</Days></Expiration></Rule></LifecycleConfiguration>";
		let right = quick_xml::se::to_string(&config).unwrap();

		assert_eq!(left, right);
	}

	// xml转换
	#[test]
	// 示例3：基于最后一次修改时间执行转换文件存储类型以及删除操作
	fn lifecycle_configuration_builder_3() {
		let config = LifecycleConfigurationBuilder::new()
			.with_rule(
				RuleBuilder::new()
					.with_id("rule")
					.with_prefix("log")
					.with_status("Enabled")
					.with_transition(
						TransitionBuilder::new()
							.with_days(30)
							.with_torage_class(StorageClass::IA)
							.build(),
					)
					.with_transition(
						TransitionBuilder::new()
							.with_days(60)
							.with_torage_class(StorageClass::Archive)
							.build(),
					)
					.with_expiration(ExpirationBuilder::new().with_days(60).build())
					.build(),
			)
			.build();
		let json_str = serde_json::to_string_pretty(&config).unwrap();
		println!("{}", json_str);

		let left = "<LifecycleConfiguration><Rule><ID>rule</ID><Prefix>log</Prefix><Status>Enabled</Status><Transition><Days>30</Days><StorageClass>IA</StorageClass></Transition><Transition><Days>60</Days><StorageClass>Archive</StorageClass></Transition><Expiration><Days>60</Days></Expiration></Rule></LifecycleConfiguration>";
		let right = quick_xml::se::to_string(&config).unwrap();

		assert_eq!(left, right);
	}

	#[test]
	// 示例4：基于最后一次修改时间执行删除历史版本文件及清理删除标记的操作
	fn lifecycle_configuration_builder_4() {
		let config = LifecycleConfigurationBuilder::new()
			.with_rule(
				RuleBuilder::new()
					.with_id("rule")
					.with_prefix("")
					.with_status("Enabled")
					.with_expiration(
						ExpirationBuilder::new()
							.with_expired_object_delete_marker(true)
							.build(),
					)
					.with_noncurrent_version_expiration(5)
					.build(),
			)
			.build();

		let json_str = serde_json::to_string_pretty(&config).unwrap();
		println!("{}", json_str);

		let left = "<LifecycleConfiguration><Rule><ID>rule</ID><Prefix/><Status>Enabled</Status><Expiration><ExpiredObjectDeleteMarker>true</ExpiredObjectDeleteMarker></Expiration><NoncurrentVersionExpiration><NoncurrentDays>5</NoncurrentDays></NoncurrentVersionExpiration></Rule></LifecycleConfiguration>";
		let right = quick_xml::se::to_string(&config).unwrap();

		assert_eq!(left, right);
	}

	#[test]
	// 示例5：基于最后一次修改时间策略限制除指定前缀、标签以外的文件执行转换存储类型及删除操作
	fn lifecycle_configuration_builder_5() {
		let config = LifecycleConfigurationBuilder::new()
			.with_rule(
				RuleBuilder::new()
					.with_id("rule")
					.with_prefix("log")
					.with_status("Enabled")
					.with_transition(
						TransitionBuilder::new()
							.with_days(30)
							.with_torage_class(StorageClass::IA)
							.with_is_access_time(true)
							.with_return_to_std_when_visit(true)
							.build(),
					)
					.build(),
			)
			.build();

		let json_str = serde_json::to_string_pretty(&config).unwrap();
		println!("{}", json_str);

		let left = "<LifecycleConfiguration><Rule><ID>rule</ID><Prefix>log</Prefix><Status>Enabled</Status><Transition><Days>30</Days><StorageClass>IA</StorageClass><IsAccessTime>true</IsAccessTime><ReturnToStdWhenVisit>true</ReturnToStdWhenVisit></Transition></Rule></LifecycleConfiguration>";
		let right = quick_xml::se::to_string(&config).unwrap();

		assert_eq!(left, right);
	}
	#[test]
	// 示例6：基于最后一次访问时间策略转换文件存储类型
	fn lifecycle_configuration_builder_6() {
		let config = LifecycleConfigurationBuilder::new()
			.with_rule(
				RuleBuilder::new()
					.with_id("rule")
					.with_prefix("/")
					.with_status("Enabled")
					.with_abort_multipart_upload(30)
					.build(),
			)
			.build();
		let json_str = serde_json::to_string_pretty(&config).unwrap();
		println!("{}", json_str);

		let left = "<LifecycleConfiguration><Rule><ID>rule</ID><Prefix>/</Prefix><Status>Enabled</Status><AbortMultipartUpload><Days>30</Days></AbortMultipartUpload></Rule></LifecycleConfiguration>";
		let right = quick_xml::se::to_string(&config).unwrap();

		assert_eq!(left, right);
	}

	#[test]
	// 示例7：基于最后一次修改时间执行删除碎片操作
	fn lifecycle_configuration_builder_7() {
		let config = LifecycleConfigurationBuilder::new()
			.with_rule(
				RuleBuilder::new()
					.with_id("rule")
					.with_prefix("/")
					.with_status("Enabled")
					.with_abort_multipart_upload(30)
					.build(),
			)
			.build();
		let json_str = serde_json::to_string_pretty(&config).unwrap();
		println!("{}", json_str);

		let left = "<LifecycleConfiguration><Rule><ID>rule</ID><Prefix>/</Prefix><Status>Enabled</Status><AbortMultipartUpload><Days>30</Days></AbortMultipartUpload></Rule></LifecycleConfiguration>";
		let right = quick_xml::se::to_string(&config).unwrap();
		assert_eq!(left, right);
	}

	#[test]
	// 示例8：基于最后一次修改时间对重叠前缀的Object执行删除操作
	fn lifecycle_configuration_builder_8() {
		let config = LifecycleConfigurationBuilder::new()
			.with_rule(
				RuleBuilder::new()
					.with_id("Rule1")
					.with_prefix("dir1")
					.with_expiration(ExpirationBuilder::new().with_days(180).build())
					.build(),
			)
			.with_rule(
				RuleBuilder::new()
					.with_id("Rule2")
					.with_prefix("dir1/dir2")
					.with_status("Status")
					.with_expiration(ExpirationBuilder::new().with_days(30).build())
					.build(),
			)
			.build();
		let json_str = serde_json::to_string_pretty(&config).unwrap();
		println!("{}", json_str);

		let left = "<LifecycleConfiguration><Rule><ID>Rule1</ID><Prefix>dir1</Prefix><Status/><Expiration><Days>180</Days></Expiration></Rule><Rule><ID>Rule2</ID><Prefix>dir1/dir2</Prefix><Status>Status</Status><Expiration><Days>30</Days></Expiration></Rule></LifecycleConfiguration>";
		let right = quick_xml::se::to_string(&config).unwrap();
		assert_eq!(left, right);
	}
}
