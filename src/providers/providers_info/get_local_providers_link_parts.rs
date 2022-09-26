use crate::lazy_static::config::CONFIG;
use crate::lazy_static::git_info::GIT_INFO;
use crate::providers::provider_kind::functions::get_link_parts_from_local_json_file::GetLinkPartsFromLocalJsonFileError;
use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
use crate::traits::provider_kind_trait::ProviderKindTrait;
use chrono::DateTime;
use chrono::FixedOffset;
use chrono::Local;
use chrono::Utc;
use futures::future::join_all;
use impl_get_source_for_struct_with_method::ImplGetSourceForStructWithMethod;
use impl_get_where_was_one_or_many_for_struct_with_hasmap_or_vec_source_with_method::ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSourceWithMethod;
use init_error::InitError;
use std::collections::HashMap;
use tufa_common::traits::get_bunyan_where_was::GetBunyanWhereWas;
use tufa_common::traits::get_source::GetSource;
use tufa_common::traits::init_error_with_possible_trace::InitErrorWithPossibleTrace;
use tufa_common::where_was::WhereWas;
use valuable::Valuable;

#[derive(
    Debug,
    InitError,
    ImplGetWhereWasOneOrManyForStructWithHasmapOrVecSourceWithMethod,
    ImplGetSourceForStructWithMethod,
)]
pub struct GetLocalProvidersLinkPartsError {
    pub source: HashMap<ProviderKind, GetLinkPartsFromLocalJsonFileError>,
    pub where_was: WhereWas,
}

#[derive(Clone, Debug, Valuable)]
pub struct TracingVec {
    pub vec: Vec<String>,
}
impl
    tufa_common::traits::with_tracing::WithTracing<
        HashMap<ProviderKind, GetLinkPartsFromLocalJsonFileError>,
    > for GetLocalProvidersLinkPartsError
{
    fn with_tracing(
        source: HashMap<ProviderKind, GetLinkPartsFromLocalJsonFileError>,
        where_was: tufa_common::where_was::WhereWas,
        source_place_type: &tufa_common::config::source_place_type::SourcePlaceType,
        git_info: &tufa_common::helpers::git::git_info::GitInformation,
    ) -> Self {
        let error_vec_struct = TracingVec {
            vec: source
                .iter()
                .map(|(pk, error)| format!("{} {}", pk, error.get_source()))
                .collect::<Vec<String>>(),
        };
        //maybe add provider_kind for where_was_vec?
        let where_was_vec_struct = TracingVec {
            vec: source
                .iter()
                .map(|(_pk, error)| error.get_bunyan_where_was(source_place_type, git_info))
                .collect::<Vec<String>>(),
        };
        match source_place_type {
            tufa_common::config::source_place_type::SourcePlaceType::Source => {
                tracing::error!(
                    error = ?error_vec_struct,
                    children_where_was = ?where_was_vec_struct,
                    source_place = where_was.file_line_column(),
                );
            }
            tufa_common::config::source_place_type::SourcePlaceType::Github => {
                tracing::error!(
                    error = ?error_vec_struct,
                    children_where_was = ?where_was_vec_struct,
                    github_source_place = where_was.github_file_line_column(git_info),
                );
            }
            tufa_common::config::source_place_type::SourcePlaceType::None => {
                tracing::error!(error = ?error_vec_struct);
            }
        }
        Self { source, where_was }
    }
}

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub async fn get_local_providers_link_parts(
    should_trace: bool,
) -> Result<HashMap<ProviderKind, Vec<String>>, Box<GetLocalProvidersLinkPartsError>> {
    let result_vec = join_all(
        ProviderKind::get_enabled_providers_vec() //maybe its not exactly correct
            .into_iter()
            .map(|pk| async move {
                (
                    pk,
                    ProviderKind::get_link_parts_from_local_json_file(pk, false).await,
                )
            }),
    )
    .await;
    let mut errors_hashmap: HashMap<ProviderKind, GetLinkPartsFromLocalJsonFileError> =
        HashMap::new();
    let mut success_hashmap: HashMap<ProviderKind, Vec<String>> =
        HashMap::with_capacity(result_vec.len());
    for (pk, result) in result_vec {
        match result {
            Err(e) => {
                errors_hashmap.insert(pk, *e);
            }
            Ok(vec) => {
                success_hashmap.insert(pk, vec);
            }
        }
    }
    if !errors_hashmap.is_empty() {
        return Err(Box::new(
            GetLocalProvidersLinkPartsError::init_error_with_possible_trace(
                errors_hashmap,
                WhereWas {
                    time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc)
                        .with_timezone(&FixedOffset::east(CONFIG.timezone)),
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
                &CONFIG.source_place_type,
                &GIT_INFO.data,
                should_trace,
            ),
        ));
    }
    Ok(success_hashmap)
}
