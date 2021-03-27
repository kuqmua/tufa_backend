use crate::fetch::provider_kind_enum::ProviderKind;
use crate::fetch::rxiv::metainfo_fetch_structures::AreThereItems;
use crate::fetch::twitter::twitter_structures::TwitterPost;
use crate::fetch::twitter::twitter_structures::TwitterPostStruct;
use crate::fetch::twitter::twitter_structures::XmlTwitterParserStruct;
use crate::overriding::prints::print_error_red;
use crate::overriding::prints::print_warning_yellow;
use serde_xml_rs::from_str;

pub fn twitter_parse_string_into_struct(
    mut fetch_result_string: String,
    key: &str,
    value: &str,
    enable_error_prints: bool,
    provider_kind: ProviderKind,
) -> (TwitterPostStruct, AreThereItems) {
    let mut rxiv_post_struct_handle: TwitterPostStruct = TwitterPostStruct::new();
    let are_there_items_handle: AreThereItems; // = AreThereItems::Initialized
                                               // println!("{:#?}", provider_kind);
                                               // if let ProviderKind::Medrxiv = provider_kind {
                                               //     fetch_result_string.remove(0);
                                               // }

    match fetch_result_string.find("<channel>") {
        Some(find_item_position_start) => match fetch_result_string.find("</channel>") {
            Some(find_item_position_end) => {
                let cutted_string: String = fetch_result_string
                    [find_item_position_start..find_item_position_end + "</channel>".len()]
                    .to_string();
                // println!(" cutted_string ***{}***", cutted_string);
                // let cutted_string_result: Result<XmlTwitterParserStruct, serde_xml_rs::Error> =
                //     from_str(&cutted_string);
                // match cutted_string_result {
                //     Ok(twitter_post) => {
                //         println!("twitter_post {:#?}", twitter_post)
                //     }
                //     Err(e) => {
                //         println!("error3, {}", e)
                //     }
                // }
            }
            _ => {
                println!("error1")
            }
        },
        _ => {
            println!("error2")
        }
    }
    match fetch_result_string.find("<channel>") {
        Some(find_item_position_start) => match fetch_result_string.find("</channel>") {
            Some(find_item_position_end) => {
                fetch_result_string = fetch_result_string
                    [find_item_position_start..find_item_position_end + "</channel>".len()]
                    .to_string();
            }
            _ => {
                println!("error1")
            }
        },
        _ => {
            println!("error2")
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
            let rxiv_struct_from_str_result: Result<XmlTwitterParserStruct, serde_xml_rs::Error> =
                from_str(&fetch_result_string);
            match rxiv_struct_from_str_result {
                Ok(rxiv_struct) => {
                    let mut count = 0;
                    let mut rxiv_page_struct: TwitterPostStruct = TwitterPostStruct::new();
                    loop {
                        if count < rxiv_struct.items.len() {
                            let mut rxiv_post: TwitterPost = TwitterPost::new();
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
            if enable_error_prints {
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
