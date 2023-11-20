use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_qs as qs;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Regions {
    pub regions: Option<String>,
}

impl Regions {
    pub const ALL: Regions = Regions { regions: None };
}

#[allow(dead_code)]
impl Display for Regions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = if let Some(_) = self.regions {
            qs::to_string(&self).unwrap()
        } else {
            "regions".to_string()
        };
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::Regions;
    // use serde_qs as qs;

    #[test]
    fn params_regions() {
        println!("{}\n", "-".repeat(60));
        // let region = "oss-cn-hangzhou".to_string();
        // let params = Regions{regions: Some(region)};
        let params = Regions { regions: None };
        println!("{}", params);
        println!("{}", Regions{regions:Some("cn-zhanghao".to_string())});
        println!("\n{}", "-".repeat(60));
    }
}
