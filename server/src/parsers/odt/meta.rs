use super::*;

impl ODTParser {
    pub fn parse_meta(
        &mut self,
        archive: &mut zip::ZipArchive<std::fs::File>,
    ) -> Result<(), String> {
        // returns a ZipFile struct which implements Read if the file is in the archive
        let meta_xml = archive.by_name("meta.xml");
        if let Err(e) = meta_xml {
            // Handle case where there is no meta.xml (so probably not actually an ODT file)
            return Err(e.to_string());
        }
        let meta_xml = BufReader::new(meta_xml.unwrap()); //add buffering because quick-xml's reader requires it

        let mut current_meta_type = MetaType::Unknown;
        let mut current_meta_value = String::new();
        let mut current_tag_name = String::new();

        let mut title = String::new();
        let mut authors: Vec<String> = Vec::new();
        let mut created_at = String::new();
        let mut updated_at = String::new();
        let mut edit_duration = String::new();
        let mut additional: HashMap<String, String> = HashMap::new();

        let mut parser = Reader::from_reader(meta_xml);
        let mut buffer = Vec::new();
        loop {
            match parser.read_event(&mut buffer) {
                Ok(Event::Start(contents)) => {
                    let name = std::str::from_utf8(contents.name()).unwrap_or(":");
                    let tag = meta_handle_element_start(name, contents.attributes());
                    match tag {
                        MetaType::Unknown => (),
                        _ => {
                            current_meta_type = tag;
                            current_tag_name = name.to_string();
                        }
                    }
                }
                Ok(Event::Text(contents)) => {
                    let contents = contents.unescape_and_decode(&parser);
                    if let Err(e) = contents {
                        println!("Metadata parsing error: {}", e);
                    } else {
                        current_meta_value = contents.unwrap();
                    }
                }
                Ok(Event::End(contents)) => {
                    if current_tag_name == std::str::from_utf8(contents.name()).unwrap_or(":") {
                        match current_meta_type {
                            MetaType::Title => title = current_meta_value,
                            MetaType::Author if !authors.contains(&current_meta_value) => {
                                authors.push(current_meta_value)
                            }
                            MetaType::CreatedAt => created_at = current_meta_value,
                            MetaType::UpdatedAt => updated_at = current_meta_value,
                            MetaType::EditDuration => edit_duration = current_meta_value,
                            MetaType::Custom(name) => {
                                additional.insert(name, current_meta_value);
                            }
                            _ => (),
                        }
                        current_meta_type = MetaType::Unknown;
                        current_meta_value = String::new();
                        current_tag_name = String::new();
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    println!("Metadata parsing error: {}", e);
                    return Err(e.to_string());
                }
                _ => (),
            }
        }

        let mut meta = Meta::new(title, authors, created_at, updated_at, edit_duration);
        meta.additional = additional;
        self.document_root.meta = Some(meta);

        Ok(())
    }
}

/// Takes the set of attributes of a meta:user-defined tag and returns the property name
fn meta_user_defined_begin(attributes: Attributes) -> String {
    let mut output_name = String::new();
    for i in attributes {
        if let Ok(i) = i {
            let name = std::str::from_utf8(i.key).unwrap_or(":");
            if name == "meta:name" {
                output_name = std::str::from_utf8(
                    &i.unescaped_value()
                        .unwrap_or_else(|_| std::borrow::Cow::from(vec![])),
                )
                .unwrap_or("")
                .to_string();
            }
        }
    }
    output_name
}

/// Handle a tag start event in meta.xml
fn meta_handle_element_start(name: &str, attributes: Attributes) -> MetaType {
    match name {
        "dc:date" => MetaType::UpdatedAt,
        "dc:title" => MetaType::Title,
        "meta:editing-duration" => MetaType::EditDuration,
        "meta:creation-date" => MetaType::CreatedAt,
        "dc:description" => MetaType::Custom("description".to_string()),
        "dc:language" => MetaType::Custom("language".to_string()),
        "dc:subject" => MetaType::Custom("subject".to_string()),
        "meta:editing-cycles" => MetaType::Custom("editingCycles".to_string()),
        "meta:generator" => MetaType::Custom("generator".to_string()),
        "meta:keyword" => MetaType::Custom("keyword".to_string()),
        "meta:print-date" => MetaType::Custom("printDate".to_string()),
        "meta:printed-by" => MetaType::Custom("printedBy".to_string()),
        "meta:initial-creator" => MetaType::Author,
        "dc:creator" => MetaType::Author,
        "meta:user-defined" => MetaType::Custom(meta_user_defined_begin(attributes)),
        _ => MetaType::Unknown,
    }
}
