use crate::config_mods::config_functions::wrap_config_checks::WrapConfigChecksError;

pub trait WrapConfigChecks {
    #[deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::integer_arithmetic,
        clippy::float_arithmetic
    )]
    fn wrap_config_checks(self) -> Result<Self, WrapConfigChecksError>
    where
        Self: Sized;
}
