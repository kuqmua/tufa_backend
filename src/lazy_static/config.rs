use lazy_static::lazy_static;
use tufa_common::config_mods::config_struct::ConfigStruct;

lazy_static! {
    pub static ref CONFIG: ConfigStruct = ConfigStruct::new()
        .expect("cannot create config")
        .wrap_config_checks()
        .expect("wrap_config_checks error");
}
