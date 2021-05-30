use crate::get_project_information::get_user_credentials::user_credentials_structures::UserCredentialsStruct;
use crate::get_project_information::project_constants::LOAD_USER_CREDENTIALS_FILE_ERROR_MESSAGE;

lazy_static! {
    pub static ref USER_CREDENTIALS: UserCredentialsStruct =
        UserCredentialsStruct::new().expect(LOAD_USER_CREDENTIALS_FILE_ERROR_MESSAGE);
}
