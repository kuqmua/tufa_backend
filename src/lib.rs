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

pub mod dev;
