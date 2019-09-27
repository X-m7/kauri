use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    title: String,
    authors: Vec<String>,
    created_at: String,
    updated_at: String,
    edit_duration: String,
    #[serde(flatten)]
    pub additional: HashMap<String, String>,
}

impl Meta {
    /// Constructs a new Meta struct
    ///
    /// - `title` A human-readable string which can be shown to both document authors and readers.
    /// - `authors` An array of strings, listing the names of each person that contributed.
    /// - `created_at` Time of document creation.
    /// - `updated_at` Time that the document was last written to.
    /// - `edit_duration` The total amount of time spent editing the document.
    pub fn new(
        title: String,
        authors: Vec<String>,
        created_at: String,
        updated_at: String,
        edit_duration: String,
    ) -> Meta {
        Meta {
            title,
            authors,
            created_at,
            updated_at,
            edit_duration,
            additional: HashMap::new(),
        }
    }
}
