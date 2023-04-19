use crate::global_variables::runtime::config::CONFIG;
use crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_methods::ProviderKindMethods;
use futures::future::join_all;
use impl_display_for_error::ImplDisplayForError;
use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use mongodb::bson::Document;
use mongodb::error::Error;
use mongodb::Database;
use std::collections::HashMap;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_methods::WhereWasMethods;

pub async fn mongo_check_collections_is_not_empty<'a>(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
    db: &Database,
    should_trace: bool,
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyError<'a>>>{
    let error_vec_count_documents =
        join_all(providers_json_local_data_hashmap.keys().map(|pk| async move {
            (
                *pk,
                db.collection::<Document>(&format!("{pk}"))
                    .count_documents(None, None)
                    .await,
            )
        }))
        .await
        .into_iter()
        .filter_map(|(pk, result)| match result {
            Err(e) => Some((
                pk.to_string(),
                tufa_common::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorEnum::CountDocumentsOrigin(
                    tufa_common::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorEnumCountDocuments::CountDocuments { 
                        error: e, 
                        code_occurence: tufa_common::code_occurence!(), 
                    }
                ),
            )),
            Ok(documents_number) => {
                if documents_number > 0 {
                    return Some((
                        pk.to_string(),
                        tufa_common::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorEnum::IsNotEmptyOrigin(
                            tufa_common::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorEnumIsNotEmptyOrigin::IsNotEmptyOrigin { 
                                error: documents_number, 
                                code_occurence: tufa_common::code_occurence!()
                            }
                        )
                    ));
                }
                None
            }
        })
        .collect::<HashMap<String, tufa_common::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyErrorEnum>>();
    if !error_vec_count_documents.is_empty() {
        return Err(Box::new(
            tufa_common::repositories_types::tufa_server::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionIsNotEmptyError::Mongo {        
                inner_errors: error_vec_count_documents, 
                code_occurence: tufa_common::code_occurence!()
            }
        ));
    }
    Ok(())
}
