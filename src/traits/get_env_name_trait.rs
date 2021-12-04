pub trait GetEnvName {
    fn get_env_name(&self) -> &'static str;
    fn into_array() -> &'static [Self]
    where
        Self: std::marker::Sized;
}
