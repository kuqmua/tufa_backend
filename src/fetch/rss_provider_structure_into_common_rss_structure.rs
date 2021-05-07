// use crate::fetch::info_structures::common_rss_structures::CommonRssPost;
// use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;

// use crate::fetch::rss_metainfo_fetch_structures::AreThereItems;
// use crate::fetch::rss_provider_kind_enum::ProviderKind;
// use crate::overriding::prints::print_error_red;
// use crate::overriding::prints::print_warning_yellow;

// use crate::fetch::info_structures::structs_for_parsing::arxiv_struct_for_parsing::ArxivStructForParsing;
// use crate::fetch::info_structures::structs_for_parsing::biorxiv_struct_for_parsing::BiorxivStructForParsing;
// use crate::fetch::info_structures::structs_for_parsing::habr_struct_for_parsing::HabrStructForParsing;
// use crate::fetch::info_structures::structs_for_parsing::medrxiv_struct_for_parsing::MedrxivStructForParsing;
// use crate::fetch::info_structures::structs_for_parsing::reddit_struct_for_parsing::RedditStructForParsing;
// use crate::fetch::info_structures::structs_for_parsing::twitter_struct_for_parsing::TwitterStructForParsing;

// pub fn rss_provider_structure_into_common_rss_structure(
//     provider_kind: ProviderKind,
//     fetch_result_string: String,
//     enable_error_prints: bool,
// ) -> (Option<CommonRssPostStruct>, AreThereItems) {
//     match provider_kind {
//         ProviderKind::Arxiv => {
//             let rss_struct_from_str_result: Result<ArxivStructForParsing, serde_xml_rs::Error> =
//                 from_str(&fetch_result_string);
//         }
//         ProviderKind::Biorxiv => {
//             let rss_struct_from_str_result: Result<BiorxivStructForParsing, serde_xml_rs::Error> =
//                 from_str(&fetch_result_string);
//         }
//         ProviderKind::Habr => {
//             let rss_struct_from_str_result: Result<HabrStructForParsing, serde_xml_rs::Error> =
//                 from_str(&fetch_result_string);
//         }
//         ProviderKind::Medrxiv => {
//             let rss_struct_from_str_result: Result<MedrxivStructForParsing, serde_xml_rs::Error> =
//                 from_str(&fetch_result_string);
//         }

//         ProviderKind::Reddit => {
//             let rss_struct_from_str_result: Result<RedditStructForParsing, serde_json::Error> =
//                 serde_json::from_str(&fetch_result_string);
//         }
//         ProviderKind::Twitter => {
//             let rss_struct_from_str_result: Result<TwitterStructForParsing, serde_xml_rs::Error> =
//                 from_str(&fetch_result_string);
//             match rss_struct_from_str_result {
//                 Ok(rss_struct) => {
//                     let mut count = 0;
//                     let mut rss_page_struct: CommonRssPostStruct = CommonRssPostStruct::new();
//                     loop {
//                         if count < rss_struct.items.len() {
//                             rss_page_struct
//                                 .items
//                                 .push(CommonRssPost::initialize_with_params(
//                                     //todo option fields
//                                     rss_struct.items[count].title.clone(),
//                                     rss_struct.items[count].link.clone(),
//                                     rss_struct.items[count].description.clone(),
//                                     rss_struct.items[count].creator.clone(),
//                                     provider_kind.get_message().unwrap().to_string(),
//                                     //biorxiv specific
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     //biorxiv specific

//                                     //habr specific
//                                     None,
//                                     None,
//                                     None,
//                                     //habr specific

//                                     //medrxiv specific
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     //medrxiv specific

//                                     //reddit specific
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     None,
//                                     //reddit specific

//                                     //twitter specific
//                                     rss_struct.items[count].pub_date.clone(),
//                                     rss_struct.items[count].guid.clone(),
//                                     //twitter specific
//                                 ));
//                             count += 1;
//                         } else {
//                             break;
//                         }
//                     }
//                     if !rss_page_struct.items.is_empty() {
//                         are_there_items_handle = AreThereItems::Yep;
//                     } else {
//                         are_there_items_handle =
//                             AreThereItems::NopeButThereIsTag(fetch_result_string);
//                     }
//                     rss_post_struct_handle = rss_page_struct;
//                 }
//                 Err(e) => {
//                     if enable_error_prints {
//                         let error_message = "Rss conversion from str for ".to_string()
//                             + key
//                             + "error: "
//                             + &e.to_string();
//                         print_error_red(file!().to_string(), line!().to_string(), error_message)
//                     };
//                     are_there_items_handle =
//                         AreThereItems::ConversionFromStrError(fetch_result_string, e.to_string());
//                     (None, are_there_items_handle)
//                 }
//             }
//         }
//     }
// }
