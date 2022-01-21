use crate::providers::provider_kind_enum::ProviderKind;

use crate::mongo_integration::mongo_drop_empty_collection::mongo_drop_empty_collection;
use crate::mongo_integration::mongo_drop_empty_collection::MongoDropEmptyCollectionError;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

use crate::config_mods::lazy_static_config::CONFIG;
use crate::traits::provider_kind_from_config_trait::ProviderKindFromConfigTrait;

pub async fn drop_mongo_provider_logs_collection_if_need(
    pk: ProviderKind,
    mongo_url: String,
) -> (ProviderKind, Result<(), MongoDropEmptyCollectionError>) {
    if pk.is_cleaning_warning_logs_directory_enabled() {
        let db_collection_name = &format!(
            "{:#?}{}",
            pk, &CONFIG.mongo_providers_logs_db_collection_handle_second_part
        );
        //using different (old) tokio runtime 0.2.25
        let future_possible_drop_collection = mongo_drop_empty_collection(
            &mongo_url,
            &CONFIG.mongo_providers_logs_db_name,
            db_collection_name,
        )
        .await;
        match future_possible_drop_collection {
            Ok(()) => (pk, Ok(())),
            Err(e) => {
                print_colorful_message(
                    Some(&pk),
                    PrintType::WarningHigh,
                    file!().to_string(),
                    line!().to_string(),
                    format!("drop fail with error {:#?}", e),
                );
                (pk, Err(e))
            }
        }
    } else {
        (pk, Ok(()))
    }
}
