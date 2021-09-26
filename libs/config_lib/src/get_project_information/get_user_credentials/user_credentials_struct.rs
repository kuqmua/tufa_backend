use crate::get_project_information::get_user_credentials::github_authorization_struct::GithubAuthorization;
use crate::get_project_information::get_user_credentials::mongo_cloud_authorization_struct::MongoCloudAuthorization;
use crate::get_project_information::get_user_credentials::mongo_own_authorization_struct::MongoOwnAuthorization;
use crate::get_project_information::get_user_credentials::postgres_own_authorization_struct::PostgresOwnAuthorization;
use crate::get_project_information::get_user_credentials::reddit_authorization_struct::RedditAuthorization;

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)] //Default,
pub struct UserCredentialsStruct {
    pub github_authorization: GithubAuthorization,
    pub reddit_authorization: RedditAuthorization,
    pub mongo_own_authorization: MongoOwnAuthorization,
    pub postgres_own_authorization: PostgresOwnAuthorization,
    pub mongo_cloud_authorization: MongoCloudAuthorization,
}
