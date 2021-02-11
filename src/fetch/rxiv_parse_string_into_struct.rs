use crate::fetch::metainfo_fetch_structures::AreThereItems;
use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::fetch::rxiv_structures::RxivPost;
use crate::fetch::rxiv_structures::RxivPostStruct;
use crate::fetch::rxiv_structures::XmlRxivParserStruct;
use crate::overriding::prints::print_error_red;
use serde_xml_rs::from_str;

pub fn rxiv_parse_string_into_struct(
    mut fetch_tuple_result: String,
    key: &str,
    value: &str,
    enable_prints: bool,
    enable_error_prints: bool,
    rxiv_kind: RxivKind,
) -> (RxivPostStruct, AreThereItems) {
    let mut rxiv_post_struct_handle: RxivPostStruct = RxivPostStruct::new();
    let are_there_items_handle: AreThereItems; // = AreThereItems::Initialized
                                               // println!("{:#?}", rxiv_kind);
    if let RxivKind::Medrxiv = rxiv_kind {
        fetch_tuple_result.remove(0);
    }
    if let RxivKind::Arxiv = rxiv_kind {
    } else {
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
    }

    match fetch_tuple_result.find("</item>") {
        Some(_) => {
            let rxiv_struct_from_str_result: Result<XmlRxivParserStruct, serde_xml_rs::Error> =
                from_str(&fetch_tuple_result);
            match rxiv_struct_from_str_result {
                Ok(rxiv_struct) => {
                    let mut count = 0;
                    let mut rxiv_page_struct: RxivPostStruct = RxivPostStruct::new();
                    loop {
                        if count < rxiv_struct.items.len() {
                            let mut rxiv_post: RxivPost = RxivPost::new();
                            rxiv_post.title = rxiv_struct.items[count].title.clone();
                            rxiv_post.link = rxiv_struct.items[count].link.clone();
                            rxiv_post.description = rxiv_struct.items[count].description.clone();
                            rxiv_page_struct.items.push(rxiv_post);
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    if !rxiv_page_struct.items.is_empty() {
                        are_there_items_handle = AreThereItems::Yep;
                    } else {
                        are_there_items_handle =
                            AreThereItems::NopeButThereIsTag(fetch_tuple_result);
                    }
                    rxiv_post_struct_handle = rxiv_page_struct;
                }
                Err(e) => {
                    if enable_error_prints {
                        let error = "rxiv conversion from str for ".to_string()
                            + key
                            + "error: "
                            + &e.to_string();
                        print_error_red(file!().to_string(), line!().to_string(), error)
                    };
                    are_there_items_handle =
                        AreThereItems::ConversionFromStrError(fetch_tuple_result, e.to_string());
                }
            }
        }
        _ => {
            if enable_prints {
                println!("rxiv no items for key {} {}", key, value);
            };
            are_there_items_handle = AreThereItems::NopeNoTag(fetch_tuple_result);
        }
    }
    (rxiv_post_struct_handle, are_there_items_handle)
}
