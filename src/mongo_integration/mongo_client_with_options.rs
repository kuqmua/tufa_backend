use crate::global_variables::runtime::config::CONFIG;
use crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES;
use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use mongodb::error::Error;
use mongodb::options::ClientOptions;
use mongodb::Client;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_methods::WhereWasMethods;

pub fn mongo_client_with_options<'a>(
    client_options: ClientOptions,
    should_trace: bool,
) -> Result<Client, Box<tufa_common::repositories_types::tufa_server::mongo_integration::mongo_client_with_options::MongoClientWithOptionsOriginError<'a>>>{
    match Client::with_options(client_options) {
        Err(e) => Err(Box::new(
            tufa_common::repositories_types::tufa_server::mongo_integration::mongo_client_with_options::MongoClientWithOptionsOriginError::Mongo { 
                error: e, 
                code_occurence: tufa_common::code_occurence!() 
            } 
        )),
        Ok(client) => Ok(client),
    }
}
