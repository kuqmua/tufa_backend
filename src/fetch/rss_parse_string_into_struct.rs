use crate::fetch::info_structures::common_rss_structures::CommonRssPost;
use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;

use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_provider_kind_enum::ProviderKind;
use crate::overriding::prints::print_error_red;
use crate::overriding::prints::print_warning_yellow;

use crate::fetch::info_structures::structs_for_parsing::arxiv_struct_for_parsing::ArxivStructForParsing;
use crate::fetch::info_structures::structs_for_parsing::biorxiv_struct_for_parsing::BiorxivStructForParsing;
use crate::fetch::info_structures::structs_for_parsing::habr_struct_for_parsing::HabrStructForParsing;
use crate::fetch::info_structures::structs_for_parsing::medrxiv_struct_for_parsing::MedrxivStructForParsing;
use crate::fetch::info_structures::structs_for_parsing::reddit_struct_for_parsing::RedditStructForParsing;
use crate::fetch::info_structures::structs_for_parsing::twitter_struct_for_parsing::TwitterStructForParsing;

use serde_xml_rs::from_str;
use strum::EnumMessage;

pub fn rss_parse_string_into_struct(
    mut fetch_result_string: String,
    key: &str,
    value: &str,
    enable_error_prints: bool,
    provider_kind: ProviderKind,
) -> (CommonRssPostStruct, AreThereItems) {
    let mut rss_post_struct_handle: CommonRssPostStruct = CommonRssPostStruct::new();
    let mut are_there_items_handle: AreThereItems = AreThereItems::Initialized;
    match provider_kind {
        ProviderKind::Reddit => {
            //todo option fields
            //cuz reddit in json, others on commit time in xml
            let rss_struct_from_str_result: Result<RedditStructForParsing, serde_json::Error> =
                serde_json::from_str(&fetch_result_string);
            match rss_struct_from_str_result {
                Ok(rss_struct) => {
                    let mut count = 0;
                    let mut rss_page_struct: CommonRssPostStruct = CommonRssPostStruct::new();
                    loop {
                        if count < rss_struct.data.children.len() {
                            rss_page_struct
                                .items
                                .push(CommonRssPost::initialize_with_params(
                                    //todo option fields
                                    rss_struct.data.children[count].data.title.clone(),
                                    rss_struct.data.children[count].data.url.clone(),
                                    rss_struct.data.children[count].data.selftext.clone(),
                                    rss_struct.data.children[count].data.author.clone(),
                                    provider_kind.get_message().unwrap().to_string(),
                                    //biorxiv specific
                                    None,
                                    None,
                                    None,
                                    None,
                                    //biorxiv specific

                                    //habr specific
                                    None,
                                    None,
                                    None,
                                    //habr specific

                                    //medrxiv specific
                                    None,
                                    None,
                                    None,
                                    None,
                                    //medrxiv specific

                                    //reddit specific
                                    rss_struct.data.children[count]
                                        .data
                                        .url_overridden_by_dest
                                        .clone(),
                                    rss_struct.data.children[count].data.subreddit.clone(),
                                    rss_struct.data.children[count].data.id.clone(),
                                    rss_struct.data.children[count].data.author_fullname.clone(),
                                    rss_struct.data.children[count].data.domain.clone(),
                                    rss_struct.data.children[count].data.permalink.clone(),
                                    rss_struct.data.children[count].data.thumbnail.clone(),
                                    rss_struct.data.children[count].data.url.clone(),
                                    rss_struct.data.children[count].data.name.clone(),
                                    rss_struct.data.children[count].data.subreddit_id.clone(),
                                    rss_struct.data.children[count].data.subreddit_subscribers,
                                    rss_struct.data.children[count].data.created,
                                    rss_struct.data.children[count].data.upvote_ratio,
                                    rss_struct.data.children[count].data.total_awards_received,
                                    rss_struct.data.children[count].data.downs,
                                    rss_struct.data.children[count].data.created_utc,
                                    rss_struct.data.children[count].data.ups,
                                    rss_struct.data.children[count].data.score,
                                    rss_struct.data.children[count].data.num_comments,
                                    rss_struct.data.children[count].data.is_video,
                                    rss_struct.data.children[count].data.hidden,
                                    rss_struct.data.children[count].data.send_replies,
                                    rss_struct.data.children[count].data.stickied,
                                    rss_struct.data.children[count].data.is_original_content,
                                    rss_struct.data.children[count].data.is_reddit_media_domain,
                                    rss_struct.data.children[count].data.is_meta,
                                    rss_struct.data.children[count].data.allow_live_comments,
                                    rss_struct.data.children[count].data.archived,
                                    rss_struct.data.children[count].data.over_18,
                                    rss_struct.data.children[count].data.quarantine,
                                    rss_struct.data.children[count].data.is_self,
                                    rss_struct.data.children[count].data.saved,
                                    rss_struct.data.children[count].data.is_crosspostable,
                                    rss_struct.data.children[count].data.pinned,
                                    rss_struct.data.children[count].data.media_only,
                                    rss_struct.data.children[count].data.spoiler,
                                    rss_struct.data.children[count].data.locked,
                                    rss_struct.data.children[count].data.visited,
                                    //reddit specific

                                    //twitter specific
                                    None,
                                    None,
                                    //twitter specific
                                ));
                            count += 1;
                        } else {
                            break;
                        }
                    }

                    if !rss_page_struct.items.is_empty() {
                        are_there_items_handle = AreThereItems::Yep;
                    } else {
                        are_there_items_handle =
                            AreThereItems::NopeButThereIsTag(fetch_result_string);
                    }
                    rss_post_struct_handle = rss_page_struct;
                }
                Err(e) => {
                    if enable_error_prints {
                        let error_message = "Rss conversion from str for ".to_string()
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
            match fetch_result_string.find("</item>") {
                Some(_) => {
                    //preparation
                    if let ProviderKind::Twitter = provider_kind {
                        match fetch_result_string.find("<channel>") {
                            Some(find_item_position_start) => {
                                match fetch_result_string.find("</channel>") {
                                    Some(find_item_position_end) => {
                                        fetch_result_string = fetch_result_string
                                            [find_item_position_start
                                                ..find_item_position_end + "</channel>".len()]
                                            .to_string();
                                    }
                                    _ => {
                                        let warning_message: String = format!(
                                            "no </channel> in response for key: {} link: {}",
                                            key, value
                                        );
                                        print_warning_yellow(
                                            file!().to_string(),
                                            line!().to_string(),
                                            warning_message,
                                        );
                                    }
                                }
                            }
                            _ => {
                                let warning_message: String = format!(
                                    "no <channel> in response for key: {} link: {}",
                                    key, value
                                );
                                print_warning_yellow(
                                    file!().to_string(),
                                    line!().to_string(),
                                    warning_message,
                                );
                            }
                        }
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
                            fetch_result_string =
                                fetch_result_string.replace("<atom:link", "<atom_link");
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
                    if let ProviderKind::Habr = provider_kind {
                        while fetch_result_string.contains("<channel>") {
                            match fetch_result_string.find("</channel>") {
                                Some(_) => {
                                    fetch_result_string =
                                        fetch_result_string.replace("<channel>", "         "); //replace wuth same string len -> no second allocation then?
                                    fetch_result_string =
                                        fetch_result_string.replace("</channel>", "          ");
                                    //replace wuth same string len -> no second allocation then?
                                }
                                None => {
                                    break;
                                }
                            }
                        }
                    }
                    //preparation
                    if let ProviderKind::Arxiv = provider_kind {
                        let rss_struct_from_str_result: Result<
                            ArxivStructForParsing,
                            serde_xml_rs::Error,
                        > = from_str(&fetch_result_string);
                        match rss_struct_from_str_result {
                            Ok(rss_struct) => {
                                let mut count = 0;
                                let mut rss_page_struct: CommonRssPostStruct =
                                    CommonRssPostStruct::new();
                                loop {
                                    if count < rss_struct.items.len() {
                                        rss_page_struct.items.push(
                                            CommonRssPost::initialize_with_params(
                                                rss_struct.items[count].title.clone(),
                                                rss_struct.items[count].link.clone(),
                                                rss_struct.items[count].description.clone(),
                                                rss_struct.items[count].creator.clone(),
                                                provider_kind.get_message().unwrap().to_string(),
                                                //biorxiv specific
                                                None,
                                                None,
                                                None,
                                                None,
                                                //biorxiv specific

                                                //habr specific
                                                None,
                                                None,
                                                None,
                                                //habr specific

                                                //medrxiv specific
                                                None,
                                                None,
                                                None,
                                                None,
                                                //medrxiv specific

                                                //reddit specific
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                //reddit specific

                                                //twitter specific
                                                None,
                                                None,
                                                //twitter specific
                                            ),
                                        );
                                        count += 1;
                                    } else {
                                        break;
                                    }
                                }
                                if !rss_page_struct.items.is_empty() {
                                    are_there_items_handle = AreThereItems::Yep;
                                } else {
                                    are_there_items_handle =
                                        AreThereItems::NopeButThereIsTag(fetch_result_string);
                                }
                                rss_post_struct_handle = rss_page_struct;
                            }
                            Err(e) => {
                                if enable_error_prints {
                                    let error_message = "Rss conversion from str for ".to_string()
                                        + key
                                        + "error: "
                                        + &e.to_string();
                                    print_error_red(
                                        file!().to_string(),
                                        line!().to_string(),
                                        error_message,
                                    )
                                };
                                are_there_items_handle = AreThereItems::ConversionFromStrError(
                                    fetch_result_string,
                                    e.to_string(),
                                );
                            }
                        }
                    } else if let ProviderKind::Biorxiv = provider_kind {
                        let rss_struct_from_str_result: Result<
                            BiorxivStructForParsing,
                            serde_xml_rs::Error,
                        > = from_str(&fetch_result_string);
                        match rss_struct_from_str_result {
                            Ok(rss_struct) => {
                                let mut count = 0;
                                let mut rss_page_struct: CommonRssPostStruct =
                                    CommonRssPostStruct::new();
                                loop {
                                    if count < rss_struct.items.len() {
                                        rss_page_struct.items.push(
                                            CommonRssPost::initialize_with_params(
                                                //todo option fields
                                                rss_struct.items[count].title.clone(),
                                                rss_struct.items[count].link.clone(),
                                                rss_struct.items[count].description.clone(),
                                                rss_struct.items[count].creator.clone(),
                                                provider_kind.get_message().unwrap().to_string(),
                                                //biorxiv specific
                                                rss_struct.items[count].date.clone(),
                                                rss_struct.items[count].identifier.clone(),
                                                rss_struct.items[count].publisher.clone(),
                                                rss_struct.items[count].publication_date.clone(),
                                                //biorxiv specific

                                                //habr specific
                                                None,
                                                None,
                                                None,
                                                //habr specific

                                                //medrxiv specific
                                                None,
                                                None,
                                                None,
                                                None,
                                                //medrxiv specific

                                                //reddit specific
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                //reddit specific

                                                //twitter specific
                                                None,
                                                None,
                                                //twitter specific
                                            ),
                                        );
                                        count += 1;
                                    } else {
                                        break;
                                    }
                                }
                                if !rss_page_struct.items.is_empty() {
                                    are_there_items_handle = AreThereItems::Yep;
                                } else {
                                    are_there_items_handle =
                                        AreThereItems::NopeButThereIsTag(fetch_result_string);
                                }
                                rss_post_struct_handle = rss_page_struct;
                            }
                            Err(e) => {
                                if enable_error_prints {
                                    let error_message = "Rss conversion from str for ".to_string()
                                        + key
                                        + "error: "
                                        + &e.to_string();
                                    print_error_red(
                                        file!().to_string(),
                                        line!().to_string(),
                                        error_message,
                                    )
                                };
                                are_there_items_handle = AreThereItems::ConversionFromStrError(
                                    fetch_result_string,
                                    e.to_string(),
                                );
                            }
                        }
                    } else if let ProviderKind::Medrxiv = provider_kind {
                        let rss_struct_from_str_result: Result<
                            MedrxivStructForParsing,
                            serde_xml_rs::Error,
                        > = from_str(&fetch_result_string);
                        match rss_struct_from_str_result {
                            Ok(rss_struct) => {
                                let mut count = 0;
                                let mut rss_page_struct: CommonRssPostStruct =
                                    CommonRssPostStruct::new();
                                loop {
                                    if count < rss_struct.items.len() {
                                        rss_page_struct.items.push(
                                            CommonRssPost::initialize_with_params(
                                                //todo option fields
                                                rss_struct.items[count].title.clone(),
                                                rss_struct.items[count].link.clone(),
                                                rss_struct.items[count].description.clone(),
                                                rss_struct.items[count].creator.clone(),
                                                provider_kind.get_message().unwrap().to_string(),
                                                //biorxiv specific
                                                None,
                                                None,
                                                None,
                                                None,
                                                //biorxiv specific

                                                //habr specific
                                                None,
                                                None,
                                                None,
                                                //habr specific

                                                //medrxiv specific
                                                rss_struct.items[count].date.clone(),
                                                rss_struct.items[count].identifier.clone(),
                                                rss_struct.items[count].publisher.clone(),
                                                rss_struct.items[count].publication_date.clone(),
                                                //medrxiv specific

                                                //reddit specific
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                //reddit specific

                                                //twitter specific
                                                None,
                                                None,
                                                //twitter specific
                                            ),
                                        );
                                        count += 1;
                                    } else {
                                        break;
                                    }
                                }
                                if !rss_page_struct.items.is_empty() {
                                    are_there_items_handle = AreThereItems::Yep;
                                } else {
                                    are_there_items_handle =
                                        AreThereItems::NopeButThereIsTag(fetch_result_string);
                                }
                                rss_post_struct_handle = rss_page_struct;
                            }
                            Err(e) => {
                                if enable_error_prints {
                                    let error_message = "Rss conversion from str for ".to_string()
                                        + key
                                        + "error: "
                                        + &e.to_string();
                                    print_error_red(
                                        file!().to_string(),
                                        line!().to_string(),
                                        error_message,
                                    )
                                };
                                are_there_items_handle = AreThereItems::ConversionFromStrError(
                                    fetch_result_string,
                                    e.to_string(),
                                );
                            }
                        }
                    } else if let ProviderKind::Habr = provider_kind {
                        let rss_struct_from_str_result: Result<
                            HabrStructForParsing,
                            serde_xml_rs::Error,
                        > = from_str(&fetch_result_string);
                        match rss_struct_from_str_result {
                            Ok(rss_struct) => {
                                let mut count = 0;
                                let mut rss_page_struct: CommonRssPostStruct =
                                    CommonRssPostStruct::new();
                                loop {
                                    if count < rss_struct.items.len() {
                                        rss_page_struct.items.push(
                                            CommonRssPost::initialize_with_params(
                                                //todo option fields
                                                rss_struct.items[count].title.clone(),
                                                rss_struct.items[count].link.clone(),
                                                rss_struct.items[count].description.clone(),
                                                rss_struct.items[count].creator.clone(),
                                                provider_kind.get_message().unwrap().to_string(),
                                                //biorxiv specific
                                                None,
                                                None,
                                                None,
                                                None,
                                                //biorxiv specific

                                                //habr specific
                                                rss_struct.items[count].guid.clone(),
                                                rss_struct.items[count].pub_date.clone(),
                                                rss_struct.items[count].category.clone(),
                                                //habr specific

                                                //medrxiv specific
                                                None,
                                                None,
                                                None,
                                                None,
                                                //medrxiv specific

                                                //reddit specific
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                //reddit specific

                                                //twitter specific
                                                None,
                                                None,
                                                //twitter specific
                                            ),
                                        );
                                        count += 1;
                                    } else {
                                        break;
                                    }
                                }
                                if !rss_page_struct.items.is_empty() {
                                    are_there_items_handle = AreThereItems::Yep;
                                } else {
                                    are_there_items_handle =
                                        AreThereItems::NopeButThereIsTag(fetch_result_string);
                                }
                                rss_post_struct_handle = rss_page_struct;
                            }
                            Err(e) => {
                                if enable_error_prints {
                                    let error_message = "Rss conversion from str for ".to_string()
                                        + key
                                        + "error: "
                                        + &e.to_string();
                                    print_error_red(
                                        file!().to_string(),
                                        line!().to_string(),
                                        error_message,
                                    )
                                };
                                are_there_items_handle = AreThereItems::ConversionFromStrError(
                                    fetch_result_string,
                                    e.to_string(),
                                );
                            }
                        }
                    } else if let ProviderKind::Twitter = provider_kind {
                        let rss_struct_from_str_result: Result<
                            TwitterStructForParsing,
                            serde_xml_rs::Error,
                        > = from_str(&fetch_result_string);
                        match rss_struct_from_str_result {
                            Ok(rss_struct) => {
                                let mut count = 0;
                                let mut rss_page_struct: CommonRssPostStruct =
                                    CommonRssPostStruct::new();
                                loop {
                                    if count < rss_struct.items.len() {
                                        rss_page_struct.items.push(
                                            CommonRssPost::initialize_with_params(
                                                //todo option fields
                                                rss_struct.items[count].title.clone(),
                                                rss_struct.items[count].link.clone(),
                                                rss_struct.items[count].description.clone(),
                                                rss_struct.items[count].creator.clone(),
                                                provider_kind.get_message().unwrap().to_string(),
                                                //biorxiv specific
                                                None,
                                                None,
                                                None,
                                                None,
                                                //biorxiv specific

                                                //habr specific
                                                None,
                                                None,
                                                None,
                                                //habr specific

                                                //medrxiv specific
                                                None,
                                                None,
                                                None,
                                                None,
                                                //medrxiv specific

                                                //reddit specific
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                None,
                                                //reddit specific

                                                //twitter specific
                                                rss_struct.items[count].pub_date.clone(),
                                                rss_struct.items[count].guid.clone(),
                                                //twitter specific
                                            ),
                                        );
                                        count += 1;
                                    } else {
                                        break;
                                    }
                                }
                                if !rss_page_struct.items.is_empty() {
                                    are_there_items_handle = AreThereItems::Yep;
                                } else {
                                    are_there_items_handle =
                                        AreThereItems::NopeButThereIsTag(fetch_result_string);
                                }
                                rss_post_struct_handle = rss_page_struct;
                            }
                            Err(e) => {
                                if enable_error_prints {
                                    let error_message = "Rss conversion from str for ".to_string()
                                        + key
                                        + "error: "
                                        + &e.to_string();
                                    print_error_red(
                                        file!().to_string(),
                                        line!().to_string(),
                                        error_message,
                                    )
                                };
                                are_there_items_handle = AreThereItems::ConversionFromStrError(
                                    fetch_result_string,
                                    e.to_string(),
                                );
                            }
                        }
                    } else {
                        print_warning_yellow(
                            file!().to_string(),
                            line!().to_string(),
                            "UNIMPLEMENTED YET".to_string(),
                        );
                    }
                }
                _ => {
                    if enable_error_prints {
                        let warning_message: String = "wrong link or there is no items for key: "
                            .to_string()
                            + key
                            + " link: "
                            + value; //разделить логику при помощи нахождения паттерна архива урла
                        print_warning_yellow(
                            file!().to_string(),
                            line!().to_string(),
                            warning_message,
                        );
                    };
                    are_there_items_handle = AreThereItems::NopeNoTag(fetch_result_string);
                }
            }
        }
    }

    (rss_post_struct_handle, are_there_items_handle)
}
