mod meta;
mod styles;
mod table;
mod text;

extern crate quick_xml;
extern crate serde_json;
extern crate zip;

use self::styles::*;
use self::table::*;
use self::text::*;
use crate::document::meta::Meta;
use crate::document::node::{ChildNode, Element, ListBullet, ListBulletVariant};
use crate::document::styles::{Style, Styles};
use crate::document::Document;
use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::io::BufReader;

enum MetaType {
    Title,
    Author,
    CreatedAt,
    UpdatedAt,
    EditDuration,
    Custom(String),
    Unknown,
}

pub struct ODTParser {
    body_begin: bool,
    styles_begin: bool,
    table_column_number: Vec<u32>,
    table_row_number: Vec<u32>,
    auto_styles: HashMap<String, HashMap<String, String>>,
    auto_list_styles: HashMap<String, Vec<ListBullet>>,
    set_children_underline: Vec<bool>,
    ensure_children_no_underline: Vec<bool>,
    document_root: Document,
    // document_hierarchy has Elements instead of ChildNodes, since Nodes can never have children
    document_hierarchy: Vec<Element>,
    table_column_default_style_names: Vec<Vec<String>>,
    table_row_default_style_names: Vec<Vec<String>>,
    in_list_style: bool,
    list_depth: u32,
    loaded_page_style: bool,
}

impl ODTParser {
    /// Initialises a new ODTParser instance
    pub fn new() -> ODTParser {
        let document_root = Document {
            content: Vec::new(),
            styles: Styles::new(),
            meta: None,
        };
        ODTParser {
            body_begin: false,
            styles_begin: false,
            table_column_number: Vec::new(),
            table_row_number: Vec::new(),
            auto_styles: HashMap::new(),
            auto_list_styles: HashMap::new(),
            set_children_underline: Vec::new(),
            ensure_children_no_underline: Vec::new(),
            document_root,
            document_hierarchy: Vec::new(),
            table_column_default_style_names: Vec::new(),
            table_row_default_style_names: Vec::new(),
            in_list_style: false,
            list_depth: 0,
            loaded_page_style: false,
        }
    }

    /// Parse the ODT file referenced by the file path
    pub fn parse(&mut self, filepath: &str) -> Result<String, String> {
        let archive = super::util::get_archive(filepath);
        // need to destructure and recreate the Err enum because the Result type is different
        if let Err(e) = archive {
            return Err(e);
        }
        let mut archive = archive.unwrap();
        if let Err(e) = self.parse_content(&mut archive) {
            return Err(format!("{}: {}", "Content parsing error", e));
        } else if let Err(e) = self.parse_styles(&mut archive) {
            return Err(format!("{}: {}", "Styles parsing error", e));
        } else if let Err(e) = self.parse_meta(&mut archive) {
            return Err(format!("{}: {}", "Meta parsing error", e));
        } else {
            return Ok(self.document_root.to_json().unwrap());
        }
    }

    /// Parse content.xml inside the ODT
    fn parse_content(
        &mut self,
        archive: &mut zip::ZipArchive<std::fs::File>,
    ) -> Result<(), String> {
        // returns a ZipFile struct which implements Read if the file is in the archive
        let content_xml = archive.by_name("content.xml");
        if let Err(e) = content_xml {
            // Handle case where there is no content.xml (so probably not actually an ODT file)
            return Err(e.to_string());
        }
        let content_xml = BufReader::new(content_xml.unwrap()); // add buffering because quick-xml's reader requires it

        // These are here instead of the struct because we may need to move the contents of these somewhere else
        let mut current_style_name = String::new();
        let mut current_style_value: HashMap<String, String> = HashMap::new();
        let mut current_list_style_value: Vec<ListBullet> = Vec::with_capacity(10); // This stores the list style information per level

        let default_bullet = ListBulletVariant::new(None, None, None, "filledBullet".to_string());
        let default_bullet = ListBullet::Variant(default_bullet);
        current_list_style_value.resize(10, default_bullet.clone());

        let mut parser = Reader::from_reader(content_xml);
        let mut buffer = Vec::new();
        loop {
            // Iterate through the XML
            match parser.read_event(&mut buffer) {
                Ok(Event::Start(contents)) => {
                    let (current_style_name_new, current_style_value_new, list_bullet_info) = self
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
                    if let Some((level, bullet)) = list_bullet_info {
                        if (1..11).contains(&level) {
                            // 1-10 inclusive, probably won't be more than this
                            current_list_style_value[(level - 1) as usize] = bullet;
                        }
                    }
                }
                Ok(Event::Text(contents)) => {
                    let contents = contents.unescape_and_decode(&parser);
                    if let Err(e) = contents {
                        println!("Content parsing error: {}", e);
                    } else {
                        self.handle_characters(contents.unwrap());
                    }
                }
                Ok(Event::End(contents)) => {
                    let (
                        current_style_name_new,
                        current_style_value_new,
                        current_list_style_value_new,
                    ) = self.handle_element_end(
                        std::str::from_utf8(contents.name()).unwrap_or(":"),
                        current_style_name,
                        current_style_value,
                        current_list_style_value,
                    );
                    // The 3 if lets below will restore the old values if they were not used,
                    // otherwise they will be reinitialised
                    if let Some(x) = current_style_name_new {
                        current_style_name = x;
                    } else {
                        current_style_name = String::new();
                    }
                    if let Some(x) = current_style_value_new {
                        current_style_value = x;
                    } else {
                        current_style_value = HashMap::new();
                    }
                    if let Some(x) = current_list_style_value_new {
                        current_list_style_value = x;
                    } else {
                        current_list_style_value = Vec::with_capacity(10);
                        current_list_style_value.resize(10, default_bullet.clone());
                    }
                }
                Ok(Event::Empty(contents)) => {
                    self.handle_element_empty(
                        std::str::from_utf8(contents.name()).unwrap_or(":"),
                        contents.attributes(),
                        &mut current_style_value,
                        &mut current_list_style_value,
                    );
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    println!("Content parsing error: {}", e);
                    return Err(e.to_string());
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Handles a StartElement event from the XML parser by taking its contents (only name and attributes needed)
    /// and returns the new values of current_style_name and current_style_value if either was set as a result
    /// as well as mutating internal state accordingly
    /// Returns style name, style contents, tuple of list bullet and level info
    /// Note: this is specifically for parsing content.xml
    fn handle_element_start(
        &mut self,
        name: &str,
        attributes: Attributes,
    ) -> (
        Option<String>,
        Option<HashMap<String, String>>,
        Option<(u32, ListBullet)>,
    ) {
        let (prefix, local_name) = name.split_at(name.find(':').unwrap_or(0));
        let local_name = &local_name[1..];
        match name {
            "office:body" => self.body_begin = true,
            _ if self.body_begin => {
                self.handle_element_start_body(prefix, local_name, attributes);
                return (None, None, None);
            }
            "office:automatic-styles" => self.styles_begin = true,
            _ if self.styles_begin && prefix == "style" => {
                return handle_element_start_style(local_name, attributes);
            }
            _ if self.styles_begin => {
                // because list styles are special snowflakes (they're prefixed by "text")
                let (style_name, style, bullet_cycle, in_list_style) =
                    handle_element_start_style_special(name, attributes);
                self.in_list_style = in_list_style;
                return (style_name, style, bullet_cycle);
            }
            _ => (),
        }
        (None, None, None)
    }

    /// Helper for handle_element_start() to handle tags when in the body
    fn handle_element_start_body(
        &mut self,
        prefix: &str,
        local_name: &str,
        attributes: Attributes,
    ) {
        match prefix {
            "text" => self.handle_element_start_text(local_name, attributes),
            "table" => self.handle_element_start_table(local_name, attributes),
            _ => (),
        }
    }

    /// Handles an EmptyElement event from the XML parser by taking its contents (only name and attributes needed)
    /// and returns the new value of current_style_value if it was set as a result
    /// as well as mutating internal state accordingly
    /// Note: this is specifically for parsing content.xml
    fn handle_element_empty(
        &mut self,
        name: &str,
        attributes: Attributes,
        style: &mut HashMap<String, String>,
        bullet_list: &mut Vec<ListBullet>,
    ) {
        let (prefix, local_name) = name.split_at(name.find(':').unwrap_or(0));
        let local_name = &local_name[1..];
        match prefix {
            "style" => {
                handle_element_empty_style(local_name, attributes, style, self.in_list_style)
            }
            "text" => self.handle_element_empty_text(local_name, attributes),
            "table" => self.handle_element_empty_table(local_name, attributes),
            _ if self.styles_begin => {
                handle_element_empty_style_special(name, attributes, bullet_list)
            }
            _ => (),
        }
    }

    /// Handles a Characters event from the XML parser by taking its contents
    /// and mutating internal state accordingly
    /// Note: this is specifically for parsing content.xml
    fn handle_characters(&mut self, contents: String) {
        // Apparently in between tags this will be called with an empty string, so ignore that
        if self.document_hierarchy.is_empty() || contents == "" {
            return;
        }
        // Currently the only type of tag expected to emit this event is the ones in the body,
        // in which case they will contain the document text
        self.document_hierarchy
            .last_mut()
            .unwrap()
            .get_common()
            .children
            .as_mut()
            .unwrap()
            .push(ChildNode::ShortHandText(contents));
    }

    /// Handles an EndElement event from the XML parser by taking its contents (the name of the element),
    /// the style name and value of the current element and mutating internal state accordingly,
    /// then it will return the current_style_name and current_style_value back if they were not used
    /// Note: this is specifically for parsing content.xml
    fn handle_element_end(
        &mut self,
        name: &str,
        current_style_name: String,
        current_style_value: HashMap<String, String>,
        current_list_style_value: Vec<ListBullet>,
    ) -> (
        Option<String>,
        Option<HashMap<String, String>>,
        Option<Vec<ListBullet>>,
    ) {
        let (prefix, local_name) = name.split_at(name.find(':').unwrap_or(0));
        let local_name = &local_name[1..];
        if self.body_begin {
            if self.document_hierarchy.is_empty() {
                // It shouldn't be empty now, if it is then this is an unmatched end tag
                return (
                    Some(current_style_name),
                    Some(current_style_value),
                    Some(current_list_style_value),
                );
            }
            if name == "office:body" {
                return (
                    Some(current_style_name),
                    Some(current_style_value),
                    Some(current_list_style_value),
                );
            } else if prefix == "text"
                && (local_name == "h"
                    || local_name == "p"
                    || local_name == "span"
                    || local_name == "a")
            {
                // The top of set_children_underline and ensure_children_no_underline is for this node's children,
                // so pop them here before we finish up with this node
                self.set_children_underline.pop();
                self.ensure_children_no_underline.pop();
                let mut child = self.document_hierarchy.pop().unwrap();
                if local_name == "span" {
                    handle_underline(
                        &mut child.get_common().styles,
                        !self.set_children_underline.is_empty()
                            && *self.set_children_underline.last().unwrap(),
                        !self.ensure_children_no_underline.is_empty()
                            && *self.ensure_children_no_underline.last().unwrap(),
                    );
                }
                let mut page_break_after = false;
                if let Some(_) = child.get_common().styles.remove("_pageBreakAfter") {
                    // Need to keep track of this in a variable since the element will be moved to
                    // the document structure
                    page_break_after = true;
                }
                if self.document_hierarchy.is_empty() {
                    self.document_root.content.push(ChildNode::Element(child));
                } else {
                    self.document_hierarchy
                        .last_mut()
                        .unwrap()
                        .get_common()
                        .children
                        .as_mut()
                        .unwrap()
                        .push(ChildNode::Element(child));
                }
                if page_break_after {
                    self.insert_break(true);
                }
            } else if prefix == "table" {
                self.handle_element_end_table(local_name);
            } else if name == "text:list" || name == "text:list-item" {
                if name == "text:list" {
                    self.list_depth -= 1;
                }
                let child = self.document_hierarchy.pop().unwrap();
                if self.document_hierarchy.is_empty() {
                    self.document_root.content.push(ChildNode::Element(child));
                } else {
                    self.document_hierarchy
                        .last_mut()
                        .unwrap()
                        .get_common()
                        .children
                        .as_mut()
                        .unwrap()
                        .push(ChildNode::Element(child));
                }
            }
        } else if self.styles_begin {
            if name == "office:automatic-styles" {
                self.styles_begin = false;
            } else if name == "style:style" {
                self.auto_styles
                    .insert(current_style_name, current_style_value);
                return (None, None, Some(current_list_style_value));
            } else if name == "text:list-style" {
                self.in_list_style = false;
                self.auto_list_styles
                    .insert(current_style_name, current_list_style_value);
                return (None, Some(current_style_value), None);
            }
        }
        (
            Some(current_style_name),
            Some(current_style_value),
            Some(current_list_style_value),
        )
    }
}
