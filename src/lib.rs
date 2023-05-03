// #![deny(
//     clippy::indexing_slicing,
//     clippy::integer_arithmetic,
//     clippy::unwrap_used,
//     clippy::float_arithmetic
// )]
// #![allow(clippy::too_many_arguments)]

mod authentication;
mod check_new_providers_posts;
mod config_mods;
pub mod configuration;
pub mod email_client;
pub mod entry;
mod fetch;
pub mod global_variables;
mod init_dbs_logic;
// pub mod issue_delivery_worker;
mod mongo_integration;
mod postgres_integration;
mod preparation;
mod providers;
// mod routes;
// mod server_wrapper;
mod session_state;
// pub mod startup;
#[cfg(test)]
mod tests;
pub mod traits;

pub mod dev;
