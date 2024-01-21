use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ListCnameResult {
	#[serde(rename = "Bucket")]
	pub bucket: String,
	#[serde(rename = "Owner")]
	pub owner: String,
	#[serde(rename = "Cname")]
	pub cname: Option<Vec<Cname>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Certificate {
	#[serde(rename = "Type")]
	pub r#type: String,
	#[serde(rename = "CertId")]
	pub cert_id: String,
	#[serde(rename = "Status")]
	pub status: String,
	#[serde(rename = "CreationDate")]
	pub creation_date: String,
	#[serde(rename = "Fingerprint")]
	pub fingerprint: String,
	#[serde(rename = "ValidStartDate")]
	pub valid_start_date: String,
	#[serde(rename = "ValidEndDate")]
	pub valid_end_date: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Cname {
	#[serde(rename = "Domain")]
	pub domain: String,
	#[serde(
		rename = "LastModified",
		with = "super::private::serde_date::utc_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub last_modified: Option<DateTime<Utc>>,
	#[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
	pub status: Option<String>,
	#[serde(rename = "IsPurgeCdnCache", skip_serializing_if = "Option::is_none")]
	pub is_purge_cdn_cache: Option<bool>,
	#[serde(rename = "Certificate", skip_serializing_if = "Option::is_none")]
	pub certificate: Option<Certificate>,
	#[serde(
		rename = "CertificateConfiguration",
		skip_serializing_if = "Option::is_none"
	)]
	pub certificate_configuration: Option<CertificateConfiguration>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CnameToken {
	#[serde(rename = "Bucket")]
	pub bucket: String,
	#[serde(rename = "Cname")]
	pub cname: String,
	#[serde(rename = "Token")]
	pub token: String,
	#[serde(rename = "ExpireTime")]
	pub expire_time: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BucketCnameConfiguration {
	#[serde(rename = "Cname")]
	pub cname: Cname,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CertificateConfiguration {
	#[serde(rename = "CertId")]
	pub cert_id: String,
	#[serde(rename = "Certificate")]
	pub certificate: String,
	#[serde(rename = "PrivateKey")]
	pub private_key: String,
	#[serde(rename = "PreviousCertId")]
	pub previous_cert_id: String,
	#[serde(rename = "Force")]
	pub force: bool,
	#[serde(rename = "DeleteCertificate")]
	pub delete_certificate: bool,
}

pub mod builder {

	use super::BucketCnameConfiguration;

	#[derive(Debug, Default)]
	pub struct BucketCnameConfigurationBuilder {
		pub bucket_cname_configuration: BucketCnameConfiguration,
	}

	impl BucketCnameConfigurationBuilder {
		pub fn new() -> Self {
			BucketCnameConfigurationBuilder {
				bucket_cname_configuration: BucketCnameConfiguration::default(),
			}
		}

		pub fn with_domain(mut self, value: &str) -> Self {
			self.bucket_cname_configuration.cname.domain = value.to_string();
			self
		}

		// pub fn with_cert_id(mut self, value: &str) -> Self {
		//     let certificate =
		//         if let Some(mut certificate) = self.bucket_cname_configuration.cname.certificate {
		//             certificate.cert_id = value.to_string();
		//             certificate
		//         } else {
		//             let mut certificate = super::Certificate::default();
		//             certificate.cert_id = value.to_string();
		//             certificate
		//         };
		//     self.bucket_cname_configuration.cname.certificate = Some(certificate);
		//     self
		// }

		pub fn config(&self) -> String {
			quick_xml::se::to_string(&self.bucket_cname_configuration).unwrap()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::builder::BucketCnameConfigurationBuilder;
	use crate::oss::entities::cname::{CnameToken, ListCnameResult};

	#[test]
	fn cname_token() {
		let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<CnameToken>
	<Bucket>examplebucket</Bucket>
	<Cname>example.com</Cname>;
	<Token>be1d49d863dea9ffeff3df7d6455****</Token>
	<ExpireTime>Wed, 23 Feb 2022 21:16:37 GMT</ExpireTime>
</CnameToken>"#;
		let obj = quick_xml::de::from_str::<CnameToken>(&xml).unwrap();
		let left = "Wed, 23 Feb 2022 21:16:37 GMT";
		let right = obj.expire_time;
		assert_eq!(left, right);
	}

	#[test]
	fn list_cname_result() {
		let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListCnameResult>
	<Bucket>targetbucket</Bucket>
	<Owner>testowner</Owner>
	<Cname>
		<Domain>example.com</Domain>
		<LastModified>2021-09-15T02:35:07.000Z</LastModified>
		<Status>Enabled</Status>
		<Certificate>
			<Type>CAS</Type>
			<CertId>493****-cn-hangzhou</CertId>
			<Status>Enabled</Status>
			<CreationDate>Wed, 15 Sep 2021 02:35:06 GMT</CreationDate>
			<Fingerprint>DE:01:CF:EC:7C:A7:98:CB:D8:6E:FB:1D:97:EB:A9:64:1D:4E:**:**</Fingerprint>
			<ValidStartDate>Wed, 12 Apr 2023 10:14:51 GMT</ValidStartDate>
			<ValidEndDate>Mon, 4 May 2048 10:14:51 GMT</ValidEndDate>
		</Certificate>
	</Cname>
	<Cname>
		<Domain>example.org</Domain>
		<LastModified>2021-09-15T02:34:58.000Z</LastModified>
		<Status>Enabled</Status>
	</Cname>
	<Cname>
		<Domain>example.edu</Domain>
		<LastModified>2021-09-15T02:50:34.000Z</LastModified>
		<Status>Enabled</Status>
	</Cname>
</ListCnameResult>"#;
		let obj: ListCnameResult = quick_xml::de::from_str(&xml).unwrap();

		println!("{:#?}", obj);

		let cname = obj.cname.unwrap()[0].clone();
		let cert = cname.certificate.unwrap();
		assert_eq!("CAS", cert.r#type);
	}

	#[test]
	fn list_cname_result2() {
		let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
			<ListCnameResult>
			<Bucket>xuetube-dev</Bucket>
			<Owner>1508492296054765</Owner>
			<Cname>
				<Domain>dev-cdn.xuetube.com</Domain>
				<LastModified>2023-06-29T02:49:16.000Z</LastModified>
				<Status>Enabled</Status>
				<IsPurgeCdnCache>false</IsPurgeCdnCache>
				<Certificate>
					<Type>CAS</Type>
					<CertId>10542783-cn-hangzhou</CertId>
					<Status>Enabled</Status>
					<CreationDate>Thu, 29 Jun 2023 02:49:14 GMT</CreationDate>
					<Fingerprint>AD:34:E3:12:D2:4F:46:23:9C:92:A6:7C:16:59:AE:AD:27:1F:29:C7</Fingerprint>
					<ValidStartDate>Thu, 29 Jun 2023 02:49:14 GMT</ValidStartDate>
					<ValidEndDate>Thu, 29 Jun 2023 02:49:14 GMT</ValidEndDate>
				</Certificate>
			</Cname>
		</ListCnameResult>"#;
		let obj: ListCnameResult = quick_xml::de::from_str(&xml).unwrap();

		println!("{:#?}", obj);
	}

	#[test]
	fn bucket_cname_configuration_builder() {
		let builder =
			BucketCnameConfigurationBuilder::default().with_domain("https://dev.xuetube.com");

		print!("{}", builder.config());
	}
}
