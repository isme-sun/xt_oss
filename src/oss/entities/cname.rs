use serde::{Deserialize, Serialize};

pub mod builders {

    use super::BucketCnameConfiguration;

    #[derive(Debug, Default, Clone)]
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

        pub fn with_cert_id(mut self, value: &str) -> Self {
            self.bucket_cname_configuration
                .cname
                .certificate_configuration
                .get_or_insert_with(super::CertificateConfiguration::default)
                .cert_id = Some(value.to_string());
            self
        }

        pub fn with_certificate(mut self, value: &str) -> Self {
            self.bucket_cname_configuration
                .cname
                .certificate_configuration
                .get_or_insert_with(super::CertificateConfiguration::default)
                .certificate = Some(value.to_string());
            self
        }
        pub fn with_private_key(mut self, value: &str) -> Self {
            self.bucket_cname_configuration
                .cname
                .certificate_configuration
                .get_or_insert_with(super::CertificateConfiguration::default)
                .private_key = Some(value.to_string());
            self
        }

        /*
           <Force>true</Force>
        */
        pub fn with_previous_cert_id(mut self, value: &str) -> Self {
            self.bucket_cname_configuration
                .cname
                .certificate_configuration
                .get_or_insert_with(super::CertificateConfiguration::default)
                .previous_cert_id = Some(value.to_string());
            self
        }

        pub fn with_force(mut self, value: bool) -> Self {
            self.bucket_cname_configuration
                .cname
                .certificate_configuration
                .get_or_insert_with(super::CertificateConfiguration::default)
                .force = Some(value);
            self
        }

        pub fn with_delete_certificate(mut self, value: bool) -> Self {
            self.bucket_cname_configuration
                .cname
                .certificate_configuration
                .get_or_insert_with(super::CertificateConfiguration::default)
                .delete_certificate = Some(value);
            self
        }

        pub fn build(&self) -> BucketCnameConfiguration {
            self.bucket_cname_configuration.clone()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ListCnameResult {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Owner")]
    pub owner: String,
    #[serde(rename = "Cname", skip_serializing_if = "Option::is_none")]
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
    #[serde(rename = "LastModified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
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
    #[serde(rename = "CertId", skip_serializing_if = "Option::is_none")]
    pub cert_id: Option<String>,
    #[serde(rename = "Certificate", skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "PrivateKey", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(rename = "PreviousCertId", skip_serializing_if = "Option::is_none")]
    pub previous_cert_id: Option<String>,
    #[serde(rename = "Force", skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "DeleteCertificate", skip_serializing_if = "Option::is_none")]
    pub delete_certificate: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::builders::BucketCnameConfigurationBuilder;
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
        let obj = quick_xml::de::from_str::<CnameToken>(xml).unwrap();
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
        let obj: ListCnameResult = quick_xml::de::from_str(xml).unwrap();

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
        let obj: ListCnameResult = quick_xml::de::from_str(xml).unwrap();

        let left = "dev-cdn.xuetube.com";
        let right = &obj.cname.unwrap()[0].domain;
        assert_eq!(&left, &right);
    }

    #[test]
    fn bucket_cname_configuration_builder_1() {
        let config = BucketCnameConfigurationBuilder::new()
            .with_domain("example.com")
            .with_cert_id("493****-cn-hangzhou")
            .with_certificate("-----BEGIN CERTIFICATE----- MIIDhDCCAmwCCQCFs8ixARsyrDANBgkqhkiG9w0BAQsFADCBgzELMAkGA1UEBhMC **** -----END CERTIFICATE-----")
            .with_private_key("-----BEGIN CERTIFICATE----- MIIDhDCCAmwCCQCFs8ixARsyrDANBgkqhkiG9w0BAQsFADCBgzELMAkGA1UEBhMC **** -----END CERTIFICATE-----<")
            .with_previous_cert_id("493****-cn-hangzhou")
            .with_force(true)
            .build();
        let left = r###"<BucketCnameConfiguration><Cname><Domain>example.com</Domain><CertificateConfiguration><CertId>493****-cn-hangzhou</CertId><Certificate>-----BEGIN CERTIFICATE----- MIIDhDCCAmwCCQCFs8ixARsyrDANBgkqhkiG9w0BAQsFADCBgzELMAkGA1UEBhMC **** -----END CERTIFICATE-----</Certificate><PrivateKey>-----BEGIN CERTIFICATE----- MIIDhDCCAmwCCQCFs8ixARsyrDANBgkqhkiG9w0BAQsFADCBgzELMAkGA1UEBhMC **** -----END CERTIFICATE-----&lt;</PrivateKey><PreviousCertId>493****-cn-hangzhou</PreviousCertId><Force>true</Force></CertificateConfiguration></Cname></BucketCnameConfiguration>"###;
        let right = quick_xml::se::to_string(&config).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn bucket_cname_configuration_builder_2() {
        let config = BucketCnameConfigurationBuilder::new()
            .with_delete_certificate(true)
            .build();
        let left = r#"<BucketCnameConfiguration><Cname><Domain/><CertificateConfiguration><DeleteCertificate>true</DeleteCertificate></CertificateConfiguration></Cname></BucketCnameConfiguration>"#;
        let right = quick_xml::se::to_string(&config).unwrap();
        assert_eq!(left, right);
    }
}
