use std::fmt;

use mongodb::bson::oid::ObjectId;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Domain {
    pub orig_url: String,
    pub short_name: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct Url {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub orig_url: String,
    pub short_url: Option<String>,
    url_id: String,
}
impl Url {
    pub fn new(input_id: Option<&str>, id: Option<ObjectId>, orig_url: &str) -> Self {
        let default_id = nanoid!(10);
        log::info!("Input_id: {:?}", &input_id);
        let url_id = match input_id {
            Some(id) => id.to_string(),
            None => {
                log::warn!("Input ID is None, using default_id: {}", default_id);
                default_id
            }
        };

        let short_url = Some(format!("http://zip.ly:8080/{}", url_id));
        log::info!("short_url: {:?}", &short_url);
        Self {
            id,
            orig_url: orig_url.to_owned(),
            short_url,
            url_id,
        }
    }
}
impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement the formatting logic here
        write!(
            f,
            "Url {{ orig_url: {:?}, short_url: {:?}, id: {:?} }}",
            self.orig_url, self.short_url, self.url_id
        )
    }
}

impl fmt::Debug for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement the formatting logic for individual fields
        write!(
            f,
            "Url {{ orig_url: {:?}, short_url: {:?}, id: {:?} }}",
            self.orig_url, self.short_url, self.url_id
        )
    }
}
