use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Text {
    content: String,
}

impl Text {
    /// Constructs a new text node.
    ///
    ///  - `content`: Internal text content.
    pub fn new(content: String) -> Text {
        Text { content }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(untagged)]
pub enum ChildNode {
    Node(Node),
    ShortHandText(String),
    Element(Element),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Node {
    Text(Text),
    LineBreak,
    PageBreak,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Element {
    Heading(Heading),
    Paragraph(ElementCommon),
    Span(ElementCommon),
    List(List),
    ListItem(ListItem),
    Caption(ElementCommon),
    Hyperlink(Hyperlink),
    Table(ElementCommon),
    TableRow(ElementCommon),
    TableColumnGroup(ElementCommon),
    TableColumn(TableColumn),
    TableCell(TableCell),
    Code(ElementCommon),
    CodeBlock(CodeBlock),
    Hint(Hint),
    BlockQuote(ElementCommon),
    BlockQuoteAttribution(ElementCommon),
}

impl Element {
    /// Returns the ElementCommon struct contained in the given Element enum
    pub fn get_common(&mut self) -> &mut ElementCommon {
        match self {
            Element::Paragraph(common)
            | Element::Span(common)
            | Element::Caption(common)
            | Element::Table(common)
            | Element::TableRow(common)
            | Element::TableColumnGroup(common)
            | Element::Code(common)
            | Element::BlockQuote(common)
            | Element::BlockQuoteAttribution(common) => common,
            Element::Heading(heading) => &mut heading.common,
            Element::List(list) => &mut list.common,
            Element::ListItem(list_item) => &mut list_item.common,
            Element::Hyperlink(hyperlink) => &mut hyperlink.common,
            Element::TableColumn(table_column) => &mut table_column.common,
            Element::TableCell(table_cell) => &mut table_cell.common,
            Element::CodeBlock(code_block) => &mut code_block.common,
            Element::Hint(hint) => &mut hint.common,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ElementCommon {
    #[serde(skip_serializing_if = "Option::is_none")]
    class: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub styles: HashMap<String, String>,
    // Children is optional here because this may be used as a template in a style class, which would not have it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ChildNode>>,
    #[serde(skip)]
    // This is meant to store attributes that can only be processed after all of this Element's children has been accounted for,
    // so this should not be part of the JSON
    pub attributes: HashMap<String, String>,
}

impl ElementCommon {
    /// Constructs a new ElementCommon struct
    ///
    /// - `class` Style class of the element.
    pub fn new(class: Option<String>) -> ElementCommon {
        ElementCommon {
            class,
            styles: HashMap::new(),
            children: Some(Vec::new()),
            attributes: HashMap::new(),
        }
    }

    /// Constructs a new ElementCommon struct meant to be used as a template for a style class
    pub fn new_template() -> ElementCommon {
        ElementCommon {
            class: None,
            styles: HashMap::new(),
            children: None,
            attributes: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Heading {
    #[serde(flatten)]
    pub common: ElementCommon,
    level: u32,
}

impl Heading {
    /// Constructs a new Heading element
    ///
    /// - `class` Style class of the element.
    /// - `level` Level of the heading.
    pub fn new(class: Option<String>, level: u32) -> Heading {
        Heading {
            common: ElementCommon::new(class),
            level,
        }
    }

    /// Constructs a new Heading element for use as a template for a style class
    ///
    /// - `level` Level of the heading
    pub fn new_template(level: u32) -> Heading {
        Heading {
            common: ElementCommon::new_template(),
            level,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct List {
    #[serde(flatten)]
    pub common: ElementCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    bullet_cycle: Option<Vec<ListBullet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bullet: Option<ListBullet>,
}

impl List {
    /// Constructs a new List element
    ///
    /// - `class` Style class of the element.
    /// - `bullet_cycle` Cycle of the bullets.
    /// - `bullet` Type of the list bullet.
    pub fn new(
        class: Option<String>,
        bullet_cycle: Option<Vec<ListBullet>>,
        bullet: Option<ListBullet>,
    ) -> List {
        List {
            common: ElementCommon::new(class),
            bullet_cycle,
            bullet,
        }
    }

    /// Constructs a new List element for use as a template in a style class
    ///
    /// - `bullet_cycle` Cycle of the bullets.
    /// - `bullet` Type of the list bullet.
    pub fn new_template(bullet_cycle: Option<Vec<ListBullet>>, bullet: Option<ListBullet>) -> List {
        List {
            common: ElementCommon::new_template(),
            bullet_cycle,
            bullet,
        }
    }

    /// Returns a reference to the bullet cycle (if any)
    pub fn get_bullet_cycle(&self) -> &Option<Vec<ListBullet>> {
        &self.bullet_cycle
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
struct ListBulletCommon {
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<String>,
}

impl ListBulletCommon {
    /// Constructs a new ListBulletCommon struct
    ///
    /// - `prefix` Prefix of the bullet.
    /// - `suffix` Suffix of the bullet.
    fn new(prefix: Option<String>, suffix: Option<String>) -> ListBulletCommon {
        ListBulletCommon { prefix, suffix }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct ListBulletVariant {
    #[serde(flatten)]
    common: ListBulletCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_index: Option<u32>,
    variant: String,
}

impl ListBulletVariant {
    /// Constructs a new ListBulletVariant struct
    ///
    /// - `prefix` Prefix of the bullet.
    /// - `suffix` Suffix of the bullet.
    /// - `start_index` Where numbering for an ordered list should begin.
    /// - `variant` Variant of the bullet.
    pub fn new(
        prefix: Option<String>,
        suffix: Option<String>,
        start_index: Option<u32>,
        variant: String,
    ) -> ListBulletVariant {
        ListBulletVariant {
            common: ListBulletCommon::new(prefix, suffix),
            start_index,
            variant,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ListBulletCharacter {
    #[serde(flatten)]
    common: ListBulletCommon,
    #[serde(rename = "char")]
    character: String,
}

impl ListBulletCharacter {
    /// Constructs a new ListBulletString struct
    ///
    /// - `prefix` Prefix of the bullet.
    /// - `suffix` Suffix of the bullet.
    /// - `character` The bullet character.
    pub fn new(
        prefix: Option<String>,
        suffix: Option<String>,
        character: String,
    ) -> ListBulletCharacter {
        ListBulletCharacter {
            common: ListBulletCommon::new(prefix, suffix),
            character,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ListBulletImage {
    #[serde(flatten)]
    common: ListBulletCommon,
    image: String,
}

impl ListBulletImage {
    /// Constructs a new ListBulletImage struct
    ///
    /// - `prefix` Prefix of the bullet.
    /// - `suffix` Suffix of the bullet.
    /// - `image` Image resource URL of the bullet.
    pub fn new(prefix: Option<String>, suffix: Option<String>, image: String) -> ListBulletImage {
        ListBulletImage {
            common: ListBulletCommon::new(prefix, suffix),
            image,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(untagged)]
pub enum ListBullet {
    Variant(ListBulletVariant),
    Character(ListBulletCharacter),
    Image(ListBulletImage),
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ListItem {
    #[serde(flatten)]
    pub common: ElementCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    bullet: Option<ListBullet>,
}

impl ListItem {
    /// Constructs a new ListItem element
    ///
    /// - `class` Style class of the element.
    /// - `bullet` Type of the list bullet.
    pub fn new(class: Option<String>, bullet: Option<ListBullet>) -> ListItem {
        ListItem {
            common: ElementCommon::new(class),
            bullet,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Hyperlink {
    #[serde(flatten)]
    pub common: ElementCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    href: String,
}

impl Hyperlink {
    /// Constructs a new Hyperlink element
    ///
    /// - `class` Style class of the element.
    /// - `title` Title of the hyperlink.
    /// - `href` Hyperlink reference.
    pub fn new(class: Option<String>, title: Option<String>, href: String) -> Hyperlink {
        Hyperlink {
            common: ElementCommon::new(class),
            title,
            href,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct TableColumn {
    #[serde(flatten)]
    pub common: ElementCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    span: Option<u32>,
}

impl TableColumn {
    /// Constructs a new TableColumn element
    ///
    /// - `class` Style class of the element.
    /// - `span` Number of columns to span.
    pub fn new(class: Option<String>, span: Option<u32>) -> TableColumn {
        TableColumn {
            common: ElementCommon::new(class),
            span,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct TableCell {
    #[serde(flatten)]
    pub common: ElementCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    row_span: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    col_span: Option<u32>,
}

impl TableCell {
    /// Constructs a new TableCell element
    ///
    /// - `class` Style class of the element.
    /// - `row_span` Number of rows to span.
    /// - `col_span` Number of columns to span.
    pub fn new(class: Option<String>, row_span: Option<u32>, col_span: Option<u32>) -> TableCell {
        TableCell {
            common: ElementCommon::new(class),
            row_span,
            col_span,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[serde(rename_all = "camelCase")]
pub struct CodeBlock {
    #[serde(flatten)]
    pub common: ElementCommon,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    line_numbers: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_name: Option<String>,
}

impl CodeBlock {
    /// Constructs a new CodeBlock element
    ///
    /// - `class` Style class of the element.
    /// - `language` Language of the code inside the block.
    /// - `line_numbers` Indicates whether to show line numbers.
    /// - `file_name` The displayed file name of the code block.
    pub fn new(
        class: Option<String>,
        language: Option<String>,
        line_numbers: bool,
        file_name: Option<String>,
    ) -> CodeBlock {
        CodeBlock {
            common: ElementCommon::new(class),
            language,
            line_numbers,
            file_name,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Hint {
    #[serde(flatten)]
    pub common: ElementCommon,
    variant: HintVariant,
}

impl Hint {
    /// Constructs a new CodeBlock element
    ///
    /// - `class` Style class of the element.
    /// - `variant` Variant of the hint element.
    pub fn new(class: Option<String>, variant: HintVariant) -> Hint {
        Hint {
            common: ElementCommon::new(class),
            variant,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum HintVariant {
    Information,
    Success,
    Warning,
    Error,
}
