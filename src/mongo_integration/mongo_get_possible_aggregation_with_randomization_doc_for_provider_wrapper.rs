use mongodb::bson::Document;

use crate::providers::provider_kind_enum::ProviderKind;

use crate::mongo_integration::mongo_get_possible_aggregation_with_randomization_doc_for_provider::mongo_get_possible_aggregation_with_randomization_doc_for_provider;

pub fn mongo_get_possible_aggregation_with_randomization_doc_for_provider_wrapper(
    provider_kind: ProviderKind,
) -> Option<Document> {
    mongo_get_possible_aggregation_with_randomization_doc_for_provider(
        provider_kind
    )
}
