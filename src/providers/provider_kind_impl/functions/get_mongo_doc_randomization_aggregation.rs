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
                    doc! { "$sample" : {"size": provider_kind.get_links_limit_for_provider() }},
                )
            } else {
                Some(doc! { "$limit" : provider_kind.get_links_limit_for_provider() })
            }
        } else {
            None
        }
    }
}
