use crate::config_mods::config_struct::ConfigStruct;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
#[test]
fn ci_check_compromised_env_vars() {
    let config_from_env = ConfigStruct::new().expect("cannot create ConfigStruct::new()");
    let default_config = ConfigStruct::default();
    assert_eq!(config_from_env, default_config)
}
