use crate::fetch::info_structures::rss_structures::RssPost;
use crate::fetch::info_structures::rss_structures::RssPostStruct;
use crate::fetch::info_structures::rss_structures::XmlRssParserStruct;
use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
use crate::fetch::rss_provider_kind_enum::ProviderKind;
use crate::overriding::prints::print_error_red;
use crate::overriding::prints::print_warning_yellow;
use serde_xml_rs::from_str;

use crate::fetch::info_structures::habr_structures::HabrPost;
use crate::fetch::info_structures::habr_structures::HabrPostStruct;
use crate::fetch::info_structures::habr_structures::XmlHabrParserStruct;

use crate::fetch::info_structures::reddit_json_structs::json_reddit_parser_struct::JsonRedditParserStruct;
use crate::fetch::info_structures::reddit_json_structs::reddit_json_struct_vector::RedditJsonStructVector;
use crate::fetch::rss_reddit_parse_every_children::rss_reddit_parse_every_children;

pub fn rss_parse_string_into_struct(
    mut fetch_result_string: String,
    key: &str,
    value: &str,
    enable_error_prints: bool,
    provider_kind: ProviderKind,
) -> (RssPostStruct, AreThereItems) {
    let mut rss_post_struct_handle: RssPostStruct = RssPostStruct::new();
    let mut are_there_items_handle: AreThereItems = AreThereItems::Initialized;
    match provider_kind {
        ProviderKind::Reddit => {
            let possible_reddit_posts_structure: JsonRedditParserStruct =
                serde_json::from_str(&fetch_result_string).unwrap();
            if !possible_reddit_posts_structure.data.children.is_empty() {
                let reddit_posts_struct: RedditJsonStructVector = rss_reddit_parse_every_children(
                    &possible_reddit_posts_structure,
                    &possible_reddit_posts_structure.data.children,
                );
                // println!("{:#?}", reddit_posts_struct.posts); //[0].author//.posts.len()
                for reddit_post in reddit_posts_struct.posts {
                    let rss_post: RssPost = RssPost::initialize_new(
                        reddit_post.title,
                        reddit_post.url,
                        reddit_post.selftext,
                        reddit_post.author,
                    );
                    rss_post_struct_handle.items.push(rss_post);
                }
                if !rss_post_struct_handle.items.is_empty() {
                    are_there_items_handle = AreThereItems::Yep;
                } else {
                    are_there_items_handle = AreThereItems::NopeButThereIsTag(fetch_result_string);
                }
            } else if enable_error_prints {
                print_error_red(
                    file!().to_string(),
                    line!().to_string(),
                    "reddit_posts_structure.data.children is empty".to_string(),
                )
            }
        }
        _ => {
            if let ProviderKind::Twitter = provider_kind {
                match fetch_result_string.find("<channel>") {
                    Some(find_item_position_start) => {
                        match fetch_result_string.find("</channel>") {
                            Some(find_item_position_end) => {
                                fetch_result_string = fetch_result_string[find_item_position_start
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
                        let warning_message: String =
                            format!("no <channel> in response for key: {} link: {}", key, value);
                        print_warning_yellow(
                            file!().to_string(),
                            line!().to_string(),
                            warning_message,
                        );
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
                                        fetch_result_string.replace("<channel>", "         ");
                                    fetch_result_string =
                                        fetch_result_string.replace("</channel>", "          ");
                                }
                                None => {
                                    break;
                                }
                            }
                        }
                        //xmlns:dc="http://purl.org/dc/elements/1.1/" version="2.0"
                        // <channel>
                        // </channel>
                        //fetch_result_string = r#"<rss >

                        // <title>Все публикации подряд</title>
                        // <link>https://habr.com/ru/all/all/</link>
                        // <description>
                        // <![CDATA[ Все публикации подряд на Хабре ]]>
                        // </description>
                        // <language>ru</language>
                        // <managingEditor>editor@habr.com</managingEditor>
                        // <generator>habr.com</generator>
                        // <pubDate>Sun, 02 May 2021 14:29:51 GMT</pubDate>
                        // <image>
                        // <link>https://habr.com/ru/</link>
                        // <url>https://habr.com/images/logo.png</url>
                        // <title>Хабр</title>
                        // </image>
                        // <item>
                        // <title>
                        // <![CDATA[ Оплата в телеграм боте — Платежи 2.0 — Сбербанк + Telegraf + Node.js ]]>
                        // </title>
                        // <pubDate>Sun, 02 May 2021 06:23:25 GMT</pubDate>
                        // <guid isPermaLink="true">https://habr.com/ru/post/555460/</guid>
                        // <link>https://habr.com/ru</link>
                        // <pubDate>Sun, 02 May 2021 06:23:25 GMT</pubDate>
                        // <category>
                        // <![CDATA[ Мессенджеры ]]>
                        // </category>
                        // </item>
                        // </rss>"#
                        //                                 .to_string();
                        // /post/555460/?utm_source=habrahabr&utm_medium=rss&utm_campaign=555460
                        //<link>https://habr.com/ru/post/555460/?utm_source=habrahabr&utm_medium=rss&utm_campaign=555460</link>//problem
                        // <dc:creator>raoffonom</dc:creator>
                        let rss_struct_from_str_result: Result<
                            XmlHabrParserStruct,
                            // XmlRssParserStruct,
                            serde_xml_rs::Error,
                        > = from_str(&fetch_result_string);
                        match rss_struct_from_str_result {
                            Ok(rss_struct) => {
                                // println!("item est {}", rss_struct);.items.len()
                                // println!("item est {}", rss_struct.data.len());
                                println!("item est {:#?}", rss_struct);
                                let mut count = 0;
                                // let mut rss_page_struct: HabrPostStruct = HabrPostStruct::new();
                                let mut rss_page_struct: RssPostStruct = RssPostStruct::new();
                                // loop {
                                //     if count < rss_struct.items.len() {
                                //         // let mut rss_post: HabrPost = HabrPost::new();
                                //         let mut rss_post: RssPost = RssPost::new();
                                //         rss_post.title = rss_struct.items[count].title.clone();
                                //         // rss_post.guid = rss_struct.items[count].guid.clone();
                                //         rss_post.link = rss_struct.items[count].link.clone();
                                //         rss_post.description =
                                //             rss_struct.items[count].description.clone();
                                //         // rss_post.pubdate = rss_struct.items[count].pubdate.clone();
                                //         rss_page_struct.items.push(rss_post);
                                //         count += 1;
                                //     } else {
                                //         break;
                                //     }
                                // }
                                // if !rss_page_struct.items.is_empty() {
                                //     are_there_items_handle = AreThereItems::Yep;
                                // } else {
                                //     are_there_items_handle =
                                //         AreThereItems::NopeButThereIsTag(fetch_result_string);
                                // }
                                // rss_post_struct_handle = rss_page_struct;
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
                        let rss_struct_from_str_result: Result<
                            XmlRssParserStruct,
                            serde_xml_rs::Error,
                        > = from_str(&fetch_result_string);
                        match rss_struct_from_str_result {
                            Ok(rss_struct) => {
                                let mut count = 0;
                                let mut rss_page_struct: RssPostStruct = RssPostStruct::new();
                                loop {
                                    if count < rss_struct.items.len() {
                                        let mut rss_post: RssPost = RssPost::new();
                                        rss_post.title = rss_struct.items[count].title.clone();
                                        rss_post.link = rss_struct.items[count].link.clone();
                                        rss_post.description =
                                            rss_struct.items[count].description.clone();
                                        rss_page_struct.items.push(rss_post);
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
