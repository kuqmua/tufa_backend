use crate::get_project_information::get_user_credentials::structures_definitions::github_authorization_def::GithubAuthorization;
use crate::get_project_information::get_user_credentials::structures_definitions::reddit_authorization_def::RedditAuthorization;
use crate::get_project_information::get_user_credentials::structures_definitions::mongo_own_authorization_def::MongoOwnAuthorization;
use crate::get_project_information::get_user_credentials::structures_definitions::mongo_cloud_authorization_def::MongoCloudAuthorization;

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)] //Default,
pub struct UserCredentialsStruct {
    pub github_authorization: GithubAuthorization,
    pub reddit_authorization: RedditAuthorization,
    pub mongo_own_authorization: MongoOwnAuthorization,
    pub mongo_cloud_authorization: MongoCloudAuthorization,
}
