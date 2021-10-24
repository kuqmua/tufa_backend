use crate::logs_logic::drop_mongo_provider_logs_collection_if_need::drop_mongo_provider_logs_collection_if_need;

use crate::config_mods::config::CONFIG;
use crate::providers::provider_kind_enum::ProviderKind;

pub async fn drop_mongo_logs_collection_wrapper_for_providers(
    provider_kind_handle: ProviderKind,
    mongo_url: String,
    db_collection_handle_second_part_clone: &str,
    db_name_handle: &str,
) -> (ProviderKind, bool) {
    let result_of_dropping_collection: (ProviderKind, bool);
    match provider_kind_handle {
        ProviderKind::Arxiv => {
            result_of_dropping_collection = drop_mongo_provider_logs_collection_if_need(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_arxiv,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            )
            .await;
        }
        ProviderKind::Biorxiv => {
            result_of_dropping_collection = drop_mongo_provider_logs_collection_if_need(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_biorxiv,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            )
            .await;
        }
        ProviderKind::Github => {
            result_of_dropping_collection = drop_mongo_provider_logs_collection_if_need(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_github,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            )
            .await;
        }
        ProviderKind::Habr => {
            result_of_dropping_collection = drop_mongo_provider_logs_collection_if_need(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_habr,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            )
            .await;
        }
        ProviderKind::Medrxiv => {
            result_of_dropping_collection = drop_mongo_provider_logs_collection_if_need(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_medrxiv,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            )
            .await;
        }
        ProviderKind::Reddit => {
            result_of_dropping_collection = drop_mongo_provider_logs_collection_if_need(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_reddit,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            )
            .await;
        }
        ProviderKind::Twitter => {
            result_of_dropping_collection = drop_mongo_provider_logs_collection_if_need(
                CONFIG
                    .enable_providers_cleaning_warning_logs_db_collections_in_mongo
                    .enable_cleaning_warning_logs_db_collections_in_mongo_for_twitter,
                provider_kind_handle,
                db_collection_handle_second_part_clone.to_string(),
                mongo_url,
                db_name_handle.to_string(),
            )
            .await;
        }
    };
    result_of_dropping_collection
}
