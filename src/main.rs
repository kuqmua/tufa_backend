// #![deny(
//     clippy::indexing_slicing,
//     clippy::integer_arithmetic,
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

fn main() {
    crate::entry::entry(once_cell::sync::Lazy::force(
        &crate::global_variables::runtime::config::CONFIG,
    ));
}
