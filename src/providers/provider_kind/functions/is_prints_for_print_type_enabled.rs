// use tufa_common::config_mods::print_type::PrintType;
// use crate::providers::provider_kind::provider_kind_enum::ProviderKind;
// use crate::providers::provider_kind::provider_kind_enum::ProviderKindFromConfig;
// use tufa_common::traits::print_type::PrintType;

// impl ProviderKind {
//     pub fn is_prints_for_print_type_enabled(&self, pt: &PrintType) -> bool {
//         match *pt {
//             tufa_common::config_mods::print_type::PrintType::WarningHigh => pt.is_prints_enabled() && self.is_error_prints_enabled(),
//             tufa_common::config_mods::print_type::PrintType::WarningHigh => {
//                 pt.is_prints_enabled() && self.is_warning_high_prints_enabled()
//             }
//             tufa_common::config_mods::print_type::PrintType::WarningLow => pt.is_prints_enabled() && self.is_warning_low_prints_enabled(),
//             tufa_common::config_mods::print_type::PrintType::Success => pt.is_prints_enabled() && self.is_success_prints_enabled(),
//             tufa_common::config_mods::print_type::PrintType::PartialSuccess => {
//                 pt.is_prints_enabled() && self.is_partial_success_prints_enabled()
//             }
//             tufa_common::config_mods::print_type::PrintType::TimeMeasurement => {
//                 pt.is_prints_enabled() && self.is_time_measurement_prints_enabled()
//             }
//             tufa_common::config_mods::print_type::PrintType::Info => pt.is_prints_enabled() && self.is_info_prints_enabled(),
//         }
//     }
// }
