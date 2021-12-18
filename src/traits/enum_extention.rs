pub trait EnumExtenstion {
    fn into_array() -> &'static [Self]
    where
        Self: std::marker::Sized;
}
