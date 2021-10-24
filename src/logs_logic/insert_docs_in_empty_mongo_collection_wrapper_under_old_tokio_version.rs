use crate::providers::provider_kind_enum::ProviderKind;

use crate::mongo_integration::mongo_insert_docs_in_empty_collection::mongo_insert_docs_in_empty_collection;

use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;

//this function was created to have ability do join_all()
pub async fn insert_docs_in_empty_mongo_collection_wrapper_under_old_tokio_version(
    provider_kind: ProviderKind,
    mongo_url: &str,
    db_name_handle: &str,
    db_collection_handle: String,
    db_collection_document_field_name_handle: &str,
    vec_of_values: Vec<String>,
) -> (ProviderKind, bool) {
    let vec_of_values_len = vec_of_values.len();
    //old tokio runtime
    let result = mongo_insert_docs_in_empty_collection(
        mongo_url,
        db_name_handle,
        db_collection_handle, //fix naming later
        db_collection_document_field_name_handle,
        vec_of_values,
    )
    .await;
    match result {
        Ok(boolean_result) => {
            print_colorful_message(
                None,
                PrintType::Success,
                file!().to_string(),
                line!().to_string(),
                format!(
                    "successfull insertion {} elements into {:#?}",
                    vec_of_values_len, provider_kind
                ),
            );
            (provider_kind, boolean_result)
        }
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::WarningHigh,
                file!().to_string(),
                line!().to_string(),
                format!("mongo_insert_docs_in_empty_collection error {:#?}", e),
            );
            (provider_kind, false)
        }
    }
}
