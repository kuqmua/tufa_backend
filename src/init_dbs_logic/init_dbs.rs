use crate::global_variables::compile_time::git_info::GIT_INFO;
use crate::global_variables::runtime::config::CONFIG;
use crate::init_dbs_logic::init_tables_enum::InitTablesEnum;
use crate::init_dbs_logic::init_tables_enum::InitTablesWrapperError;
use futures::future::join_all;
use impl_error_with_tracing::ImplErrorWithTracingFromTufaCommon;
use impl_get_source::ImplGetSourceFromTufaCommon;
use impl_get_where_was_origin_or_wrapper::ImplGetWhereWasOriginOrWrapperFromTufaCommon;
use init_error::InitErrorFromTufaCommon;
use strum::IntoEnumIterator;
use tufa_common::common::where_was::WhereWas;
use tufa_common::traits::get_log_where_was::GetLogWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::traits::where_was_methods::WhereWasMethods;

#[derive(
    Debug,
    InitErrorFromTufaCommon,
    ImplGetWhereWasOriginOrWrapperFromTufaCommon,
    ImplGetSourceFromTufaCommon,
    ImplErrorWithTracingFromTufaCommon,
)]
pub struct InitDbsWrapperError {
    source: Vec<InitTablesWrapperError>,
    where_was: WhereWas,
}

pub async fn init_dbs(should_trace: bool) -> Result<(), Box<InitDbsWrapperError>> {
    let results =
        join_all(InitTablesEnum::iter().map(|table| async move { table.init(false).await }))
            .await
            .into_iter()
            .filter_map(|result| {
                if let Err(e) = result {
                    return Some(*e);
                }
                None
            })
            .collect::<Vec<InitTablesWrapperError>>();
    if !results.is_empty() {
        return Err(Box::new(
            InitDbsWrapperError::init_error_with_possible_trace(
                results,
                WhereWas {
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("cannot convert time to unix_epoch"),
                    file: String::from(file!()),
                    line: line!(),
                    column: column!(),
                    git_info: crate::global_variables::runtime::git_info_without_lifetimes::GIT_INFO_WITHOUT_LIFETIMES.clone(),
                },
                &CONFIG.source_place_type,
                should_trace,
            ),
        ));
    }
    Ok(())
}
