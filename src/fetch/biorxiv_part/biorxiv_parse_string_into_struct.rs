use super::biorxiv_structures::BiorxivPageStruct;
use super::biorxiv_structures::BiorxivPost;
use crate::fetch::metainfo_fetch_structures::AreThereItems;
// use super::biorxiv_structures::BiorxivPageStructItem;
use super::biorxiv_structures::Creator;
use super::biorxiv_structures::XmlBiorxivParserStruct;
use crate::config::ENABLE_ERROR_PRINTS_BIORXIV;
use crate::config::ENABLE_PRINTS_BIORXIV;
use serde_xml_rs::from_str;

pub fn biorxiv_parse_string_into_struct(
    mut fetch_tuple_result: String,
    key: &str,
    value: &str,
) -> (BiorxivPageStruct, AreThereItems) {
    let mut biorxiv_post_struct_handle: BiorxivPageStruct = BiorxivPageStruct::new();
    let are_there_items_handle: AreThereItems; // = AreThereItems::Initialized
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
    } //
    match fetch_tuple_result.find("</item>") {
        Some(_) => {
            let biorxiv_struct_from_str_result: Result<
                XmlBiorxivParserStruct,
                serde_xml_rs::Error,
            > = from_str(&fetch_tuple_result);
            match biorxiv_struct_from_str_result {
                Ok(biorxiv_struct) => {
                    let mut count = 0;
                    let mut biorxiv_page_struct: BiorxivPageStruct = BiorxivPageStruct::new();
                    loop {
                        if count < biorxiv_struct.items.len() {
                            let mut biorxiv_post: BiorxivPost = BiorxivPost::new();
                            biorxiv_post.title = biorxiv_struct.items[count].title.clone();
                            biorxiv_post.link = biorxiv_struct.items[count].link.clone();
                            biorxiv_post.description =
                                biorxiv_struct.items[count].description.clone();
                            let mut string_part_for_loop =
                                biorxiv_struct.items[count].creator.clone();
                            while let Some(link_index_from_start) =
                                string_part_for_loop.find("<a href=\"")
                            {
                                if let Some(link_index_from_end) = string_part_for_loop.find("\">")
                                {
                                    if let Some(name_index_from_end) =
                                        string_part_for_loop.find("</a>")
                                    {
                                        let mut creator = Creator::new();
                                        creator.link = string_part_for_loop[link_index_from_start
                                            + "<a href=\"".len()
                                            ..link_index_from_end]
                                            .to_string();
                                        let name_index_from_start =
                                            link_index_from_end + "\">".len();
                                        creator.name = string_part_for_loop
                                            [name_index_from_start..name_index_from_end]
                                            .to_string();
                                        string_part_for_loop = string_part_for_loop
                                            [name_index_from_end + "\">".len()..]
                                            .to_string();
                                        biorxiv_post.creators.push(creator);
                                    }
                                }
                            }

                            biorxiv_page_struct.items.push(biorxiv_post);
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    if !biorxiv_page_struct.items.is_empty() {
                        are_there_items_handle = AreThereItems::Yep;
                    } else {
                        are_there_items_handle =
                            AreThereItems::NopeButThereIsTag(fetch_tuple_result);
                    }
                    biorxiv_post_struct_handle = biorxiv_page_struct;
                }
                Err(e) => {
                    if ENABLE_ERROR_PRINTS_BIORXIV {
                        println!(
                            "biorxiv conversion from str for {}, error {}",
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
            if ENABLE_PRINTS_BIORXIV {
                println!("biorxiv no items for key {} {}", key, value);
            };
            are_there_items_handle = AreThereItems::NopeNoTag(fetch_tuple_result);
        }
    }
    (biorxiv_post_struct_handle, are_there_items_handle)
}
