pub trait EnumExtenstion {
    fn into_array() -> &'static [Self]
    where
        Self: std::marker::Sized;
    fn into_vec() -> Vec<Self>
    where
        Self: std::marker::Sized;
}
