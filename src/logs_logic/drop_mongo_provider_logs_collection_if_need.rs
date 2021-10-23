use crate::config_mods::provider_kind_enum::ProviderKind;

use crate::mongo_integration::mongo_drop_collection_wrapper::mongo_drop_collection_wrapper;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

pub async fn drop_mongo_provider_logs_collection_if_need(
    enable_cleaning_warning_logs_db_provider_collection: bool,
    provider_kind_handle: ProviderKind,
    db_collection_handle_second_part: String,
    mongo_url: String,
    db_name_handle: String,
) -> (ProviderKind, bool) {
    if enable_cleaning_warning_logs_db_provider_collection {
        let db_collection_name = &format!(
            "{:#?}{}",
            provider_kind_handle, db_collection_handle_second_part
        );
        //using different (old) tokio runtime 0.2.25
        let future_possible_drop_collection = mongo_drop_collection_wrapper(
            &mongo_url,
            &db_name_handle,
            db_collection_name,
            false, //todo
        )
        .await;
        match future_possible_drop_collection {
            Ok(result_flag) => (provider_kind_handle, result_flag),
            Err(e) => {
                print_colorful_message(
                    Some(&provider_kind_handle),
                    PrintType::WarningHigh,
                    file!().to_string(),
                    line!().to_string(),
                    format!("drop fail with error {:#?}", e),
                );
                (provider_kind_handle, false)
            }
        }
    } else {
        //its true coz if disable do nothing successully
        (provider_kind_handle, true)
    }
}
