pub trait EnvVarTrait {
    fn get_env_name(&self) -> &'static str;
}
