use serde_derive::{Deserialize, Serialize};

use crate::Manager;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Url(String);

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Architecture {
    #[serde(rename = "64bit")]
    bit64: Url,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Checkver {
    github: String,
    regex: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ScoopJson {
    version: Option<String>,
    description: String,
    homepage: String,
    license: String,
    // architecture: Archtecture,
    bin: String,
    checkver: Checkver,
    // autoupdate: Archtecture,
}

impl From<Manager> for ScoopJson {
    fn from(manager: Manager) -> Self {
        Self {
            version: None,
            description: manager.description,
            homepage: manager.homepage.to_owned(),
            license: manager.license,
            bin: manager.name,
            checkver: Checkver {
                github: manager.homepage,
                regex: "tag/v([\\d.]+)".to_string(),
            },
        }
    }
}
