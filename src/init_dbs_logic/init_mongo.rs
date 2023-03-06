use crate::global_variables::runtime::config::CONFIG;
use crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES;
use crate::mongo_integration::mongo_check_collection_is_not_empty::mongo_check_collections_is_not_empty;
use crate::mongo_integration::mongo_client_options_parse::mongo_client_options_parse;
use crate::mongo_integration::mongo_client_with_options::mongo_client_with_options;
use crate::mongo_integration::mongo_insert_many::mongo_insert_many;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_methods::ProviderKindMethods;
use futures::future::join_all;
use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
use impl_get_git_info::ImplGetGitInfoFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::error::Error;
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::collections::HashMap;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::get_log_with_additional_where_was::GetLogWithAdditionalWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_methods::WhereWasMethods;

pub async fn init_mongo<'a>(
    providers_json_local_data_hashmap: HashMap<ProviderKind, Vec<String>>,
    should_trace: bool,
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::init_dbs_logic::init_mongo::InitMongoWrapperError<'a>>>{
    match mongo_client_options_parse().await {
        Err(e) => Err(Box::new(
            tufa_common::repositories_types::tufa_server::init_dbs_logic::init_mongo::InitMongoWrapperError::Mongo {
                inner_error: tufa_common::repositories_types::tufa_server::init_dbs_logic::init_mongo::InitMongoWrapperErrorEnum::ClientOptionsParseWrapper(
                    *e
                ),
                code_occurence: tufa_common::code_occurence!()
            }
        )),
        Ok(client_options) => match mongo_client_with_options(client_options, false) {
            Err(e) => Err(Box::new(
                tufa_common::repositories_types::tufa_server::init_dbs_logic::init_mongo::InitMongoWrapperError::Mongo {
                    inner_error: tufa_common::repositories_types::tufa_server::init_dbs_logic::init_mongo::InitMongoWrapperErrorEnum::ClientWithOptionsWrapper(*e),
                    code_occurence: tufa_common::code_occurence!()
                }
            )),
            Ok(client) => {
                let db = client.database(&CONFIG.mongo_providers_link_parts_db_name);
                if let Err(error_vec_count_documents) = mongo_check_collections_is_not_empty(
                    providers_json_local_data_hashmap.clone(),
                    &db,
                    false,
                )
                .await
                {
                    return Err(Box::new(
                        tufa_common::repositories_types::tufa_server::init_dbs_logic::init_mongo::InitMongoWrapperError::Mongo {
                            inner_error: tufa_common::repositories_types::tufa_server::init_dbs_logic::init_mongo::InitMongoWrapperErrorEnum::CollectionIsNotEmpty(*error_vec_count_documents),
                            code_occurence: tufa_common::code_occurence!()
                        }
                    ));
                }
                if let Err(error_vec_insert_many) =
                    mongo_insert_many(providers_json_local_data_hashmap, db, false).await
                {
                    return Err(Box::new(
                        tufa_common::repositories_types::tufa_server::init_dbs_logic::init_mongo::InitMongoWrapperError::Mongo {
                            inner_error: tufa_common::repositories_types::tufa_server::init_dbs_logic::init_mongo::InitMongoWrapperErrorEnum::InsertManyErrorWrapper(*error_vec_insert_many),
                            code_occurence: tufa_common::code_occurence!()
                        }
                    ));
                }
                Ok(())
            }
        },
    }
}
