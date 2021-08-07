use crate::get_project_information::get_user_credentials::structures_definitions::user_credentials_struct_def::UserCredentialsStruct;
use crate::get_project_information::project_constants::LOAD_USER_CREDENTIALS_FILE_ERROR_MESSAGE;
use crate::get_project_information::project_constants::PATH_TO_CONFIG;

lazy_static! {
    pub static ref USER_CREDENTIALS: UserCredentialsStruct =
        UserCredentialsStruct::new(PATH_TO_CONFIG).expect(LOAD_USER_CREDENTIALS_FILE_ERROR_MESSAGE);
}
