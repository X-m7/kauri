use super::*;
use crate::document::node::{ElementCommon, TableCell, TableColumn};

enum TableAlign {
    Center,
    Left,
    Right,
    Margins,
}

impl TableAlign {
    fn from_string(direction: String) -> Option<TableAlign> {
        match direction.as_str() {
            "center" => Some(TableAlign::Center),
            "left" => Some(TableAlign::Left),
            "right" => Some(TableAlign::Right),
            "margins" => Some(TableAlign::Margins),
            _ => None,
        }
    }
}

impl ODTParser {
    /// Helper for handle_element_start() to respond to tags with "table" prefix
    pub fn handle_element_start_table(&mut self, local_name: &str, attributes: Attributes) {
        match local_name {
            "table" => {
                self.document_hierarchy
                    .push(table_begin(attributes, &self.auto_styles));
                self.table_column_default_style_names.push(Vec::new());
                self.table_column_number.push(0);
                self.table_row_default_style_names.push(Vec::new());
                self.table_row_number.push(0);
            }
            "table-row" => {
                if self.table_row_default_style_names.is_empty() {
                    return;
                }
                let (row, default_cell_style_name) = table_row_begin(attributes, &self.auto_styles);
                self.document_hierarchy.push(row);
                let table_row_default_style_names =
                    self.table_row_default_style_names.last_mut().unwrap();
                if let Some(default_cell_style_name) = default_cell_style_name {
                    table_row_default_style_names.push(default_cell_style_name.clone());
                } else {
                    table_row_default_style_names.push("".to_string());
                }
                let len = self.table_column_number.len();
                self.table_column_number[len - 1] = 0;
            }
            "table-cell" => {
                // This should not be empty, just check this since the other three will be set up together with it
                if self.table_row_default_style_names.is_empty() {
                    return;
                }
                let mut default_style_name = String::new();
                let row_default = &self.table_row_default_style_names.last().unwrap()
                    [*self.table_row_number.last().unwrap() as usize];
                let column_default = &self.table_column_default_style_names.last().unwrap()
                    [*self.table_column_number.last().unwrap() as usize];
                if row_default != "" {
                    default_style_name = row_default.to_string();
                } else if column_default != "" {
                    default_style_name = column_default.to_string();
                }
                self.document_hierarchy.push(table_cell_begin(
                    attributes,
                    &self.auto_styles,
                    default_style_name,
                ));
            }
            _ => (),
        }
    }

    /// Helper for handle_element_end() to respond to tags with "table" prefix
    pub fn handle_element_end_table(&mut self, local_name: &str) {
        match local_name {
            "table" => {
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
                self.table_column_default_style_names.pop();
                self.table_column_number.pop();
                self.table_row_default_style_names.pop();
                self.table_row_number.pop();
            }
            "table-row" => {
                let mut child = self.document_hierarchy.pop().unwrap();
                let mut repeat = child
                    .get_common()
                    .attributes
                    .remove("_repeat")
                    .unwrap_or_else(|| "1".to_string())
                    .parse::<u32>()
                    .unwrap_or(1);
                let len = self.table_row_number.len();
                self.table_row_number[len - 1] += repeat;
                while repeat != 0 {
                    if self.document_hierarchy.is_empty() {
                        self.document_root
                            .content
                            .push(ChildNode::Element(child.clone()));
                    } else {
                        self.document_hierarchy
                            .last_mut()
                            .unwrap()
                            .get_common()
                            .children
                            .as_mut()
                            .unwrap()
                            .push(ChildNode::Element(child.clone()));
                    }
                    repeat -= 1;
                }
            }
            "table-cell" => {
                let mut child = self.document_hierarchy.pop().unwrap();
                let mut repeat = child
                    .get_common()
                    .attributes
                    .remove("_repeat")
                    .unwrap_or_else(|| "1".to_string())
                    .parse::<u32>()
                    .unwrap_or(1);
                let len = self.table_column_number.len();
                self.table_column_number[len - 1] += repeat;
                while repeat != 0 {
                    if self.document_hierarchy.is_empty() {
                        self.document_root
                            .content
                            .push(ChildNode::Element(child.clone()));
                    } else {
                        self.document_hierarchy
                            .last_mut()
                            .unwrap()
                            .get_common()
                            .children
                            .as_mut()
                            .unwrap()
                            .push(ChildNode::Element(child.clone()));
                    }
                    repeat -= 1;
                }
            }
            _ => (),
        }
    }

    pub fn handle_element_empty_table(&mut self, local_name: &str, attributes: Attributes) {
        if local_name != "table-column" {
            return;
        }
        // We should be inside a table if we see this, so if it is empty ignore
        if self.document_hierarchy.is_empty() || self.table_column_default_style_names.is_empty() {
            return;
        }
        if let ChildNode::Element(ref mut element) = &mut self
            .document_hierarchy
            .last_mut()
            .unwrap()
            .get_common()
            .children
            .as_mut()
            .unwrap()[1]
        {
            let (table, default_cell_style_name, mut repeat) =
                table_column_begin(attributes, &self.auto_styles);
            element
                .get_common()
                .children
                .as_mut()
                .unwrap()
                .push(ChildNode::Element(table));
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
    }
}

/// Takes the set of attributes of a style:table-properties tag in the ODT's content.xml,
/// and inserts the CSS properties and values into the referenced HashMap
pub fn table_properties_begin(attributes: Attributes, map: &mut HashMap<String, String>) {
    let mut table_alignment = TableAlign::Margins;
    let mut margin_left = "0cm".to_string();
    let mut margin_right = "0cm".to_string();
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            let (prefix, local_name) = name.split_at(name.find(':').unwrap_or(0));
            let local_name = &local_name[1..];
            let value = std::str::from_utf8(
                &i.unescaped_value()
                    .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
            )
            .unwrap_or("what")
            .to_string();
            match prefix {
                "fo" => {
                    let (margin_left_option, margin_right_option) =
                        table_properties_begin_fo(local_name, value, map);
                    if let Some(margin_left_option) = margin_left_option {
                        margin_left = margin_left_option;
                    }
                    if let Some(margin_right_option) = margin_right_option {
                        margin_right = margin_right_option;
                    }
                }
                "style" => table_properties_begin_style(local_name, value, map),
                "table" => {
                    if let Some(table_alignment_option) =
                        table_properties_begin_table(local_name, value, map)
                    {
                        table_alignment = table_alignment_option;
                    }
                }
                _ => (),
            }
        }
    }
    match table_alignment {
        // The specification says to ignore the right margin if it is aligned left
        TableAlign::Left => {
            map.insert("marginRight".to_string(), "unset".to_string());
        }
        // Similarly for align right
        TableAlign::Right => {
            map.insert("marginLeft".to_string(), "unset".to_string());
        }
        // For center alignment we are supposed to ignore both side margins, while in CSS centering is done by making both of them auto
        TableAlign::Center => {
            map.insert("marginLeft".to_string(), "auto".to_string());
            map.insert("marginRight".to_string(), "auto".to_string());
        }
        TableAlign::Margins => {
            map.insert(
                "width".to_string(),
                format!("calc(100% - {} - {})", margin_left, margin_right),
            );
        }
    }
}

/// Helper for table_properties_begin() for attributes with "fo" prefix,
/// returns the values of the left and right margin if the attribute specifies either
fn table_properties_begin_fo(
    local_name: &str,
    value: String,
    styles: &mut HashMap<String, String>,
) -> (Option<String>, Option<String>) {
    match local_name {
        "background-color" => {
            styles.insert("fontWeight".to_string(), value);
        }
        "break-after" => {
            if value == "auto" || value == "column" || value == "page" {
                styles.insert("breakAfter".to_string(), value);
            }
        }
        "break-before" => {
            if value == "auto" || value == "column" || value == "page" {
                styles.insert("breakBefore".to_string(), value);
            }
        }
        "margin" => {
            let margin_split = value.clone();
            let mut margin_split = margin_split.split(' ');
            let right = margin_split.nth(1);
            let left = margin_split.nth(1);
            let left = left.unwrap_or("0cm").to_string();
            let right = right.unwrap_or("0cm").to_string();
            styles.insert("margin".to_string(), value);
            return (Some(left), Some(right));
        }
        "margin-top" => {
            styles.insert("marginTop".to_string(), value);
        }
        "margin-bottom" => {
            styles.insert("marginLeft".to_string(), value);
        }
        "margin-left" => {
            let left = value.clone();
            styles.insert("marginRight".to_string(), value);
            return (Some(left), None);
        }
        "margin-right" => {
            let right = value.clone();
            styles.insert("marginBottom".to_string(), value);
            return (None, Some(right));
        }
        _ => (),
    }
    (None, None)
}

/// Helper for table_properties_begin() for attributes with "style" prefix
fn table_properties_begin_style(
    local_name: &str,
    value: String,
    styles: &mut HashMap<String, String>,
) {
    match local_name {
        "rel-width" | "width" => {
            styles.insert("width".to_string(), value);
        }
        "shadow" => {
            styles.insert("boxShadow".to_string(), value);
        }
        "writing-mode" => table_properties_begin_style_writing_mode(value, styles),
        _ => (),
    }
}

/// Helper for table_properties_begin_style() and table_cell_properties_begin_style() for the writing mode
fn table_properties_begin_style_writing_mode(value: String, styles: &mut HashMap<String, String>) {
    match value.as_str() {
        // According to the MDN the replacement for "rl" and "rl-tb" is also "horizontal-tb" apparently
        "lr-tb" | "lr" | "rl" | "rl-tb" => {
            styles.insert("writingMode".to_string(), "horizontal-tb".to_string());
        }
        // MDN says "tb" is supposed to be replaced by "vertical-lr", but the ODT definition says that "tb" is a synonym for "tb-rl"
        "tb-rl" | "tb" => {
            styles.insert("writingMode".to_string(), "vertical-rl".to_string());
        }
        "tb-lr" => {
            styles.insert("writingMode".to_string(), "vertical-lr".to_string());
        }
        _ => (),
    }
}

/// Helper for table_properties_begin() for attributes with "table" prefix
fn table_properties_begin_table(
    local_name: &str,
    value: String,
    styles: &mut HashMap<String, String>,
) -> Option<TableAlign> {
    match local_name {
        "align" => return TableAlign::from_string(value),
        "border-model" => table_properties_begin_table_border_model(value, styles),
        "display" if value == "false" => {
            styles.insert("display".to_string(), "none".to_string());
        }
        _ => (),
    }
    None
}

/// Helper for table_properties_begin_table() for the border-model attribute
fn table_properties_begin_table_border_model(value: String, styles: &mut HashMap<String, String>) {
    match value.as_str() {
        "collapsing" => {
            styles.insert("borderCollapse".to_string(), "collapse".to_string());
        }
        "separating" => {
            styles.insert("borderCollapse".to_string(), "separate".to_string());
        }
        _ => (),
    }
}

/// Takes the set of attributes of a style:table-column-properties tag in the ODT's content.xml,
/// and inserts the CSS properties and values into the referenced HashMap
pub fn table_column_properties_begin(attributes: Attributes, map: &mut HashMap<String, String>) {
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            let value = std::str::from_utf8(
                &i.unescaped_value()
                    .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
            )
            .unwrap_or("what")
            .to_string();
            match name {
                "fo:break-after" => {
                    if value == "auto" || value == "column" || value == "page" {
                        map.insert("breakAfter".to_string(), value);
                    }
                }
                "fo:break-before" => {
                    if value == "auto" || value == "column" || value == "page" {
                        map.insert("breakBefore".to_string(), value);
                    }
                }
                "style:column-width" => {
                    map.insert("width".to_string(), value);
                }
                "style:use-optimal-column-width" => {
                    if value == "true" {
                        map.insert("width".to_string(), "auto".to_string());
                    }
                }
                _ => (),
            }
        }
    }
}

/// Takes the set of attributes of a style:table-row-properties tag in the ODT's content.xml,
/// and inserts the CSS properties and values into the referenced HashMap
pub fn table_row_properties_begin(attributes: Attributes, map: &mut HashMap<String, String>) {
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            let value = std::str::from_utf8(
                &i.unescaped_value()
                    .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
            )
            .unwrap_or("what")
            .to_string();
            match name {
                "fo:break-after" => {
                    if value == "auto" || value == "column" || value == "page" {
                        map.insert("breakAfter".to_string(), value);
                    }
                }
                "fo:break-before" => {
                    if value == "auto" || value == "column" || value == "page" {
                        map.insert("breakBefore".to_string(), value);
                    }
                }
                "fo:background-color" => {
                    map.insert("backgroundColor".to_string(), value);
                }
                "style:row-height" => {
                    map.insert("height".to_string(), value);
                }
                "style:use-optimal-row-height" => {
                    if value == "true" {
                        map.insert("height".to_string(), "auto".to_string());
                    }
                }
                "style:min-row-height" => {
                    map.insert("min-height".to_string(), value);
                }
                _ => (),
            }
        }
    }
}

/// Takes the set of attributes of a style:table-cell-properties tag in the ODT's content.xml,
/// and inserts the CSS properties and values into the referenced HashMap
pub fn table_cell_properties_begin(attributes: Attributes, map: &mut HashMap<String, String>) {
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            let (prefix, local_name) = name.split_at(name.find(':').unwrap_or(0));
            let local_name = &local_name[1..];
            let value = std::str::from_utf8(
                &i.unescaped_value()
                    .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
            )
            .unwrap_or("what")
            .to_string();
            match prefix {
                "fo" => table_cell_properties_begin_fo(local_name, value, map),
                "style" => table_cell_properties_begin_style(local_name, value, map),
                _ => (),
            }
        }
    }
}

/// Helper for table_cell_properties_begin() for attributes with "fo" prefix
fn table_cell_properties_begin_fo(
    local_name: &str,
    value: String,
    styles: &mut HashMap<String, String>,
) {
    match local_name {
        "background-color" => {
            styles.insert("backgroundColor".to_string(), value);
        }
        "border" => {
            styles.insert("border".to_string(), value);
        }
        "border-left" => {
            styles.insert("borderLeft".to_string(), value);
        }
        "border-right" => {
            styles.insert("borderRight".to_string(), value);
        }
        "border-top" => {
            styles.insert("borderTop".to_string(), value);
        }
        "border-bottom" => {
            styles.insert("borderBottom".to_string(), value);
        }
        "padding" => {
            styles.insert("padding".to_string(), value);
        }
        "padding-top" => {
            styles.insert("paddingTop".to_string(), value);
        }
        "padding-bottom" => {
            styles.insert("paddingBottom".to_string(), value);
        }
        "padding-left" => {
            styles.insert("paddingLeft".to_string(), value);
        }
        "padding-right" => {
            styles.insert("paddingRight".to_string(), value);
        }
        _ => (),
    }
}

/// Helper for table_cell_properties_begin() for attributes with "style" prefix
fn table_cell_properties_begin_style(
    local_name: &str,
    value: String,
    styles: &mut HashMap<String, String>,
) {
    match local_name {
        "rotation-angle" => {
            styles.insert("transform".to_string(), format!("rotate({})", value));
        }
        "shadow" => {
            styles.insert("boxShadow".to_string(), value);
        }
        "writing-mode" => table_properties_begin_style_writing_mode(value, styles),
        "vertical-align" => {
            if value == "middle" || value == "top" || value == "bottom" {
                styles.insert("verticalAlign".to_string(), value);
            }
        }
        _ => (),
    }
}

/// Takes the set of attributes of a table:table tag in the ODT's content.xml
/// and a reference to the map of automatic style names to the map of CSS properties,
/// then returns a table element with styles attached
fn table_begin(
    attributes: Attributes,
    auto_styles: &HashMap<String, HashMap<String, String>>,
) -> Element {
    let mut style_name = String::new();
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            if name == "table:style-name" {
                style_name = std::str::from_utf8(
                    &i.unescaped_value()
                        .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                )
                .unwrap_or("")
                .to_string();
            }
        }
    }
    let mut element = ElementCommon::new(None);
    element.styles = auto_styles
        .get(&style_name)
        .unwrap_or(&HashMap::new())
        .clone();
    // This is only used by the children, so get rid of it in the copy here
    element.styles.remove("_parent");
    // Caption is always the first child of a table in HTML if it exists, and colgroup always comes after if it is there too,
    // so we add it in in order to have a definitive position as to where the colgroup will come
    element
        .children
        .as_mut()
        .unwrap()
        .push(ChildNode::Element(Element::Caption(ElementCommon::new(
            None,
        ))));
    element
        .children
        .as_mut()
        .unwrap()
        .push(ChildNode::Element(Element::TableColumnGroup(
            ElementCommon::new(None),
        )));
    Element::Table(element)
}

/// Takes the set of attributes of a table:table-column tag in the ODT's content.xml
/// and a reference to the map of automatic style names to the map of CSS properties,
/// then returns a table column element with styles attached, the default cell style name (if any)
/// and how many times it should be repeated (as in the HTML span attribute for columns)
pub fn table_column_begin(
    attributes: Attributes,
    auto_styles: &HashMap<String, HashMap<String, String>>,
) -> (Element, Option<String>, u32) {
    let mut style_name = String::new();
    let mut repeat = 1;
    let mut default_cell_style_name: Option<String> = None;
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            match name {
                "table:style-name" => {
                    style_name = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("")
                    .to_string()
                }
                "table:number-columns-repeated" => {
                    repeat = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("1")
                    .parse::<u32>()
                    .unwrap_or(1);
                }
                "table:default-cell-style-name" => {
                    default_cell_style_name = Some(
                        std::str::from_utf8(
                            &i.unescaped_value()
                                .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                        )
                        .unwrap_or("1")
                        .to_string(),
                    )
                }
                _ => (),
            }
        }
    }
    let mut style = auto_styles
        .get(&style_name)
        .unwrap_or(&HashMap::new())
        .clone();
    let parent_style = style.remove("_parent");
    let mut element = TableColumn::new(parent_style, Some(repeat));
    element.common.styles = style;
    let element = Element::TableColumn(element);
    if let Some(default_cell_style_name) = default_cell_style_name {
        return (element, Some(default_cell_style_name), repeat);
    }
    (element, None, repeat)
}

/// Takes the set of attributes of a table:table-row tag in the ODT's content.xml
/// and a reference to the map of automatic style names to the map of CSS properties,
/// then returns a table row element with styles attached, the default cell style name (if any)
/// and how many times it should be repeated (stored inside the element as an attribute called "_repeat")
fn table_row_begin(
    attributes: Attributes,
    auto_styles: &HashMap<String, HashMap<String, String>>,
) -> (Element, Option<String>) {
    let mut style_name = String::new();
    let mut default_cell_style_name: Option<String> = None;
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            match name {
                "table:style-name" => {
                    style_name = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("")
                    .to_string()
                }
                "table:default-cell-style-name" => {
                    default_cell_style_name = Some(
                        std::str::from_utf8(
                            &i.unescaped_value()
                                .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                        )
                        .unwrap_or("1")
                        .to_string(),
                    )
                }
                _ => (),
            }
        }
    }
    let mut style = auto_styles
        .get(&style_name)
        .unwrap_or(&HashMap::new())
        .clone();
    let parent_style = style.remove("_parent");
    let mut element = ElementCommon::new(parent_style);
    element.styles = style;
    let element = Element::TableRow(element);
    if let Some(default_cell_style_name) = default_cell_style_name {
        return (element, Some(default_cell_style_name));
    }
    (element, None)
}

/// Takes the set of attributes of a table:table-cell tag in the ODT's content.xml,
/// a reference to the map of automatic style names to the map of CSS properties,
/// and the default cell style name,
/// then returns a table cell element with styles attached and how many times it should be repeated
/// (stored inside the element as an attribute called "_repeat")
fn table_cell_begin(
    attributes: Attributes,
    auto_styles: &HashMap<String, HashMap<String, String>>,
    default_style_name: String,
) -> Element {
    let mut style_name = String::new();
    let mut repeat = String::new();
    let mut col_span: Option<u32> = None;
    let mut row_span: Option<u32> = None;
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            match name {
                "table:style-name" => {
                    style_name = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("")
                    .to_string()
                }
                "table:number-columns-repeated" => {
                    repeat = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("1")
                    .to_string();
                }
                "table:number-columns-spanned" => {
                    col_span = Some(
                        std::str::from_utf8(
                            &i.unescaped_value()
                                .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                        )
                        .unwrap_or("1")
                        .parse::<u32>()
                        .unwrap_or(1),
                    );
                }
                "table:number-rows-spanned" => {
                    row_span = Some(
                        std::str::from_utf8(
                            &i.unescaped_value()
                                .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                        )
                        .unwrap_or("1")
                        .parse::<u32>()
                        .unwrap_or(1),
                    );
                }
                _ => (),
            }
        }
    }
    let mut final_style_name = &style_name;
    if final_style_name == "" {
        final_style_name = &default_style_name;
    }
    let mut style = auto_styles
        .get(final_style_name)
        .unwrap_or(&HashMap::new())
        .clone();
    let parent_style = style.remove("_parent");
    let mut element = TableCell::new(parent_style, row_span, col_span);
    element
        .common
        .attributes
        .insert("_repeat".to_string(), repeat);
    element.common.styles = style;
    Element::TableCell(element)
}
