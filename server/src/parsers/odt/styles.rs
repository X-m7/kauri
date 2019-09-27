use super::*;
use crate::document::node::{
    Heading, List, ListBulletCharacter, ListBulletImage, ListBulletVariant,
};

impl ODTParser {
    pub fn parse_styles(
        &mut self,
        archive: &mut zip::ZipArchive<std::fs::File>,
    ) -> Result<(), String> {
        // returns a ZipFile struct which implements Read if the file is in the archive
        let styles_xml = archive.by_name("styles.xml");
        if let Err(e) = styles_xml {
            // Handle case where there is no content.xml (so probably not actually an ODT file)
            return Err(e.to_string());
        }
        let content_xml = BufReader::new(styles_xml.unwrap()); //add buffering because quick-xml's reader requires it
        let mut parser = Reader::from_reader(content_xml);
        let mut buffer = Vec::new();

        // These are here instead of the struct because we may need to move the contents of these somewhere else
        let mut current_style_name = String::new();
        let mut current_style_value: Option<Style> = None;
        let mut current_list_style_value: Vec<ListBullet> = Vec::with_capacity(10);

        let default_bullet = ListBulletVariant::new(None, None, None, "filledBullet".to_string());
        let default_bullet = ListBullet::Variant(default_bullet);
        current_list_style_value.resize(10, default_bullet.clone());

        loop {
            // Iterate through the XML
            match parser.read_event(&mut buffer) {
                Ok(Event::Start(contents)) => {
                    // If there is already an initialised Style object
                    if let Some(style) = current_style_value.as_mut() {
                        if let Some((current_style_name_new, current_style_value_new)) = self
                            .styles_handle_element_start(
                                std::str::from_utf8(contents.name()).unwrap_or(":"),
                                contents.attributes(),
                                Some(style),
                                &mut current_list_style_value,
                            )
                        {
                            current_style_name = current_style_name_new;
                            current_style_value = Some(current_style_value_new);
                        }
                    // Else if there is none yet
                    } else if let Some((current_style_name_new, current_style_value_new)) = self
                        .styles_handle_element_start(
                            std::str::from_utf8(contents.name()).unwrap_or(":"),
                            contents.attributes(),
                            None,
                            &mut current_list_style_value,
                        )
                    {
                        current_style_name = current_style_name_new;
                        current_style_value = Some(current_style_value_new);
                    }
                }
                Ok(Event::End(contents)) => {
                    let (
                        current_style_name_new,
                        current_style_value_new,
                        current_list_style_value_new,
                    ) = self.styles_handle_element_end(
                        std::str::from_utf8(contents.name()).unwrap_or(":"),
                        current_style_name,
                        current_style_value,
                        current_list_style_value,
                    );
                    if let Some(x) = current_style_name_new {
                        current_style_name = x;
                    } else {
                        current_style_name = String::new();
                    }
                    current_style_value = current_style_value_new;
                    if let Some(x) = current_list_style_value_new {
                        current_list_style_value = x;
                    } else {
                        current_list_style_value = Vec::with_capacity(10);
                        current_list_style_value.resize(10, default_bullet.clone());
                    }
                }
                Ok(Event::Empty(contents)) => {
                    if let Some(style) = current_style_value.as_mut() {
                        self.styles_handle_element_empty(
                            std::str::from_utf8(contents.name()).unwrap_or(":"),
                            contents.attributes(),
                            style,
                            &mut current_list_style_value,
                        );
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    println!("Styles parsing error: {}", e);
                    return Err(e.to_string());
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Returns the style name and Style object
    fn styles_handle_element_start(
        &mut self,
        name: &str,
        attributes: Attributes,
        style: Option<&mut Style>,
        bullet_cycle: &mut Vec<ListBullet>,
    ) -> Option<(String, Style)> {
        let mut level_and_bullet: Option<(u32, ListBullet)> = None;
        match name {
            "style:default-style" => return Some(default_style_begin(attributes)),
            "style:style" => return Some(style_style_begin(attributes)),
            "text:list-style" => {
                self.in_list_style = true;
                return Some(style_list_style_begin(attributes));
            }
            "table:table-row-properties" if style.is_some() => {
                table_row_properties_begin(attributes, &mut style.unwrap().styles)
            }
            "table:table-properties" if style.is_some() => {
                table_properties_begin(attributes, &mut style.unwrap().styles)
            }
            "table:table-cell-properties" if style.is_some() => {
                table_cell_properties_begin(attributes, &mut style.unwrap().styles)
            }
            "text:list-level-style-bullet" => {
                level_and_bullet = Some(list_style_bullet_begin(attributes));
            }
            "text:list-level-style-number" => {
                level_and_bullet = Some(list_style_number_begin(attributes));
            }
            "text:list-level-style-image" => {
                level_and_bullet = Some(list_style_image_begin(attributes));
            }
            "style:page-layout-properties" if !self.loaded_page_style => {
                self.loaded_page_style = true;
                page_layout(attributes, &mut self.document_root.styles.page);
            }
            "style:paragraph-properties" => {
                paragraph_properties(attributes, &mut style.unwrap().styles)
            }
            _ => (),
        }
        if let Some((level, bullet)) = level_and_bullet {
            if (1..11).contains(&level) {
                // 1-10 inclusive, probably won't be more than this
                bullet_cycle[(level - 1) as usize] = bullet;
            }
        }
        None
    }

    /// This function may or may not actually utilise the style_name and style attributes depending on the tag name,
    /// if they are not used then they are returned
    fn styles_handle_element_end(
        &mut self,
        name: &str,
        style_name: String,
        style: Option<Style>,
        bullet_cycle: Vec<ListBullet>,
    ) -> (Option<String>, Option<Style>, Option<Vec<ListBullet>>) {
        match name {
            "style:default-style" | "style:style" => {
                if let Some(style) = style {
                    self.document_root.styles.classes.insert(style_name, style);
                    return (None, None, Some(bullet_cycle));
                }
            }
            "text:list-style" => {
                self.in_list_style = false;
                if let Some(mut style) = style {
                    let element = List::new_template(Some(bullet_cycle), None);
                    style.element = Some(Element::List(element));
                    self.document_root.styles.classes.insert(style_name, style);
                    return (None, None, None);
                }
            }
            _ => (),
        }
        (Some(style_name), style, Some(bullet_cycle))
    }

    /// Takes the given tag information and inserts them in the proper format to the given Style struct
    fn styles_handle_element_empty(
        &mut self,
        name: &str,
        attributes: Attributes,
        style: &mut Style,
        bullet_cycle: &mut Vec<ListBullet>,
    ) {
        let mut level_and_bullet: Option<(u32, ListBullet)> = None;
        match name {
            "style:text-properties" if !self.in_list_style => {
                text_properties_begin(attributes, &mut style.styles)
            }
            "style:table-column-properties" => {
                table_column_properties_begin(attributes, &mut style.styles)
            }
            "style:table-cell-properties" => {
                table_cell_properties_begin(attributes, &mut style.styles)
            }
            "style:table-properties" => table_properties_begin(attributes, &mut style.styles),
            "text:list-level-style-bullet" => {
                level_and_bullet = Some(list_style_bullet_begin(attributes));
            }
            "text:list-level-style-number" => {
                level_and_bullet = Some(list_style_number_begin(attributes));
            }
            "text:list-level-style-image" => {
                level_and_bullet = Some(list_style_image_begin(attributes));
            }
            "style:page-layout-properties" if !self.loaded_page_style => {
                self.loaded_page_style = true;
                page_layout(attributes, &mut self.document_root.styles.page);
            }
            "style:paragraph-properties" => paragraph_properties(attributes, &mut style.styles),
            _ => (),
        }
        if let Some((level, bullet)) = level_and_bullet {
            if (1..11).contains(&level) {
                // 1-10 inclusive, probably won't be more than this
                bullet_cycle[(level - 1) as usize] = bullet;
            }
        }
    }
}

/// Takes the set of attributes of a style:style tag,
/// and returns the name of the style, the displayed name of the style and the parent style name
fn style_begin_helper(attributes: Attributes) -> (String, String, String, Option<u32>) {
    let mut display_name = String::new();
    let mut style_name = String::new();
    let mut family = String::new();
    let mut parent_style_name: Option<String> = None;
    let mut default_outline_level: Option<u32> = None;
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            match name {
                "style:name" => {
                    style_name = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("")
                    .to_string();
                }
                "style:family" => {
                    family = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("")
                    .to_string();
                }
                "style:parent-style-name" => {
                    parent_style_name = Some(
                        std::str::from_utf8(
                            &i.unescaped_value()
                                .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                        )
                        .unwrap_or("")
                        .to_string(),
                    );
                }
                "style:display-name" => {
                    display_name = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("")
                    .to_string();
                }
                "style:default-outline-level" => {
                    let outline_level_raw = &i
                        .unescaped_value()
                        .unwrap_or_else(|_| std::borrow::Cow::from(vec![]));
                    let outline_level_str = std::str::from_utf8(outline_level_raw).unwrap_or("");
                    if outline_level_str != "" {
                        default_outline_level = Some(outline_level_str.parse::<u32>().unwrap_or(1));
                    }
                }
                _ => (),
            }
        }
    }
    if let Some(parent_style_name) = parent_style_name {
        (
            style_name,
            display_name,
            parent_style_name,
            default_outline_level,
        )
    } else {
        (style_name, display_name, family, default_outline_level)
    }
}

/// Takes the set of attributes of a style:style tag,
/// and returns the name of the style and the parent style name
/// Note: for use when parsing content.xml
fn style_begin(attributes: Attributes) -> (String, String) {
    let (style_name, _, parent_style_name, _) = style_begin_helper(attributes);
    (style_name, parent_style_name)
}

/// Takes the set of attributes of a style:style tag,
/// and returns the name of the style and the associated style object
/// Note: for use when parsing styles.xml
fn style_style_begin(attributes: Attributes) -> (String, Style) {
    let (style_name, display_name, parent_style_name, default_outline_level) =
        style_begin_helper(attributes);
    let mut element: Option<Element> = None;
    if let Some(default_outline_level) = default_outline_level {
        let heading = Heading::new_template(default_outline_level);
        element = Some(Element::Heading(heading));
    }
    let mut style = Style::new(display_name, Some(parent_style_name));
    style.element = element;
    (style_name, style)
}

/// Helper for handle_element_empty() to respond to tags with "style" prefix
/// local_name here is the name of the tag without the prefix
pub fn handle_element_empty_style(
    local_name: &str,
    attributes: Attributes,
    style: &mut HashMap<String, String>,
    in_list_style: bool,
) {
    match local_name {
        "text-properties" if !in_list_style => text_properties_begin(attributes, style),
        "table-column-properties" => table_column_properties_begin(attributes, style),
        "table-cell-properties" => table_cell_properties_begin(attributes, style),
        "table-properties" => table_properties_begin(attributes, style),
        "paragraph-properties" => paragraph_properties(attributes, style),
        _ => (),
    }
}

/// Helper for handle_element_empty() to handle style tags which aren't prefixed by "style"
/// (currently only list bullets)
pub fn handle_element_empty_style_special(
    name: &str,
    attributes: Attributes,
    bullet_list: &mut Vec<ListBullet>,
) {
    let mut level_and_bullet: (u32, ListBullet) = (
        1,
        ListBullet::Variant(ListBulletVariant::new(
            None,
            None,
            None,
            "filledBullet".to_string(),
        )),
    );
    match name {
        "text:list-level-style-bullet" => level_and_bullet = list_style_bullet_begin(attributes),
        "text:list-level-style-number" => level_and_bullet = list_style_number_begin(attributes),
        "text:list-level-style-image" => level_and_bullet = list_style_image_begin(attributes),
        _ => (),
    }
    let (level, bullet) = level_and_bullet;
    if (1..11).contains(&level) {
        // 1-10 inclusive, probably won't be more than this
        bullet_list[(level - 1) as usize] = bullet;
    }
}

/// Helper for handle_element_start() to respond to tags with "style" prefix
/// local_name here is the name of the tag without the prefix
/// Returns style name, style contents, tuple of list bullet and level info (always None here)
pub fn handle_element_start_style(
    local_name: &str,
    attributes: Attributes,
) -> (
    Option<String>,
    Option<HashMap<String, String>>,
    Option<(u32, ListBullet)>,
) {
    let mut current_style_name: Option<String> = None;
    let mut current_style_value: HashMap<String, String> = HashMap::new();
    let mut is_valid = true;
    match local_name {
        "style" => {
            let (new_style_name, style_parent) = style_begin(attributes);
            current_style_name = Some(new_style_name);
            current_style_value.insert("_parent".to_string(), style_parent);
        }
        "table-row-properties" => table_row_properties_begin(attributes, &mut current_style_value),
        "table-properties" => table_properties_begin(attributes, &mut current_style_value),
        "table-cell-properties" => {
            table_cell_properties_begin(attributes, &mut current_style_value)
        }
        "paragraph-properties" => paragraph_properties(attributes, &mut current_style_value),
        _ => is_valid = false,
    }
    if is_valid {
        (current_style_name, Some(current_style_value), None)
    } else {
        (current_style_name, None, None)
    }
}

/// Takes the set of attributes of a style:default-style tag,
/// and returns the name of the style and the associated style object
fn default_style_begin(attributes: Attributes) -> (String, Style) {
    let mut style_name = String::new();
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            if name == "style:family" {
                style_name = std::str::from_utf8(
                    &i.unescaped_value()
                        .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                )
                .unwrap_or("")
                .to_string();
            }
        }
    }
    // use an empty string as the displayed string for default styles for now
    (style_name, Style::new("".to_string(), None))
}

/// Helper for handle_element_start() to handle style tags which aren't prefixed by "style"
/// Returns style name, style contents (will always be None here) and tuple of list bullet and level info
pub fn handle_element_start_style_special(
    name: &str,
    attributes: Attributes,
) -> (
    Option<String>,
    Option<HashMap<String, String>>,
    Option<(u32, ListBullet)>,
    bool,
) {
    match name {
        "text:list-style" => {
            let (style_name, _) = list_style_begin(attributes); //discard the display name because this is in the context of an automatic style
            (Some(style_name), None, None, true)
        }
        "text:list-level-style-bullet" => {
            (None, None, Some(list_style_bullet_begin(attributes)), false)
        }
        "text:list-level-style-number" => {
            (None, None, Some(list_style_number_begin(attributes)), false)
        }
        "text:list-level-style-image" => {
            (None, None, Some(list_style_image_begin(attributes)), false)
        }
        _ => (None, None, None, false),
    }
}

/// Returns the style name and the display name (if any)
fn list_style_begin(attributes: Attributes) -> (String, Option<String>) {
    let mut style_name = String::new();
    let mut display_name: Option<String> = None;
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            match name {
                "style:name" => {
                    style_name = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("")
                    .to_string();
                }
                "style:display-name" => {
                    display_name = Some(
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
    (style_name, display_name)
}

/// Returns the style name and a Style object containing the displayed name of a text:list-style tag
fn style_list_style_begin(attributes: Attributes) -> (String, Style) {
    let (style_name, display_name_opt) = list_style_begin(attributes);
    let mut display_name = String::new();
    if let Some(x) = display_name_opt {
        display_name = x;
    }
    (style_name, Style::new(display_name, None))
}

/// Handles text:list-level-style-bullet tags, returns the level and bullet
fn list_style_bullet_begin(attributes: Attributes) -> (u32, ListBullet) {
    let mut prefix: Option<String> = None;
    let mut suffix: Option<String> = None;
    let mut level: u32 = 1;
    let mut bullet_char = String::new();

    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            match name {
                "style:num-prefix" => {
                    prefix = Some(
                        std::str::from_utf8(
                            &i.unescaped_value()
                                .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                        )
                        .unwrap_or("")
                        .to_string(),
                    );
                }
                "style:num-suffix" => {
                    suffix = Some(
                        std::str::from_utf8(
                            &i.unescaped_value()
                                .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                        )
                        .unwrap_or("")
                        .to_string(),
                    );
                }
                "text:level" => {
                    level = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("1")
                    .parse::<u32>()
                    .unwrap_or(1);
                }
                "text:bullet-char" => {
                    bullet_char = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("")
                    .to_string();
                }
                _ => (),
            }
        }
    }

    let bullet = ListBulletCharacter::new(prefix, suffix, bullet_char);
    (level, ListBullet::Character(bullet))
}

/// Handles text:list-level-style-number tags, returns the level and bullet
fn list_style_number_begin(attributes: Attributes) -> (u32, ListBullet) {
    let mut prefix: Option<String> = None;
    let mut suffix: Option<String> = None;
    let mut level: u32 = 1;
    let mut start_value: Option<u32> = None;
    let mut variant = String::new();
    let mut is_number = false;

    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            match name {
                "style:num-prefix" => {
                    prefix = Some(
                        std::str::from_utf8(
                            &i.unescaped_value()
                                .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                        )
                        .unwrap_or("")
                        .to_string(),
                    );
                }
                "style:num-suffix" => {
                    suffix = Some(
                        std::str::from_utf8(
                            &i.unescaped_value()
                                .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                        )
                        .unwrap_or("")
                        .to_string(),
                    );
                }
                "text:level" => {
                    level = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("1")
                    .parse::<u32>()
                    .unwrap_or(1);
                }
                "style:num-format" => {
                    let format = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("")
                    .to_string();
                    let (variant_new, is_number_new) =
                        list_style_number_begin_helper(format.as_str());
                    variant = variant_new;
                    is_number = is_number_new;
                }
                "text:start-value" => {
                    start_value = Some(
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

    if is_number {
        let bullet = ListBulletVariant::new(prefix, suffix, start_value, variant);
        (level, ListBullet::Variant(bullet))
    } else {
        let bullet = ListBulletCharacter::new(prefix, suffix, variant);
        (level, ListBullet::Character(bullet))
    }
}

/// Converts ODT number format to KDF numbering variant
fn list_style_number_begin_helper(format: &str) -> (String, bool) {
    let mut is_number = true;
    let variant;
    match format {
        "1" => variant = "decimal".to_string(),
        "a" => variant = "lowerLatin".to_string(),
        "A" => variant = "upperLatin".to_string(),
        "i" => variant = "lowerRoman".to_string(),
        "I" => variant = "upperRoman".to_string(),
        _ => {
            is_number = false;
            variant = format.to_string(); // in case it's none of the above (ODT allows any string)
        }
    }
    (variant, is_number)
}

/// Handles text:list-level-style-image tags, returns the level and bullet
fn list_style_image_begin(attributes: Attributes) -> (u32, ListBullet) {
    let mut href = String::new();
    let mut level: u32 = 1;

    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            match name {
                "text:level" => {
                    level = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("1")
                    .parse::<u32>()
                    .unwrap_or(1);
                }
                "xlink:href" => {
                    href = std::str::from_utf8(
                        &i.unescaped_value()
                            .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                    )
                    .unwrap_or("")
                    .to_string();
                }
                _ => (),
            }
        }
    }

    let bullet = ListBulletImage::new(None, None, href);
    (level, ListBullet::Image(bullet))
}

/// Handles a style:page-layout-properties tag
fn page_layout(attributes: Attributes, page_style: &mut HashMap<String, String>) {
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
                // There are a number of attributes prefixed with "style", but none are supported
                // by KDF at the moment
                page_layout_fo(local_name, value, page_style);
            }
        }
    }
}

/// Handle attributes common to block elements (like page layout and paragraphs for example)
fn general_block_properties(local_name: &str, value: String, styles: &mut HashMap<String, String>) {
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
        "margin" => {
            styles.insert("margin".to_string(), value);
        }
        "margin-left" => {
            styles.insert("marginLeft".to_string(), value);
        }
        "margin-right" => {
            styles.insert("marginRight".to_string(), value);
        }
        "margin-top" => {
            styles.insert("marginTop".to_string(), value);
        }
        "margin-bottom" => {
            styles.insert("marginBottom".to_string(), value);
        }
        _ => (),
    }
}

/// Helper for page_layout() to handle attributes with "fo" prefix
fn page_layout_fo(local_name: &str, value: String, page_style: &mut HashMap<String, String>) {
    match local_name {
        "page-height" => {
            page_style.insert("height".to_string(), value);
        }
        "page-width" => {
            page_style.insert("width".to_string(), value);
        }
        _ => general_block_properties(local_name, value, page_style),
    };
}

/// Handles a "style:paragraph-properties" tag
fn paragraph_properties(attributes: Attributes, styles: &mut HashMap<String, String>) {
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
            match prefix {
                "fo" => paragraph_properties_fo(local_name, value, styles),
                "style" => paragraph_properties_style(local_name, value, styles),
                _ => (),
            }
        }
    }
    if let Some(mut bg_alpha) = styles.remove("_bgAlpha") {
        bg_alpha.pop(); // Remove the percent sign from the end
        let mut bg_alpha = 100.0 - bg_alpha.parse::<f64>().unwrap_or(0.0); // Assuming 100% transparency means 0% alpha (can't test this, because LO doesn't even use it)
        bg_alpha = bg_alpha / 100.0 * 255.0;
        if let Some(mut bg) = styles.remove("backgroundColor") {
            bg = format!("{}{:X}", bg, bg_alpha as u32); // Append the alpha as a hex value to the original background
            styles.insert("backgroundColor".to_string(), bg);
        }
    }
}

/// Helper for paragraph_properties() to handle attributes with "fo" prefix
fn paragraph_properties_fo(local_name: &str, value: String, styles: &mut HashMap<String, String>) {
    match local_name {
        "break-after" if value == "page" => {
            // Will be converted to the node later
            styles.insert("_pageBreakAfter".to_string(), "true".to_string());
        }
        "break-before" if value == "page" => {
            styles.insert("_pageBreakBefore".to_string(), "true".to_string());
        }
        "line-height" => {
            styles.insert("lineHeight".to_string(), value);
        }
        "orphans" => {
            styles.insert("orphans".to_string(), value);
        }
        "text-align" => {
            styles.insert("textAlign".to_string(), value);
        }
        "text-align-last" => {
            styles.insert("textAlignLast".to_string(), value);
        }
        "text-indent" => {
            styles.insert("textIndent".to_string(), value);
        }
        "widows" => {
            styles.insert("widows".to_string(), value);
        }
        _ => general_block_properties(local_name, value, styles),
    }
}

/// Helper for paragraph_properties() to handle attributes with "style" prefix
fn paragraph_properties_style(
    local_name: &str,
    value: String,
    styles: &mut HashMap<String, String>,
) {
    match local_name {
        "background-transparency" => {
            styles.insert("_bgAlpha".to_string(), value);
        }
        "shadow" => {
            styles.insert("boxShadow".to_string(), value);
        }
        "vertical-align" if value != "auto" => {
            styles.insert("verticalAlign".to_string(), value);
        }
        _ => (),
    }
}
