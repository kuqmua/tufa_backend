/*
#[path = "providers/initialization/reddit_part.rs"]
mod reddit_part;
use reddit_part::reddit_part;

fn main() {
    reddit_part();
}
*/

extern crate reqwest;
//extern crate serde_json;
extern crate xml;
/*
use std::collections::HashMap;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};
*/
use quick_xml::events::Event;
use quick_xml::Reader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://export.arxiv.org/rss/cs")
        .await?
        .text()
        .await?;
    let xml: String = resp;
    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);

    let mut count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"tag1" => println!(
                    "attributes values: {:?}",
                    e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                ),
                b"tag2" => count += 1,
                _ => (),
            },
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    println!("{:#?}", txt);

    Ok(())
}
/*
fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

fn main() {
    let file = BufReader::new(file);/test

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{}+{}", indent(depth), name);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
*/
