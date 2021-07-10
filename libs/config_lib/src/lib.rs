pub mod get_project_information {
    pub mod get_config {
        pub mod config_structures;
        pub mod get_config_information;
    }
    pub mod get_user_credentials {
        pub mod get_user_credentials_information;
        pub mod user_credentials_structures;
    }
    pub mod project_constants;
    pub mod provider_kind_enum;
}

#[macro_use]
extern crate lazy_static;
