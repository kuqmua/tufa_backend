extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use crate::check_net::check_link::check_link;
use crate::config::WARNING_LOGS_DIRECTORY_NAME;
use crate::fetch::provider_kind_enum::ProviderKind;

use crate::fetch::twitter_async_write_fetch_error_logs_into_files_wrapper::twitter_async_write_fetch_error_logs_into_files_wrapper;
use crate::fetch::twitter_fetch_and_parse_xml::twitter_fetch_and_parse_xml;
use crate::fetch::twitter_filter_fetched_and_parsed_posts::twitter_filter_fetched_and_parsed_posts;
use crate::overriding::prints::print_error_red;
use crate::overriding::prints::print_partial_success_cyan;
use crate::overriding::prints::print_success_green;
use crate::overriding::prints::print_warning_orange;
use futures::executor::block_on;
use std::collections::HashMap;
use std::fs;
use std::mem;
use std::path::Path;
use std::thread;

use crate::fetch::twitter::twitter_check_available_providers::twitter_check_available_providers;
use crate::get_group_names::get_arxiv_links::get_arxiv_links;
use crate::get_group_names::get_biorxiv_links::get_biorxiv_links;
use crate::get_group_names::get_medrxiv_links::get_medrxiv_links;
use crate::get_group_names::get_twitter_providers_names::get_twitter_providers_names;
use crate::get_group_names::get_twitter_subs::get_twitter_subs;
use std::sync::{Arc, Mutex};

pub fn divide_to_equal_for_each_provider<'a>(
    twitter_available_providers_links: Vec<&str>,
    links_temp_naming: HashMap<&'static str, String>,
    links_len: usize,
) -> Vec<HashMap<&'a str, String>> {
    let twitter_available_providers_links_len = twitter_available_providers_links.len();
    let links_for_each_provider: usize;
    let is_links_len_more_than_twitter_available_providers_links_len =
        links_len > twitter_available_providers_links_len;
    let vec_of_hashmap_parts_len: usize;
    if is_links_len_more_than_twitter_available_providers_links_len {
        if links_len % twitter_available_providers_links_len == 0 {
            links_for_each_provider = links_len / twitter_available_providers_links_len;
        } else {
            //little bit more memory usage than needed but no second allocation!
            links_for_each_provider = (links_len / twitter_available_providers_links_len) + 1;
        }
        vec_of_hashmap_parts_len = twitter_available_providers_links_len;
    } else {
        links_for_each_provider = links_len;
        vec_of_hashmap_parts_len = links_len;
    }
    let mut vec_of_hashmap_parts: Vec<HashMap<&str, String>> =
        vec![HashMap::with_capacity(links_for_each_provider); vec_of_hashmap_parts_len];
    let mut vec_of_hashmap_parts_element_index_counter = 0;
    let mut even_vec_of_hashmap_parts_element_index_counter = 0;
    let mut even_flag = false;
    if is_links_len_more_than_twitter_available_providers_links_len {
        for element in links_temp_naming {
            if !even_flag {
                if vec_of_hashmap_parts[vec_of_hashmap_parts_element_index_counter].len()
                    == links_for_each_provider
                {
                    if (vec_of_hashmap_parts.len() - 1)
                        != vec_of_hashmap_parts_element_index_counter
                    {
                        vec_of_hashmap_parts_element_index_counter += 1;
                        vec_of_hashmap_parts[vec_of_hashmap_parts_element_index_counter]
                            .insert(element.0, element.1);
                    } else {
                        even_flag = true;
                        vec_of_hashmap_parts[even_vec_of_hashmap_parts_element_index_counter]
                            .insert(element.0, element.1);
                        even_vec_of_hashmap_parts_element_index_counter += 1;
                    }
                } else {
                    vec_of_hashmap_parts[vec_of_hashmap_parts_element_index_counter]
                        .insert(element.0, element.1);
                }
            } else if (vec_of_hashmap_parts.len() - 1)
                != even_vec_of_hashmap_parts_element_index_counter
            {
                even_vec_of_hashmap_parts_element_index_counter += 1;
                vec_of_hashmap_parts[even_vec_of_hashmap_parts_element_index_counter]
                    .insert(element.0, element.1);
            } else {
                vec_of_hashmap_parts[even_vec_of_hashmap_parts_element_index_counter]
                    .insert(element.0, element.1);
                even_vec_of_hashmap_parts_element_index_counter = 0;
            }
        }
    } else {
        for (element_index, element) in links_temp_naming.into_iter().enumerate() {
            vec_of_hashmap_parts[element_index].insert(element.0, element.1);
        }
    }
    vec_of_hashmap_parts
}
