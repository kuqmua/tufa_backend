use crate::get_project_information::get_user_credentials::user_credentials_structures::UserCredentialsStruct;

lazy_static! {
    pub static ref USER_CREDENTIALS: UserCredentialsStruct =
        UserCredentialsStruct::new().expect("сan not load user_credentials file");
}
