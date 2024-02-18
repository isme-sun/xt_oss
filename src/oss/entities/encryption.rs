use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
pub enum SSEAlgorithm {
  KMS,
  #[default]
  AES256,
  SM4,
}

impl fmt::Display for SSEAlgorithm {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let value = match self {
      Self::KMS => "KMS",
      Self::AES256 => "AES256",
      Self::SM4 => "SM4",
    };
    write!(f, "{}", value)
  }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApplyServerSideEncryptionByDefault {
  #[serde(rename = "SSEAlgorithm")]
  pub sse_algorithm: SSEAlgorithm,
  #[serde(rename = "KMSDataEncryption", skip_serializing_if = "Option::is_none")]
  pub kms_data_encryption: Option<String>,
  #[serde(rename = "KMSMasterKeyID")]
  pub kms_master_key_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServerSideEncryptionRule {
  #[serde(rename = "ApplyServerSideEncryptionByDefault")]
  pub(crate) apply_server_side_encryption_by_default: ApplyServerSideEncryptionByDefault,
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn server_side_encryption_rule1() {
    let xml_conrtent = r#"<ServerSideEncryptionRule><ApplyServerSideEncryptionByDefault> <SSEAlgorithm>KMS</SSEAlgorithm><KMSDataEncryption>SM4</KMSDataEncryption> <KMSMasterKeyID></KMSMasterKeyID></ApplyServerSideEncryptionByDefault></ServerSideEncryptionRule>"#;

    let object: ServerSideEncryptionRule = quick_xml::de::from_str(xml_conrtent).unwrap();
    assert_eq!(
      object.apply_server_side_encryption_by_default.kms_data_encryption,
      Some("SM4".to_string())
    )
  }

  #[test]
  fn server_side_encryption_rule2() {
    let object = ServerSideEncryptionRule {
      apply_server_side_encryption_by_default: ApplyServerSideEncryptionByDefault {
        sse_algorithm: super::SSEAlgorithm::KMS,
        kms_data_encryption: Some("9468da86-3509-4f8d-a61e-6eab1eac****".to_string()),
        kms_master_key_id: None,
      },
    };
    let left = r#"<ServerSideEncryptionRule><ApplyServerSideEncryptionByDefault><SSEAlgorithm>KMS</SSEAlgorithm><KMSDataEncryption>9468da86-3509-4f8d-a61e-6eab1eac****</KMSDataEncryption><KMSMasterKeyID/></ApplyServerSideEncryptionByDefault></ServerSideEncryptionRule>"#;

    let right = quick_xml::se::to_string(&object).unwrap();
    assert_eq!(left, right)
  }

  #[test]
  fn server_side_encryption_rule3() {
    let object = ServerSideEncryptionRule {
      apply_server_side_encryption_by_default: ApplyServerSideEncryptionByDefault {
        sse_algorithm: super::SSEAlgorithm::SM4,
        kms_data_encryption: None,
        kms_master_key_id: None,
      },
    };
    let left = r#"<ServerSideEncryptionRule><ApplyServerSideEncryptionByDefault><SSEAlgorithm>SM4</SSEAlgorithm><KMSMasterKeyID/></ApplyServerSideEncryptionByDefault></ServerSideEncryptionRule>"#;

    let right = quick_xml::se::to_string(&object).unwrap();
    assert_eq!(left, right)
  }

  #[test]
  fn server_side_encryption_rule4() {
    let object = ServerSideEncryptionRule {
      apply_server_side_encryption_by_default: ApplyServerSideEncryptionByDefault {
        sse_algorithm: super::SSEAlgorithm::KMS,
        kms_data_encryption: Some("SM4".to_string()),
        kms_master_key_id: None,
      },
    };
    let left = r#"<ServerSideEncryptionRule><ApplyServerSideEncryptionByDefault><SSEAlgorithm>KMS</SSEAlgorithm><KMSDataEncryption>SM4</KMSDataEncryption><KMSMasterKeyID/></ApplyServerSideEncryptionByDefault></ServerSideEncryptionRule>"#;
    let right = quick_xml::se::to_string(&object).unwrap();
    assert_eq!(left, right);
  }
}
