use mongodb::bson::Document;

use crate::get_project_information::get_config::get_lazy_config_information::CONFIG;
use crate::get_project_information::provider_kind_enum::ProviderKind;

use crate::mongo_integration::mongo_get_possible_aggregation_with_randomization_doc_for_provider::mongo_get_possible_aggregation_with_randomization_doc_for_provider;

pub fn mongo_get_possible_aggregation_with_randomization_doc_for_provider_wrapper(
    provider_kind: ProviderKind,
) -> Option<Document> {
    let limit = ProviderKind::get_links_limit_for_provider(provider_kind);
    mongo_get_possible_aggregation_with_randomization_doc_for_provider(
        CONFIG
            .enable_providers_links_limits
            .enable_links_limit_for_arxiv,
        CONFIG
            .params
            .enable_randomize_order_for_providers_link_parts_for_mongo,
        CONFIG
            .enable_randomize_order_for_providers_link_parts_for_mongo
            .enable_randomize_order_for_arxiv_link_parts_for_mongo,
        limit,
    )
}
