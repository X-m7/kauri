extern crate serde_json;
extern crate tiny_http;
extern crate xml;
extern crate zip;

use serde_json::map::Map;
use serde_json::value::Value;
use serde_json::Number;
use std::fs;
use std::io;
use xml::reader::{EventReader, XmlEvent};

fn main() {
    let addr = "127.0.0.1:3000";
    let server = tiny_http::Server::http(addr).unwrap();
    println!("Listening on http://{}", addr);

    loop {
        let mut request = match server.recv() {
            //server.recv() blocks until a request actually comes
            Ok(rq) => rq,
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        };
        match request.url() {
            //check the URL and respond accordingly
            "/load" => {
                let req_reader = request.as_reader();
                let mut body_bytes: Vec<u8> = Vec::new();
                if let Err(e) = req_reader.read_to_end(&mut body_bytes) {
                    println!("error: {}", e);
                    continue;
                }
                let body_str = std::str::from_utf8(&body_bytes);
                if let Err(e) = body_str {
                    println!("error: {}", e);
                    continue;
                }
                let response = tiny_http::Response::from_string(read_odt(body_str.unwrap()));
                if let Err(e) = request.respond(response) {
                    println!("error: {}", e);
                    continue;
                }
            }
            _ => {
                let response = tiny_http::Response::empty(404);
                if let Err(e) = request.respond(response) {
                    println!("error: {}", e);
                    continue;
                }
            }
        }
    }
}

/// Reads an ODT file referred to by the given path
/// and returns a JSON string containing a DOM
fn read_odt(filepath: &str) -> String {
    let file = std::path::Path::new(&filepath);
    if !file.exists() {
        //make sure the file actually exists
        println!("{:?}", fs::metadata(file));
        return serde_json::to_string(&Value::Null).unwrap();
    }

    let file = fs::File::open(&file).unwrap();
    let archive = zip::ZipArchive::new(file);
    if let Err(e) = archive {
        //handle case where the file is not even a zip file
        println!("{}", e);
        return serde_json::to_string(&Value::Null).unwrap();
    }
    let mut archive = archive.unwrap();
    let content_xml = archive.by_name("content.xml"); //returns a ZipFile struct which implements Read if the file is in the archive
    if let Err(e) = content_xml {
        //handle case where there is no content.xml (so probably not actually an ODT file)
        println!("{}", e);
        return serde_json::to_string(&Value::Null).unwrap();
    }
    let content_xml = io::BufReader::new(content_xml.unwrap());

    let parser = EventReader::new(content_xml);
    let mut begin = false; //used to ignore all the other tags before office:body for now
    let mut document_contents: Map<String, Value> = Map::new(); //value of the "document" key
    document_contents.insert(
        "title".to_string(),
        Value::String("Kauri (Working title)".to_string()),
    );
    document_contents.insert("paper".to_string(), Value::String("A4".to_string()));
    document_contents.insert("children".to_string(), Value::Array(Vec::new()));
    let mut document_hierarchy: Vec<Value> = Vec::new(); //in case of nested tags, not actually handled yet
    let mut current_value = Value::Null;
    document_hierarchy.push(Value::Object(document_contents));

    for e in parser {
        //iterate through the XML
        match e {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if let Some(prefix) = name.prefix {
                    if prefix == "office" && name.local_name == "body" {
                        begin = true;
                    } else if begin {
                        if prefix == "text" && name.local_name == "h" {
                            //heading
                            let mut level = 0.0; //because JS numbers are always floats apparently
                            for i in attributes {
                                //attributes is a Vec, so need to search for the level
                                if i.name.prefix.unwrap() == "text"
                                    && i.name.local_name == "outline-level"
                                {
                                    level = i.value.parse::<f64>().unwrap();
                                }
                            }
                            let mut map: Map<String, Value> = Map::new();
                            map.insert("type".to_string(), Value::String("heading".to_string()));
                            map.insert(
                                "level".to_string(),
                                Value::Number(Number::from_f64(level).unwrap()),
                            );
                            map.insert("children".to_string(), Value::Array(Vec::new()));
                            current_value = Value::Object(map);
                        } else if prefix == "text" && name.local_name == "p" {
                            let mut map: Map<String, Value> = Map::new();
                            map.insert("type".to_string(), Value::String("paragraph".to_string()));
                            map.insert("children".to_string(), Value::Array(Vec::new()));
                            current_value = Value::Object(map);
                        }
                    }
                }
            }
            Ok(XmlEvent::Characters(contents)) => {
                //the contents of a tag
                let mut map: Map<String, Value> = Map::new();
                map.insert("type".to_string(), Value::String("text".to_string()));
                map.insert("content".to_string(), Value::String(contents));
                current_value
                    .as_object_mut()
                    .unwrap()
                    .get_mut("children")
                    .unwrap()
                    .as_array_mut()
                    .unwrap()
                    .push(Value::Object(map));
            }
            Ok(XmlEvent::EndElement { name }) => {
                if begin {
                    if let Some(prefix) = name.prefix {
                        if prefix == "office" && name.local_name == "body" {
                            break;
                        } else if prefix == "text"
                            && (name.local_name == "h" || name.local_name == "p")
                        {
                            document_hierarchy
                                .last_mut()
                                .unwrap()
                                .as_object_mut()
                                .unwrap()
                                .get_mut("children")
                                .unwrap()
                                .as_array_mut()
                                .unwrap()
                                .push(current_value);
                            current_value = Value::Null;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                return serde_json::to_string(&Value::Null).unwrap();
            }
            _ => {}
        }
    }

    let mut document_object: Map<String, Value> = Map::new();
    document_object.insert("document".to_string(), document_hierarchy.pop().unwrap());
    let document_object = Value::Object(document_object);
    serde_json::to_string(&document_object).unwrap()
}