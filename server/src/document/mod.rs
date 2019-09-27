pub mod meta;
pub mod node;
pub mod styles;

use self::meta::Meta; //need to specify self here for some reason
use node::ChildNode;
use serde::{Deserialize, Serialize};
use styles::Styles;

#[derive(Serialize, Deserialize)]
pub struct Document {
    pub content: Vec<ChildNode>,
    pub styles: Styles,
    // Meta here is an Option because we can only initialise it when we have actually read the metadata
    pub meta: Option<Meta>,
}

impl Document {
    /// Converts the document to a JSON string (pretty print in debug mode)
    #[cfg(debug_assertions)]
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    #[cfg(not(debug_assertions))]
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}
