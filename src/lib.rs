// #![deny(
//     clippy::indexing_slicing,
//     clippy::integer_arithmetic,
//     clippy::unwrap_used,
//     clippy::float_arithmetic
// )]
// #![allow(clippy::too_many_arguments)]

mod authentication;
// mod check_new_providers_posts;
mod config_mods;
pub mod configuration;
pub mod domain;
pub mod email_client;
pub mod entry;
mod fetch;
pub mod global_variables;
mod idempotency;
// mod init_dbs_logic;
// pub mod issue_delivery_worker;
mod logs_logic;
// mod mongo_integration;
// mod postgres_integration;
// mod preparation;
mod prints;
mod providers;
// mod routes;
// mod server_wrapper;
mod session_state;
// pub mod startup;
pub mod telemetry;
// #[cfg(test)]
// mod tests;
pub mod traits;
mod utils;
pub mod write_error_posts_wrapper;

pub mod dev;
