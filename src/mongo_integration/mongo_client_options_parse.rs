use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::mongo_integration::mongo_check_collection_is_not_empty::mongo_check_collections_is_not_empty;
use crate::mongo_integration::mongo_check_collection_is_not_empty::MongoCheckCollectionsIsNotEmptyError;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use futures::future::join_all;
use impl_error_with_tracing_for_struct_without_get_source::ImplErrorWithTracingForStructWithoutGetSourceFromTufaCommon;
use impl_get_source_with_method::ImplGetSourceWithMethodFromTufaCommon;
use impl_get_source_without_method::ImplGetSourceWithoutMethodFromTufaCommon;
use impl_get_where_was_one_or_many_one_for_error_struct::ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::error::Error;
use mongodb::options::ClientOptions;
use mongodb::Client;
use std::collections::HashMap;
use tufa_common::common::where_was::WhereWas;
use tufa_common::config_mods::traits::get_mongo_url_trait::GetMongoUrl;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_trait::WhereWasTrait;

#[derive(
    Debug,
    InitErrorFromTufaCommon,
    ImplGetSourceWithoutMethodFromTufaCommon,
    ImplGetWhereWasOneOrManyOneForErrorStructFromTufaCommon,
    ImplErrorWithTracingForStructWithoutGetSourceFromTufaCommon,
)]
pub struct MongoClientOptionsParseError {
    source: Error,
    where_was: WhereWas,
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn mongo_client_options_parse(
    should_trace: bool,
) -> Result<ClientOptions, Box<MongoClientOptionsParseError>> {
    match ClientOptions::parse(&CONFIG.get_mongo_url()).await {
        Err(e) => Err(Box::new(
            MongoClientOptionsParseError::init_error_with_possible_trace(
                e,
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    location: *core::panic::Location::caller(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO,
                should_trace,
            ),
        )),
        Ok(client_options) => Ok(client_options),
    }
}
