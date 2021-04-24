use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_provider_kind_enum::ProviderKind;
use crate::fetch::rss_structures::RxivPost;
use crate::fetch::rss_structures::RxivPostStruct;
use crate::fetch::rss_structures::XmlRxivParserStruct;
use crate::overriding::prints::print_error_red;
use crate::overriding::prints::print_warning_yellow;
use serde_xml_rs::from_str;

pub fn rss_parse_string_into_struct(
    mut fetch_result_string: String,
    key: &str,
    value: &str,
    enable_error_prints: bool,
    provider_kind: ProviderKind,
) -> (RxivPostStruct, AreThereItems) {
    let mut rxiv_post_struct_handle: RxivPostStruct = RxivPostStruct::new();
    let are_there_items_handle: AreThereItems; // = AreThereItems::Initialized
    if let ProviderKind::Twitter = provider_kind {
        match fetch_result_string.find("<channel>") {
            Some(find_item_position_start) => match fetch_result_string.find("</channel>") {
                Some(find_item_position_end) => {
                    fetch_result_string = fetch_result_string
                        [find_item_position_start..find_item_position_end + "</channel>".len()]
                        .to_string();
                }
                _ => {
                    let warning_message: String =
                        format!("no </channel> in response for key: {} link: {}", key, value);
                    print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
                }
            },
            _ => {
                let warning_message: String =
                    format!("no <channel> in response for key: {} link: {}", key, value);
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
            }
        }
    }

    match fetch_result_string.find("</item>") {
        Some(_) => {
            if let ProviderKind::Twitter = provider_kind {
                while fetch_result_string.contains("<dc:creator>") {
                    match fetch_result_string.find("</dc:creator>") {
                        Some(_) => {
                            fetch_result_string =
                                fetch_result_string.replace("<dc:creator>", "<creator>");
                            fetch_result_string =
                                fetch_result_string.replace("</dc:creator>", "</creator>");
                        }
                        None => {
                            break;
                        }
                    }
                }
                while fetch_result_string.contains("<atom:link") {
                    fetch_result_string = fetch_result_string.replace("<atom:link", "<atom_link");
                }
            }
            if let ProviderKind::Medrxiv = provider_kind {
                fetch_result_string.remove(0);
                while fetch_result_string.contains("<dc:title>") {
                    match fetch_result_string.find("</dc:title>") {
                        Some(_) => {
                            fetch_result_string =
                                fetch_result_string.replace("<dc:title>", "<dcstitle>");
                            fetch_result_string =
                                fetch_result_string.replace("</dc:title>", "</dcstitle>");
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
            if let ProviderKind::Biorxiv = provider_kind {
                while fetch_result_string.contains("<dc:title>") {
                    match fetch_result_string.find("</dc:title>") {
                        Some(_) => {
                            fetch_result_string =
                                fetch_result_string.replace("<dc:title>", "<dcstitle>");
                            fetch_result_string =
                                fetch_result_string.replace("</dc:title>", "</dcstitle>");
                        }
                        None => {
                            break;
                        }
                    }
                }
            }
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
                        let error_message = "rxiv conversion from str for ".to_string()
                            + key
                            + "error: "
                            + &e.to_string();
                        print_error_red(file!().to_string(), line!().to_string(), error_message)
                    };
                    are_there_items_handle =
                        AreThereItems::ConversionFromStrError(fetch_result_string, e.to_string());
                }
            }
        }
        _ => {
            if enable_error_prints {
                let warning_message: String = "wrong link or there is no items for key: "
                    .to_string()
                    + key
                    + " link: "
                    + value; //разделить логику при помощи нахождения паттерна архива урла
                print_warning_yellow(file!().to_string(), line!().to_string(), warning_message);
            };
            are_there_items_handle = AreThereItems::NopeNoTag(fetch_result_string);
        }
    }
    (rxiv_post_struct_handle, are_there_items_handle)
}
