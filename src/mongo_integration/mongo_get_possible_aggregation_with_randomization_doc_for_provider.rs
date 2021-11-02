use mongodb::bson::{doc, Document};

use crate::providers::provider_kind_enum::ProviderKind;

pub fn mongo_get_possible_aggregation_with_randomization_doc_for_provider(
    provider_kind: ProviderKind,
) -> Option<Document> {
    if ProviderKind::enable_links_limit_for(provider_kind) {
        if ProviderKind::enable_randomize_order_mongo_link_parts_for(provider_kind)
        {
            Some(doc! { "$sample" : {"size": ProviderKind::get_links_limit_for_provider(provider_kind) }})
        } else {
            Some(doc! { "$limit" : ProviderKind::get_links_limit_for_provider(provider_kind) })
        }
    } else {
        None
    }
}
