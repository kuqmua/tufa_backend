use crate::prints::print_type_enum::PrintType;

pub trait ProviderKindForPrintTypeTrait {
    fn is_prints_for_print_type_enabled(&self, print_type: &PrintType) -> bool;
}