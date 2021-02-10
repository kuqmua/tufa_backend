use crate::config::ENABLE_ERROR_PRINTS_MEDRXIV;
use crate::config::ENABLE_PRINTS_MEDRXIV;
use crate::fetch::metainfo_fetch_structures::AreThereItems;
use crate::fetch::rxiv_structures::RxivPost;
use crate::fetch::rxiv_structures::RxivPostStruct;
use crate::fetch::rxiv_structures::XmlRxivParserStruct;
use serde_xml_rs::from_str;

pub fn medrxiv_parse_string_into_struct(
    mut fetch_tuple_result: String,
    key: &str,
    value: &str,
) -> (RxivPostStruct, AreThereItems) {
    let mut medrxiv_post_struct_handle: RxivPostStruct = RxivPostStruct::new();
    let are_there_items_handle: AreThereItems; // = AreThereItems::Initialized
    fetch_tuple_result.remove(0);
    while fetch_tuple_result.find("<dc:title>").is_some() {
        match fetch_tuple_result.find("</dc:title>") {
            Some(_) => {
                fetch_tuple_result = fetch_tuple_result.replace("<dc:title>", "<dcstitle>");
                fetch_tuple_result = fetch_tuple_result.replace("</dc:title>", "</dcstitle>");
            }
            None => {
                break;
            }
        }
    }
    match fetch_tuple_result.find("</item>") {
        Some(_) => {
            let medrxiv_struct_from_str_result: Result<XmlRxivParserStruct, serde_xml_rs::Error> =
                from_str(&fetch_tuple_result);
            match medrxiv_struct_from_str_result {
                Ok(medrxiv_struct) => {
                    let mut count = 0;
                    let mut medrxiv_page_struct: RxivPostStruct = RxivPostStruct::new();
                    loop {
                        if count < medrxiv_struct.items.len() {
                            let mut medrxiv_post: RxivPost = RxivPost::new();
                            medrxiv_post.title = medrxiv_struct.items[count].title.clone();
                            medrxiv_post.link = medrxiv_struct.items[count].link.clone();
                            medrxiv_post.description =
                                medrxiv_struct.items[count].description.clone();
                            medrxiv_page_struct.items.push(medrxiv_post);
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    if !medrxiv_page_struct.items.is_empty() {
                        are_there_items_handle = AreThereItems::Yep;
                    } else {
                        are_there_items_handle =
                            AreThereItems::NopeButThereIsTag(fetch_tuple_result);
                    }
                    medrxiv_post_struct_handle = medrxiv_page_struct;
                }
                Err(e) => {
                    if ENABLE_ERROR_PRINTS_MEDRXIV {
                        println!(
                            "medrxiv conversion from str for {}, error {}",
                            key,
                            e.to_string()
                        );
                    };
                    are_there_items_handle =
                        AreThereItems::ConversionFromStrError(fetch_tuple_result, e.to_string());
                }
            }
        }
        _ => {
            if ENABLE_PRINTS_MEDRXIV {
                println!("medrxiv no items for key {} {}", key, value);
            };
            are_there_items_handle = AreThereItems::NopeNoTag(fetch_tuple_result);
        }
    }
    (medrxiv_post_struct_handle, are_there_items_handle)
}
