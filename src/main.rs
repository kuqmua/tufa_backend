// #![deny(
//     clippy::indexing_slicing,
//     clippy::integer_arithmetic,
//     clippy::unwrap_used,
//     clippy::float_arithmetic
// )]
// #![allow(clippy::too_many_arguments)]

pub mod entry;
pub mod global_variables;
mod preparation;
mod server_wrapper;
#[cfg(test)]
mod tests;

//query! containing mods
pub mod issue_delivery_worker;
pub mod routes;
pub mod startup;
pub mod idempotency;

pub mod dev;

fn main() {
    crate::entry::entry(&once_cell::sync::Lazy::force(
        &crate::global_variables::runtime::config::CONFIG,
    ));
    tufa_common::dev::dev();
    crate::dev::dev();
}