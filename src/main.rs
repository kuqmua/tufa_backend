// #![deny(
//     clippy::indexing_slicing,
//     clippy::arithmetic_side_effects,
//     clippy::unwrap_used,
//     clippy::float_arithmetic
// )]
// #![allow(clippy::too_many_arguments)]

pub mod entry;
pub mod global_variables;
mod server_wrapper;
#[cfg(test)]
mod tests;

//query! containing mods
pub mod authentication;
pub mod idempotency;
pub mod issue_delivery_worker;
pub mod routes;
pub mod try_build_actix_web_dev_server;

pub mod dev;

check_specific_dependency_version_usage::check_specific_dependency_version_usage!(tufa_server);

fn main() {
    crate::entry::entry(crate::global_variables::runtime::config::CONFIG.get_or_init(|| tufa_common::repositories_types::tufa_server::config::config_struct::Config::try_from_config_unchecked(
        tufa_common::repositories_types::tufa_server::config::config_struct::ConfigUnchecked::new()
        .unwrap_or_else(|e| panic!("failed to ConfigUnchecked::new(), reason: {e:#?}"))
    ).unwrap_or_else(|e| panic!("failed to Config try_from ConfigUnchecked, reason: {e}"))));
}
