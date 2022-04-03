#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate box_err_from_err;
#[macro_use]
extern crate enum_extention;
#[macro_use]
extern crate error_display;
#[macro_use]
extern crate gen_enum;
#[macro_use]
extern crate gen_enum_without_values;
#[macro_use]
extern crate git_info;
#[macro_use]
extern crate impl_display;
#[macro_use]
extern crate impl_from_for_upper_struct;
#[macro_use]
extern crate init_from_env;
#[macro_use]
extern crate provider_kind_from_config;
extern crate dotenv;

fn main() {
    tufa_backend::entry::entry();
}
