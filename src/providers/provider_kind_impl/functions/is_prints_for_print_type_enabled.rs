use crate::prints::print_type_enum::PrintType;
use crate::providers::provider_kind_enum::ProviderKind;
use crate::traits::print_type_trait::PrintTypeTrait;
use crate::traits::provider_kind_from_config_trait::ProviderKindFromConfigTrait;

impl ProviderKind {
    pub fn is_prints_for_print_type_enabled(&self, pt: &PrintType) -> bool {
        match *pt {
            PrintType::Error => pt.is_prints_enabled() && self.is_error_prints_enabled(),
            PrintType::WarningHigh => pt.is_prints_enabled() && self.is_warning_high_prints_enabled(),
            PrintType::WarningLow => pt.is_prints_enabled() && self.is_warning_low_prints_enabled(),
            PrintType::Success => pt.is_prints_enabled() && self.is_success_prints_enabled(),
            PrintType::PartialSuccess => pt.is_prints_enabled() && self.is_partial_success_prints_enabled(),
            PrintType::TimeMeasurement => pt.is_prints_enabled() && self.is_time_measurement_prints_enabled(),
            PrintType::Info => pt.is_prints_enabled() && self.is_info_prints_enabled(),
       }
    }
}