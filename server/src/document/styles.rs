use super::node::Element;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Styles {
    pub page: HashMap<String, String>,
    pub classes: HashMap<String, Style>,
}

impl Styles {
    /// Constructs a new Styles struct
    pub fn new() -> Styles {
        Styles {
            page: HashMap::new(),
            classes: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Style {
    display: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    inherit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element: Option<Element>,
    pub styles: HashMap<String, String>,
}

impl Style {
    /// Constructs a new Style struct, which represents a style class
    ///
    /// - `display` A human-readable string which can be shown to users.
    /// - `inherit` A string containing the unique ID of another class, from which to inherit styles from.
    /// - `element` An optional Element that will be used as a template
    pub fn new(display: String, inherit: Option<String>) -> Style {
        Style {
            display,
            inherit,
            element: None,
            styles: HashMap::new(),
        }
    }
}
