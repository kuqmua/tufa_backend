use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::mongo_integration::mongo_drop_empty_collection::mongo_drop_empty_collection;
use crate::mongo_integration::mongo_drop_empty_collection::MongoDropEmptyCollectionErrorEnum;
use crate::prints::print_colorful_message::print_colorful_message;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
use tufa_common::config_mods::print_type::PrintType;
use tufa_common::traits::get_git_source_file_link::GetGitSourceFileLink;

pub async fn drop_mongo_provider_logs_collection_if_need(
    pk: &ProviderKind,
    mongo_url: String,
) -> Result<(), Box<MongoDropEmptyCollectionErrorEnum>> {
    if !pk.is_cleaning_warning_logs_directory_enabled() {
        return Ok(());
    }
    if let Err(e) = mongo_drop_empty_collection(
        &mongo_url,
        &CONFIG.mongo_providers_logs_db_name,
        &format!(
            "{pk:#?}{}",
            &CONFIG.mongo_providers_logs_db_collection_handle_second_part
        ),
    )
    .await
    {
        print_colorful_message(
            Some(pk),
            tufa_common::config_mods::print_type::PrintType::WarningHigh,
            vec![format!("{}:{}:{}", file!(), line!(), column!())],
            vec![GIT_INFO.get_git_source_file_link(file!(), line!())],
            format!("drop fail with error {e:#?}"),
        );
        return Err(e);
    }
    Ok(())
}
