use std::collections::HashMap;

pub trait EnumExtenstion {
    fn get_length() -> usize;
    fn into_array() -> &'static [Self]
    where
        Self: std::marker::Sized;
    fn into_vec() -> Vec<Self>
    where
        Self: std::marker::Sized;
    fn into_string_name_and_variant_hashmap() -> HashMap<String, Self>
    where
        Self: std::marker::Sized;
    fn into_string_name_and_variant_tuple_vec() -> Vec<(String, Self)>
    where
        Self: std::marker::Sized;
    fn to_upper_snake_case(&self) -> String;
}
