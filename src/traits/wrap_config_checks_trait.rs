
use crate::config_mods::config_functions::wrap_config_checks::WrapConfigChecksError;

pub trait WrapConfigChecks {
    fn wrap_config_checks(self) -> Result<Self, WrapConfigChecksError> where Self: Sized;
}