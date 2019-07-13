mod table;
mod text;

extern crate quick_xml;
extern crate serde_json;
extern crate zip;

use self::table::*;
use self::text::*;
use crate::document::node::{Element, Node, Text};
use crate::document::units::DistanceUnit;
use crate::document::{Document, PaperSize};
use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::io::BufReader;

pub struct ODTParser {
    body_begin: bool,
    styles_begin: bool,
    table_column_number: Vec<u32>,
    table_row_number: Vec<u32>,
    auto_styles: HashMap<String, HashMap<String, String>>,
    set_children_underline: Vec<bool>,
    ensure_children_no_underline: Vec<bool>,
    document_root: Document,
    document_hierarchy: Vec<Element>,
    table_column_default_style_names: Vec<Vec<String>>,
    table_row_default_style_names: Vec<Vec<String>>,
}

impl ODTParser {
    /// Initialises a new ODTParser instance
    pub fn new() -> ODTParser {
        let document_root = Document::new(
            "Kauri (Working Title)".to_string(),
            PaperSize::new(297, 210, DistanceUnit::Millimetres),
        );
        ODTParser {
            body_begin: false,
            styles_begin: false,
            table_column_number: Vec::new(),
            table_row_number: Vec::new(),
            auto_styles: HashMap::new(),
            set_children_underline: Vec::new(),
            ensure_children_no_underline: Vec::new(),
            document_root,
            document_hierarchy: Vec::new(),
            table_column_default_style_names: Vec::new(),
            table_row_default_style_names: Vec::new(),
        }
    }

    /// Parse the ODT file referenced by the file path
    pub fn parse(&mut self, filepath: &str) -> Result<String, String> {
        let archive = super::util::get_archive(filepath);
        if let Err(e) = archive {
            return Err(e.to_string());
        }
        let archive = archive.unwrap();
        self.parse_private(archive)
    }

    /// Actually parse the file, this is a separate function so we actually own the archive here
    fn parse_private(
        &mut self,
        mut archive: zip::ZipArchive<std::fs::File>,
    ) -> Result<String, String> {
        // returns a ZipFile struct which implements Read if the file is in the archive
        let content_xml = archive.by_name("content.xml");
        if let Err(e) = content_xml {
            // Handle case where there is no content.xml (so probably not actually an ODT file)
            return Err(e.to_string());
        }
        let content_xml = BufReader::new(content_xml.unwrap()); //add buffering because quick-xml's reader requires it

        // These are here instead of the struct because we may need to move the contents of these somewhere else
        let mut current_style_name = String::new();
        let mut current_style_value: HashMap<String, String> = HashMap::new();

        let mut parser = Reader::from_reader(content_xml);
        let mut buffer = Vec::new();
        loop {
            // Iterate through the XML
            match parser.read_event(&mut buffer) {
                Ok(Event::Start(contents)) => {
                    let (current_style_name_new, current_style_value_new) = self
                        .handle_element_start(
                            std::str::from_utf8(contents.name()).unwrap_or(":"),
                            contents.attributes(),
                        );
                    if let Some(x) = current_style_name_new {
                        current_style_name = x;
                    }
                    if let Some(x) = current_style_value_new {
                        current_style_value = x;
                    }
                }
                Ok(Event::Text(contents)) => {
                    let contents = contents.unescape_and_decode(&parser);
                    if let Err(e) = contents {
                        println!("Error: {}", e);
                    } else {
                        self.handle_characters(contents.unwrap());
                    }
                }
                Ok(Event::End(contents)) => {
                    let result = self.handle_element_end(
                        std::str::from_utf8(contents.name()).unwrap_or(":"),
                        current_style_name,
                        current_style_value,
                    );
                    if let Some(x) = result {
                        // If they were not used inside handle_element_end() then put them back
                        let (current_style_name_new, current_style_value_new) = x;
                        current_style_name = current_style_name_new;
                        current_style_value = current_style_value_new;
                    } else {
                        // Otherwise reinitialise them
                        current_style_name = String::new();
                        current_style_value = HashMap::new();
                    }
                }
                Ok(Event::Empty(contents)) => {
                    let current_style_value_new = self.handle_element_empty(
                        std::str::from_utf8(contents.name()).unwrap_or(":"),
                        contents.attributes(),
                    );
                    if let Some(x) = current_style_value_new {
                        current_style_value = x;
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    println!("Error: {}", e);
                    return Err(e.to_string());
                }
                _ => {}
            }
        }

        Ok(self.document_root.to_json().unwrap())
    }

    /// Handles a StartElement event from the XML parser by taking its contents (only name and attributes needed)
    /// and returns the new values of current_style_name and current_style_value if either was set as a result
    /// as well as mutating internal state accordingly
    fn handle_element_start(
        &mut self,
        name: &str,
        attributes: Attributes,
    ) -> (Option<String>, Option<HashMap<String, String>>) {
        let mut current_style_name: Option<String> = None;
        let mut current_style_value: Option<HashMap<String, String>> = None;
        let (prefix, local_name) = name.split_at(name.find(':').unwrap_or(0));
        let local_name = &local_name[1..];
        if name == "office:body" {
            self.body_begin = true;
        } else if self.body_begin {
            match prefix {
                "text" => self.handle_element_start_text(local_name, attributes),
                "table" => self.handle_element_start_table(local_name, attributes),
                _ => return (current_style_name, current_style_value),
            }
        } else if name == "office:automatic-styles" {
            self.styles_begin = true;
        } else if self.styles_begin {
            if prefix != "style" {
                return (current_style_name, current_style_value);
            }
            match local_name {
                "style" => current_style_name = Some(style_begin(attributes)),
                "table-row-properties" => {
                    current_style_value = Some(table_row_properties_begin(attributes))
                }
                "table-properties" => {
                    current_style_value = Some(table_properties_begin(attributes))
                }
                "table-cell-properties" => {
                    current_style_value = Some(table_cell_properties_begin(attributes))
                }
                _ => (),
            }
        }
        (current_style_name, current_style_value)
    }

    /// Handles an EmptyElement event from the XML parser by taking its contents (only name and attributes needed)
    /// and returns the new value of current_style_value if it was set as a result
    /// as well as mutating internal state accordingly
    fn handle_element_empty(
        &mut self,
        name: &str,
        attributes: Attributes,
    ) -> Option<HashMap<String, String>> {
        let (prefix, local_name) = name.split_at(name.find(':').unwrap_or(0));
        let local_name = &local_name[1..];
        if name == "style:text-properties" {
            Some(text_properties_begin(attributes))
        } else if name == "style:table-column-properties" {
            Some(table_column_properties_begin(attributes))
        } else if name == "table:table-column" {
            // We should be inside a table if we see this, so if it is empty ignore
            if self.document_hierarchy.is_empty()
                || self.table_column_default_style_names.is_empty()
            {
                return None;
            }
            if let Node::Element(ref mut element) =
                &mut self.document_hierarchy.last_mut().unwrap().children[1]
            {
                let (table, default_cell_style_name, mut repeat) =
                    table_column_begin(attributes, &self.auto_styles);
                element.children.push(Node::Element(table));
                let table_column_default_style_names =
                    self.table_column_default_style_names.last_mut().unwrap();
                if let Some(default_cell_style_name) = default_cell_style_name {
                    while repeat != 0 {
                        table_column_default_style_names.push(default_cell_style_name.clone());
                        repeat -= 1;
                    }
                } else {
                    while repeat != 0 {
                        table_column_default_style_names.push("".to_string());
                        repeat -= 1;
                    }
                }
            }
            None
        } else if prefix == "text" {
            self.handle_element_empty_text(local_name, attributes);
            None
        } else {
            None
        }
    }

    /// Handles a Characters event from the XML parser by taking its contents
    /// and mutating internal state accordingly
    fn handle_characters(&mut self, contents: String) {
        // Apparently in between tags this will be called with an empty string, so ignore that
        if self.document_hierarchy.is_empty() || contents == "" {
            return;
        }
        // Currently the only type of tag expected to emit this event is the ones in the body,
        // in which case they will contain the document text
        let text = Text::new(contents);
        self.document_hierarchy
            .last_mut()
            .unwrap()
            .children
            .push(Node::Text(text));
    }

    /// Handles an EndElement event from the XML parser by taking its contents (the name of the element),
    /// the style name and value of the current element and mutating internal state accordingly,
    /// then it will return the current_style_name and current_style_value back if they were not used
    fn handle_element_end(
        &mut self,
        name: &str,
        current_style_name: String,
        current_style_value: HashMap<String, String>,
    ) -> Option<(String, HashMap<String, String>)> {
        let (prefix, local_name) = name.split_at(name.find(':').unwrap_or(0));
        let local_name = &local_name[1..];
        if self.body_begin {
            if self.document_hierarchy.is_empty() {
                // It shouldn't be empty now, if it is then this is an unmatched end tag
                return Some((current_style_name, current_style_value));
            }
            if name == "office:body" {
                return Some((current_style_name, current_style_value));
            } else if prefix == "text"
                && (local_name == "h" || local_name == "p" || local_name == "span")
            {
                // The top of set_children_underline and ensure_children_no_underline is for this node's children,
                // so pop them here before we finish up with this node
                self.set_children_underline.pop();
                self.ensure_children_no_underline.pop();
                let mut child = self.document_hierarchy.pop().unwrap();
                if local_name == "span" {
                    handle_underline(
                        &mut child.styles,
                        !self.set_children_underline.is_empty()
                            && *self.set_children_underline.last().unwrap(),
                        !self.ensure_children_no_underline.is_empty()
                            && *self.ensure_children_no_underline.last().unwrap(),
                    );
                }
                if self.document_hierarchy.is_empty() {
                    self.document_root.children.push(Node::Element(child));
                } else {
                    self.document_hierarchy
                        .last_mut()
                        .unwrap()
                        .children
                        .push(Node::Element(child));
                }
            } else if prefix == "table" {
                self.handle_element_end_table(local_name);
            }
        } else if self.styles_begin {
            if name == "office:automatic-styles" {
                self.styles_begin = false;
            } else if name == "style:style" {
                self.auto_styles
                    .insert(current_style_name, current_style_value);
                return None;
            }
        }
        Some((current_style_name, current_style_value))
    }
}

/// Takes the set of attributes of a style:style tag in the ODT's content.xml,
/// and returns the name of the style
fn style_begin(attributes: Attributes) -> String {
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            if name == "style:name" {
                return std::str::from_utf8(
                    &i.unescaped_value()
                        .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                )
                .unwrap_or("")
                .to_string();
            }
        }
    }
    String::new()
}
