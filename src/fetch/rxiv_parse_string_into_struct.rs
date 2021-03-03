use crate::fetch::metainfo_fetch_structures::AreThereItems;
use crate::fetch::rxiv_kind_enum::RxivKind;
use crate::fetch::rxiv_structures::RxivPost;
use crate::fetch::rxiv_structures::RxivPostStruct;
use crate::fetch::rxiv_structures::XmlRxivParserStruct;
use crate::overriding::prints::print_error_red;
use crate::overriding::prints::print_warning_yellow;
use serde_xml_rs::from_str;

pub fn rxiv_parse_string_into_struct(
    mut fetch_result_string: String,
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
        fetch_result_string.remove(0);
    }
    if let RxivKind::Arxiv = rxiv_kind {
    } else {
        while fetch_result_string.contains("<dc:title>") {
            match fetch_result_string.find("</dc:title>") {
                Some(_) => {
                    fetch_result_string = fetch_result_string.replace("<dc:title>", "<dcstitle>");
                    fetch_result_string = fetch_result_string.replace("</dc:title>", "</dcstitle>");
                }
                None => {
                    break;
                }
            }
        }
    }

    match fetch_result_string.find("</item>") {
        Some(_) => {
            let rxiv_struct_from_str_result: Result<XmlRxivParserStruct, serde_xml_rs::Error> =
                from_str(&fetch_result_string);
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
                            AreThereItems::NopeButThereIsTag(fetch_result_string);
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
                        AreThereItems::ConversionFromStrError(fetch_result_string, e.to_string());
                }
            }
        }
        _ => {
            if enable_prints {
                let warning: String = "wrong url or there is no items for key: ".to_string()
                    + key
                    + " link: "
                    + value; //разделить логику при помощи нахождения паттерна архива урла
                print_warning_yellow(file!().to_string(), line!().to_string(), warning);
            };
            are_there_items_handle = AreThereItems::NopeNoTag(fetch_result_string);
        }
    }
    (rxiv_post_struct_handle, are_there_items_handle)
}
