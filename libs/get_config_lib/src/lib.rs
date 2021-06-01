mod get_project_information {
    pub mod get_config {
        pub mod config_structures;
        pub mod get_config_information;
    }
    pub mod get_user_credentials {
        pub mod get_user_credentials_information;
        pub mod user_credentials_structures;
    }
    pub mod project_constants;
}

#[macro_use]
extern crate lazy_static;

use crate::get_project_information::get_config::config_structures::ConfigStruct;
use crate::get_project_information::project_constants::LOAD_CONFIG_FILE_ERROR_MESSAGE;

lazy_static! {
    pub static ref CONFIG: ConfigStruct =
        ConfigStruct::new(None).expect(LOAD_CONFIG_FILE_ERROR_MESSAGE);
}

use crate::get_project_information::get_user_credentials::user_credentials_structures::UserCredentialsStruct;
use crate::get_project_information::project_constants::LOAD_USER_CREDENTIALS_FILE_ERROR_MESSAGE;

lazy_static! {
    pub static ref USER_CREDENTIALS: UserCredentialsStruct =
        UserCredentialsStruct::new().expect(LOAD_USER_CREDENTIALS_FILE_ERROR_MESSAGE);
}
