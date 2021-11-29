use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;

use mongodb::bson::{doc, Document};

impl ProviderKind {
    pub fn get_mongo_doc_randomization_aggregation(
        provider_kind: ProviderKind,
    ) -> Option<Document> {
        if provider_kind.is_link_limits_enabled() {
            if provider_kind.is_randomize_order_mongo_link_parts_enabled() {
                Some(
                    doc! { "$sample" : {"size": ProviderKind::get_links_limit_for_provider(provider_kind) }},
                )
            } else {
                Some(doc! { "$limit" : ProviderKind::get_links_limit_for_provider(provider_kind) })
            }
        } else {
            None
        }
    }
}
