use std::collections::HashMap;

pub trait EnvVarTrait {
    fn get_env_name(&self) -> &'static str;
    fn into_array() -> &'static [Self]
    where
        Self: std::marker::Sized;
    fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, Self>
    where
        Self: std::marker::Sized;
    fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, Self)>
    where
        Self: std::marker::Sized;
}
