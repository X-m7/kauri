use super::*;
use crate::document::node::{ElementCommon, Heading, Hyperlink, List, ListItem, Node};

impl ODTParser {
    /// Helper for handle_element_start() to respond to tags with "text" prefix
    pub fn handle_element_start_text(&mut self, local_name: &str, attributes: Attributes) {
        match local_name {
            "h" => {
                let (mut element, set_children_underline_new, ensure_children_no_underline_new) =
                    check_underline(
                        heading_begin(attributes, &mut self.auto_styles),
                        &self.auto_styles,
                        !self.set_children_underline.is_empty()
                            && *self.set_children_underline.last().unwrap(),
                        !self.ensure_children_no_underline.is_empty()
                            && *self.ensure_children_no_underline.last().unwrap(),
                    );
                self.ensure_children_no_underline
                    .push(ensure_children_no_underline_new);
                self.set_children_underline.push(set_children_underline_new);
                if let Some(_) = element.get_common().styles.remove("_pageBreakBefore") {
                    self.insert_break(true);
                }
                self.document_hierarchy.push(element);
            }
            "p" => {
                let (mut element, set_children_underline_new, ensure_children_no_underline_new) =
                    check_underline(
                        paragraph_begin(attributes, &mut self.auto_styles),
                        &self.auto_styles,
                        !self.set_children_underline.is_empty()
                            && *self.set_children_underline.last().unwrap(),
                        !self.ensure_children_no_underline.is_empty()
                            && *self.ensure_children_no_underline.last().unwrap(),
                    );
                self.ensure_children_no_underline
                    .push(ensure_children_no_underline_new);
                self.set_children_underline.push(set_children_underline_new);
                if let Some(_) = element.get_common().styles.remove("_pageBreakBefore") {
                    self.insert_break(true);
                }
                self.document_hierarchy.push(element);
            }
            "span" => {
                let (element, set_children_underline_new, ensure_children_no_underline_new) =
                    check_underline(
                        span_begin(attributes, &mut self.auto_styles),
                        &self.auto_styles,
                        !self.set_children_underline.is_empty()
                            && *self.set_children_underline.last().unwrap(),
                        !self.ensure_children_no_underline.is_empty()
                            && *self.ensure_children_no_underline.last().unwrap(),
                    );
                self.ensure_children_no_underline
                    .push(ensure_children_no_underline_new);
                self.set_children_underline.push(set_children_underline_new);
                self.document_hierarchy.push(element);
            }
            "a" => {
                let (element, set_children_underline_new, ensure_children_no_underline_new) =
                    check_underline(
                        a_begin(attributes, &mut self.auto_styles),
                        &self.auto_styles,
                        !self.set_children_underline.is_empty()
                            && *self.set_children_underline.last().unwrap(),
                        !self.ensure_children_no_underline.is_empty()
                            && *self.ensure_children_no_underline.last().unwrap(),
                    );
                self.ensure_children_no_underline
                    .push(ensure_children_no_underline_new);
                self.set_children_underline.push(set_children_underline_new);
                self.document_hierarchy.push(element);
            }
            "list" => {
                let style_name = list_begin(attributes);
                let element;
                if let Some(x) = style_name {
                    if let Some(x) = self.auto_list_styles.get(&x) {
                        // if the referenced style is an automatic one just copy it into the list itself
                        element = List::new(None, Some(x.clone()), None);
                    } else {
                        // else assume it is a named style
                        element = List::new(Some(x), None, None);
                    }
                } else {
                    element = List::new(None, None, None);
                }

                self.list_depth += 1;
                self.document_hierarchy.push(Element::List(element));
            }
            "list-item" => {
                let element = self.handle_list_item_start(list_item_begin(attributes));
                self.document_hierarchy.push(Element::ListItem(element));
            }
            _ => (),
        }
    }

    /// Helper for handle_element_empty() to respond to tags with "text" prefix
    pub fn handle_element_empty_text(&mut self, local_name: &str, attributes: Attributes) {
        let mut child: Option<Element> = None;
        match local_name {
            "h" => {
                let (mut element, ..) = check_underline(
                    heading_begin(attributes, &mut self.auto_styles),
                    &self.auto_styles,
                    !self.set_children_underline.is_empty()
                        && *self.set_children_underline.last().unwrap(),
                    !self.ensure_children_no_underline.is_empty()
                        && *self.ensure_children_no_underline.last().unwrap(),
                );
                if let Some(_) = element.get_common().styles.remove("_pageBreakBefore") {
                    self.insert_break(true);
                }
                child = Some(element);
            }
            "p" => {
                let (mut element, ..) = check_underline(
                    paragraph_begin(attributes, &mut self.auto_styles),
                    &self.auto_styles,
                    !self.set_children_underline.is_empty()
                        && *self.set_children_underline.last().unwrap(),
                    !self.ensure_children_no_underline.is_empty()
                        && *self.ensure_children_no_underline.last().unwrap(),
                );
                if let Some(_) = element.get_common().styles.remove("_pageBreakBefore") {
                    self.insert_break(true);
                }
                child = Some(element);
            }
            "span" => {
                let (element, ..) = check_underline(
                    span_begin(attributes, &mut self.auto_styles),
                    &self.auto_styles,
                    !self.set_children_underline.is_empty()
                        && *self.set_children_underline.last().unwrap(),
                    !self.ensure_children_no_underline.is_empty()
                        && *self.ensure_children_no_underline.last().unwrap(),
                );
                child = Some(element);
            }
            "a" => {
                let (element, ..) = check_underline(
                    a_begin(attributes, &mut self.auto_styles),
                    &self.auto_styles,
                    !self.set_children_underline.is_empty()
                        && *self.set_children_underline.last().unwrap(),
                    !self.ensure_children_no_underline.is_empty()
                        && *self.ensure_children_no_underline.last().unwrap(),
                );
                child = Some(element);
            }
            "soft-page-break" => self.insert_break(true),
            "line-break" => self.insert_break(false),
            _ => (),
        }
        if let Some(element) = child {
            if self.document_hierarchy.is_empty() {
                self.document_root.content.push(ChildNode::Element(element));
            } else {
                self.document_hierarchy
                    .last_mut()
                    .unwrap()
                    .get_common()
                    .children
                    .as_mut()
                    .unwrap()
                    .push(ChildNode::Element(element));
            }
        }
    }

    /// Takes the override style name of the list item and returns a ListItem with the appropriate
    /// bullet set
    fn handle_list_item_start(&mut self, override_style_name: Option<String>) -> ListItem {
        let mut element = ListItem::new(None, None);
        if let Some(x) = override_style_name {
            // If the override style name is defined
            if let Some(x) = self.auto_list_styles.get(&x) {
                // If the referenced style is an automatic style
                let bullet = x[(self.list_depth - 1) as usize].clone();
                element = ListItem::new(None, Some(bullet));
            } else if let Some(x) = self.document_root.styles.classes.get(&x) {
                // If the referenced style is a named style
                if let Some(Element::List(x)) = &x.element {
                    // If the referenced named style is actually for a list
                    if let Some(x) = x.get_bullet_cycle() {
                        // If the referenced named style actually defines a bullet cycle
                        // (ODT will always define a bullet cycle, so wen don't need to
                        // look at the single bullet definition here)
                        let bullet = x[(self.list_depth - 1) as usize].clone();
                        element = ListItem::new(None, Some(bullet));
                    }
                }
            }
        }
        element
    }

    /// Inserts a page break into the next position in the document
    pub fn insert_break(&mut self, page: bool) {
        let node;
        if page {
            node = Node::PageBreak;
        } else {
            node = Node::LineBreak;
        }
        if self.document_hierarchy.is_empty() {
            self.document_root.content.push(ChildNode::Node(node));
        } else {
            self.document_hierarchy
                .last_mut()
                .unwrap()
                .get_common()
                .children
                .as_mut()
                .unwrap()
                .push(ChildNode::Node(node));
        }
    }
}

/// Takes the results of either heading_begin() or paragraph_begin() (called params)
/// and a reference to the map of automatic style names to the map of CSS properties,
/// and returns the element from params with its styles attached together
/// with the values for set_children_underline and ensure_children_no_underline in ODTParser
fn check_underline(
    params: (Element, String),
    auto_styles: &HashMap<String, HashMap<String, String>>,
    set_children_underline_old: bool,
    ensure_children_no_underline_old: bool,
) -> (Element, bool, bool) {
    let mut ensure_children_no_underline = ensure_children_no_underline_old;
    let mut set_children_underline = set_children_underline_old;
    let (mut element, style_name) = params;
    let mut style = auto_styles
        .get(&style_name)
        .unwrap_or(&HashMap::new())
        .clone();
    style.remove("_parent");
    let underline = style.get("textDecorationLine");
    let underline_color = style.get("textDecorationColor");
    if let Some(x) = underline {
        match x.as_str() {
            "underline" => {
                ensure_children_no_underline = true;
                if let Some(x) = underline_color {
                    set_children_underline = x == "currentcolor";
                }
            }
            "none" => ensure_children_no_underline = false,
            _ => (),
        }
    }
    element.get_common().styles = style;
    (
        element,
        set_children_underline,
        ensure_children_no_underline,
    )
}

/// Takes a mutable reference to a map of CSS style properties to values and 2 booleans
/// (the boolean results of check_underline()), and adds an extra CSS property to
/// handle some special cases related to underlines if needed depending on the booleans
pub fn handle_underline(
    style_map: &mut HashMap<String, String>,
    set_children_underline: bool,
    ensure_children_no_underline: bool,
) {
    if set_children_underline {
        if let Some(x) = style_map.get("textDecorationLine") {
            if x != "none" {
                style_map.insert("textDecorationLine".to_string(), "underline".to_string());
            } else if ensure_children_no_underline {
                // Need this to make sure the underline is actually not there, because CSS things
                style_map.insert("display".to_string(), "inline-block".to_string());
            }
        } else {
            style_map.insert("textDecoration".to_string(), "underline".to_string());
        }
    } else if ensure_children_no_underline {
        if let Some(x) = style_map.get("textDecorationLine") {
            if x == "none" {
                // Need this to make sure the underline is actually not there, because CSS things
                style_map.insert("display".to_string(), "inline-block".to_string());
            }
        }
    }
}

/// Takes the set of attributes of a text:h tag in the ODT's content.xml,
/// and returns a heading element based on the attributes,
/// together with the value of the text:style-name attribute of the tag
fn heading_begin(
    attributes: Attributes,
    auto_styles: &mut HashMap<String, HashMap<String, String>>,
) -> (Element, String) {
    let mut level = 0;
    let mut style_name = String::new();
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            match name {
                "text:outline-level" => {
                    level = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("1")
                    .parse::<u32>()
                    .unwrap_or(1);
                }
                "text:style-name" => {
                    style_name = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("")
                    .to_string()
                }
                _ => (),
            }
        }
    }
    let mut parent_style: Option<String> = None;
    let parent = auto_styles.get_mut(&style_name);
    if let Some(parent) = parent {
        parent_style = Some(parent.get("_parent").unwrap_or(&String::new()).to_string());
    }
    let element = Heading::new(parent_style, level);
    (Element::Heading(element), style_name)
}

/// Takes the set of attributes of a text:p tag in the ODT's content.xml,
/// and returns a paragraph element together with the value of the text:style-name attribute of the tag
fn paragraph_begin(
    attributes: Attributes,
    auto_styles: &mut HashMap<String, HashMap<String, String>>,
) -> (Element, String) {
    let mut style_name = String::new();
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            if name == "text:style-name" {
                style_name = std::str::from_utf8(
                    &i.unescaped_value()
                        .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                )
                .unwrap_or("")
                .to_string();
            }
        }
    }
    let mut parent_style: Option<String> = None;
    let parent = auto_styles.get_mut(&style_name);
    if let Some(parent) = parent {
        parent_style = Some(parent.get("_parent").unwrap_or(&String::new()).to_string());
    }
    (
        Element::Paragraph(ElementCommon::new(parent_style)),
        style_name,
    )
}

/// Takes the set of attributes of a text:span tag in the ODT's content.xml
/// and returns a span element together with the value of the text:style-name attribute of the tag
fn span_begin(
    attributes: Attributes,
    auto_styles: &mut HashMap<String, HashMap<String, String>>,
) -> (Element, String) {
    let mut style_name = String::new();
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            if name == "text:style-name" {
                style_name = std::str::from_utf8(
                    &i.unescaped_value()
                        .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                )
                .unwrap_or("")
                .to_string();
            }
        }
    }
    let mut parent_style: Option<String> = None;
    let parent = auto_styles.get_mut(&style_name);
    if let Some(parent) = parent {
        parent_style = Some(parent.get("_parent").unwrap_or(&String::new()).to_string());
    }
    (Element::Span(ElementCommon::new(parent_style)), style_name)
}

/// Takes the set of attributes of a text:a tag in the ODT's content.xml
/// and returns an anchor element together with the value of the text:style-name attribute of the tag
fn a_begin(
    attributes: Attributes,
    auto_styles: &mut HashMap<String, HashMap<String, String>>,
) -> (Element, String) {
    let mut href = String::new();
    let mut style_name = String::new();
    let mut title: Option<String> = None;
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            match name {
                "text:style-name" => {
                    style_name = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("")
                    .to_string();
                }
                "xlink:href" => {
                    href = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("")
                    .to_string();
                }
                "office:title" => {
                    title = Some(
                        std::str::from_utf8(
                            &i.unescaped_value()
                                .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                        )
                        .unwrap_or("")
                        .to_string(),
                    );
                }
                _ => (),
            }
        }
    }
    let mut parent_style: Option<String> = None;
    let parent = auto_styles.get_mut(&style_name);
    if let Some(parent) = parent {
        parent_style = Some(parent.get("_parent").unwrap_or(&String::new()).to_string());
    }
    (
        Element::Hyperlink(Hyperlink::new(parent_style, title, href)),
        style_name,
    )
}

/// Returns the style name, id, id of the list to continue and whether to continue from the
/// previous list (only style name for now since KDF doesn't support the rest yet)
fn list_begin(attributes: Attributes) -> Option<String> {
    let mut style_name: Option<String> = None;
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            if name == "text:style-name" {
                style_name = Some(
                    std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("")
                    .to_string(),
                );
            }
        }
    }
    style_name
}

/// Returns the override style name of the list item
fn list_item_begin(attributes: Attributes) -> Option<String> {
    let mut override_style_name: Option<String> = None;
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            if name == "text:style-override" {
                override_style_name = Some(
                    std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("")
                    .to_string(),
                );
            }
        }
    }
    override_style_name
}

/// Takes the set of attributes of a style:text-properties tag in the ODT's content.xml,
/// and inserts the CSS properties and values into the referenced HashMap
pub fn text_properties_begin(attributes: Attributes, map: &mut HashMap<String, String>) {
    let mut is_double_underline = false;
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            let (prefix, local_name) = name.split_at(name.find(':').unwrap_or(0));
            let local_name = &local_name[1..];
            let value = std::str::from_utf8(
                &i.unescaped_value()
                    .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
            )
            .unwrap_or("")
            .to_string();
            if prefix == "fo" {
                text_properties_begin_fo(local_name, value, map);
            } else if prefix == "style" && text_properties_begin_style(local_name, value, map) {
                is_double_underline = true;
            }
        }
    }
    if is_double_underline {
        // The ODT standard supports double underlines of any kind (solid, dotted, etc), while CSS
        // only supports double solid underlines, so prioritize the double over the line style?
        map.insert("textDecorationStyle".to_string(), "double".to_string());
    }
}

/// Helper for text_properties_begin() to respond to attributes with "fo" prefix
fn text_properties_begin_fo(local_name: &str, value: String, styles: &mut HashMap<String, String>) {
    match local_name {
        "font-weight" => {
            // All valid values for this attribute is also valid in the CSS equivalent, so just use it as is
            styles.insert("fontWeight".to_string(), value);
        }
        "font-style" if value != "backslant" => {
            // `backslant` is not valid in CSS, but all the other ones are
            styles.insert("fontStyle".to_string(), value);
        }
        "font-family" => {
            if let Some(font_name) = styles.remove("fontFamily") {
                // If the font name is there already then prepend the font family to it
                styles.insert(
                    "fontFamily".to_string(),
                    format!("{}, {}", value, font_name),
                );
            } else {
                // Otherwise just put the font family there
                styles.insert("fontFamily".to_string(), value);
            }
        }
        "color" => {
            styles.insert("color".to_string(), value);
        }
        "font-size" => {
            styles.insert("fontSize".to_string(), value);
        }
        "background-color" => {
            styles.insert("backgroundColor".to_string(), value);
        }
        "font-variant" => {
            styles.insert("fontVariant".to_string(), value);
        }
        "hyphenate" => {
            text_properties_begin_fo_hyphenate(value, styles);
        }
        "letter-spacing" => {
            styles.insert("letterSpacing".to_string(), value);
        }
        "text-shadow" => {
            styles.insert("textShadow".to_string(), value);
        }
        "text-transform" => {
            styles.insert("textTransform".to_string(), value);
        }
        _ => (),
    };
}

/// Helper for text_properties_begin() to respond to attributes with "style" prefix,
/// returns true if the attribute indicates that the underline style should be a double underline,
/// returns false otherwise
/// local_name here is the name of the tag without the prefix
fn text_properties_begin_style(
    local_name: &str,
    value: String,
    styles: &mut HashMap<String, String>,
) -> bool {
    match local_name {
        "text-underline-type" if value == "double" => true,
        "font-name" => {
            if let Some(font_family) = styles.remove("fontFamily") {
                // If the font family was already set previously then append the font name
                styles.insert(
                    "fontFamily".to_string(),
                    format!("{}, {}", font_family, value),
                );
            } else {
                // Otherwise just put the font name there
                styles.insert("fontFamily".to_string(), value);
            }
            false
        }
        "text-underline-style" => {
            text_properties_begin_style_underline_style(value, styles);
            false
        }
        "text-underline-color" => {
            text_properties_begin_style_underline_color(value, styles);
            false
        }
        "letter-kerning" => {
            text_properties_begin_style_letter_kerning(value, styles);
            false
        }
        "text-position" => {
            text_properties_begin_style_text_position(value, styles);
            false
        }
        _ => false,
    }
}

/// Helper for text_properties_begin_style() to handle underline style
fn text_properties_begin_style_underline_style(
    value: String,
    styles: &mut HashMap<String, String>,
) {
    if value == "none" {
        styles.insert("textDecorationLine".to_string(), "none".to_string());
        return;
    }
    styles.insert("textDecorationLine".to_string(), "underline".to_string());
    match value.as_str() {
        "dash" => styles.insert("textDecorationStyle".to_string(), "dashed".to_string()),
        "dotted" => styles.insert("textDecorationStyle".to_string(), "dotted".to_string()),
        "wave" => styles.insert("textDecorationStyle".to_string(), "wavy".to_string()),
        // There are a few possible styles in ODF that aren't present in CSS
        // (dot-dash, dot-dot-dash, long-dash), so just put in a basic underline?
        "solid" | _ => styles.insert("textDecorationStyle".to_string(), "solid".to_string()),
    };
}

/// Helper for text_properties_begin_style() to handle underline color
fn text_properties_begin_style_underline_color(
    value: String,
    styles: &mut HashMap<String, String>,
) {
    if value == "font-color" {
        styles.insert(
            "textDecorationColor".to_string(),
            "currentcolor".to_string(),
        );
    } else {
        // The other valid values are all in hex format
        styles.insert("textDecorationColor".to_string(), value);
    }
}

/// Helper for text_properties_begin_fo() to handle hyphenate
fn text_properties_begin_fo_hyphenate(value: String, styles: &mut HashMap<String, String>) {
    match value.as_str() {
        "true" => {
            styles.insert("hyphens".to_string(), "auto".to_string());
        }
        "false" => {
            styles.insert("hyphens".to_string(), "none".to_string());
        }
        _ => (),
    }
}

/// Helper for text_properties_begin_style() to handle letter kerning
fn text_properties_begin_style_letter_kerning(value: String, styles: &mut HashMap<String, String>) {
    match value.as_str() {
        "true" => {
            styles.insert("fontKerning".to_string(), "normal".to_string());
        }
        "false" => {
            styles.insert("fontKerning".to_string(), "none".to_string());
        }
        _ => (),
    }
}

/// Helper for text_properties_begin_style to handle text position (superscript and subscript)
fn text_properties_begin_style_text_position(value: String, styles: &mut HashMap<String, String>) {
    let mut split_values = value.split_whitespace();
    // The first parameter specifies how high/low the text is (mandatory)
    if let Some(vertical_align) = split_values.next() {
        styles.insert("verticalAlign".to_string(), vertical_align.to_string());
    }
    // The second one specifies how small the text is (optional)
    if let Some(font_size) = split_values.next() {
        styles.insert("fontSize".to_string(), font_size.to_string());
    } else {
        // The ODT spec does not specify an explicit default, this is what LibreOffice uses
        styles.insert("fontSize".to_string(), "58%".to_string());
    }
}
