use mongodb::bson::{doc, Document};

pub fn mongo_get_possible_aggregation_with_randomization_doc_for_provider(
    enable_links_limit_for_provider: bool,
    enable_randomize_order_for_providers_link_parts_for_mongo: bool,
    enable_randomize_order_for_provider_link_parts_for_mongo: bool,
    link_limits_for_provider: u64,
) -> Option<Document> {
    if enable_links_limit_for_provider {
        if enable_randomize_order_for_providers_link_parts_for_mongo
            && enable_randomize_order_for_provider_link_parts_for_mongo
        {
            Some(doc! { "$sample" : {"size": link_limits_for_provider }})
        } else {
            Some(doc! { "$limit" : link_limits_for_provider })
        }
    } else {
        None
    }
}
