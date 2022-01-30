use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::fetch::info_structures::common_rss_structures::CommonRssPostStruct;
use crate::fetch::rss_part::{rss_part, RssPartErrorEnum};

use crate::providers::provider_kind_enum::ProviderKind;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn providers_new_posts_check(
    pk: ProviderKind,
    vec_of_provider_links: Vec<String>,
    posts_and_errors_arc_mutex: Arc<
        Mutex<HashMap<ProviderKind, Result<Vec<CommonRssPostStruct>, RssPartErrorEnum>>>,
    >,
) {
    match rss_part(pk, vec_of_provider_links).await {
        Ok(posts_vec) => {
            //maybe do it in parrallel? success and error posts
            //todo: try to lock few times
            if !posts_vec.is_empty() {
                //must check on empty coz lock it for nothing is bad
                match posts_and_errors_arc_mutex.lock() {
                    Ok(mut posts_handle_locked) => {
                        posts_handle_locked.insert(pk, Ok(posts_vec));
                    }
                    Err(e) => {
                        print_colorful_message(
                            None,
                            PrintType::Error,
                            file!().to_string(),
                            line!().to_string(),
                            format!("posts_handle.lock() (success_posts) error: {:#?}", e),
                        );
                    }
                }
            }
        }
        Err(e) => match posts_and_errors_arc_mutex.lock() {
            Ok(mut posts_handle_locked) => {
                posts_handle_locked.insert(pk, Err(*e));
            }
            Err(e) => {
                print_colorful_message(
                    None,
                    PrintType::Error,
                    file!().to_string(),
                    line!().to_string(),
                    format!("posts_handle.lock() error: {:#?}", e),
                );
            }
        },
    }
}
