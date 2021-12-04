use crate::providers::provider_kind_enum::ProviderKind;

use crate::mongo_integration::mongo_drop_empty_collection::mongo_drop_empty_collection;
use crate::mongo_integration::mongo_drop_empty_collection::MongoDropEmptyCollectionError;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::config_mods::lazy_static_config::CONFIG;

use crate::traits::env_var_bool_trait::EnvVarBoolTrait;

pub async fn drop_mongo_provider_logs_collection_if_need(
    provider_kind: ProviderKind,
    mongo_url: String,
) -> (ProviderKind, Result<(), MongoDropEmptyCollectionError>) {
    if provider_kind.is_cleaning_warning_logs_db_collections_in_mongo_enabled() {
        let db_collection_name = &format!(
            "{:#?}{}",
            provider_kind,
            &CONFIG
                .mongo_params
                .db_providers_logs_collection_handle_second_part
        );
        //using different (old) tokio runtime 0.2.25
        let future_possible_drop_collection = mongo_drop_empty_collection(
            &mongo_url,
            &CONFIG.mongo_params.db_providers_logs_name_handle,
            db_collection_name,
        )
        .await;
        match future_possible_drop_collection {
            Ok(()) => (provider_kind, Ok(())),
            Err(e) => {
                print_colorful_message(
                    Some(&provider_kind),
                    PrintType::WarningHigh,
                    file!().to_string(),
                    line!().to_string(),
                    format!("drop fail with error {:#?}", e),
                );
                (provider_kind, Err(e))
            }
        }
    } else {
        (provider_kind, Ok(()))
    }
}
