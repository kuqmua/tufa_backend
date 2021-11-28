use crate::config_mods::env_var_enum::EnvVar;

use crate::config_mods::config_values_types_enums::env_var_string_enum::EnvStringVar;

use crate::traits::get_env_name_trait::GetEnvName;

impl GetEnvName for EnvStringVar {
    fn get_env_name(self: &EnvStringVar) -> &'static str {
        match self {
            EnvStringVar::GithubName => EnvVar::GithubName.get_env_name(),
            EnvStringVar::GithubToken => EnvVar::GithubToken.get_env_name(),

            EnvStringVar::RedditUserAgent => EnvVar::RedditUserAgent.get_env_name(),
            EnvStringVar::RedditClientId => EnvVar::RedditClientId.get_env_name(),
            EnvStringVar::RedditClientSecret => EnvVar::RedditClientSecret.get_env_name(),
            EnvStringVar::RedditUsername => EnvVar::RedditUsername.get_env_name(),
            EnvStringVar::RedditPassword => EnvVar::RedditPassword.get_env_name(),

            EnvStringVar::StartingCheckLink => EnvVar::StartingCheckLink.get_env_name(),
            EnvStringVar::WarningLogsDirectoryName => {
                EnvVar::WarningLogsDirectoryName.get_env_name()
            }
            EnvStringVar::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir => {
                EnvVar::UnhandledSuccessHandledSuccessAreThereItemsInitializedPostsDir
                    .get_env_name()
            }

            EnvStringVar::ProvidersDbNameHandle => EnvVar::ProvidersDbNameHandle.get_env_name(),
            EnvStringVar::ProvidersDbCollectionHandleSecondPart => {
                EnvVar::ProvidersDbCollectionHandleSecondPart.get_env_name()
            }
            EnvStringVar::ProvidersDbCollectionDocumentFieldNameHandle => {
                EnvVar::ProvidersDbCollectionDocumentFieldNameHandle.get_env_name()
            }
            EnvStringVar::PathToProviderLinkPartsFolder => {
                EnvVar::PathToProviderLinkPartsFolder.get_env_name()
            }
            EnvStringVar::LogFileExtension => EnvVar::LogFileExtension.get_env_name(),
            EnvStringVar::DbProvidersLogsNameHandle => {
                EnvVar::DbProvidersLogsNameHandle.get_env_name()
            }
            EnvStringVar::DbProvidersLogsCollectionHandleSecondPart => {
                EnvVar::DbProvidersLogsCollectionHandleSecondPart.get_env_name()
            }
            EnvStringVar::DbProvidersLogsCollectionDocumentFieldNameHandle => {
                EnvVar::DbProvidersLogsCollectionDocumentFieldNameHandle.get_env_name()
            }

            EnvStringVar::MongoFirstHandleUrlPart => EnvVar::MongoFirstHandleUrlPart.get_env_name(),
            EnvStringVar::MongoSecondHandleUrlPart => {
                EnvVar::MongoSecondHandleUrlPart.get_env_name()
            }
            EnvStringVar::MongoThirdHandleUrlPart => EnvVar::MongoThirdHandleUrlPart.get_env_name(),
            EnvStringVar::MongoFourthHandleUrlPart => {
                EnvVar::MongoFourthHandleUrlPart.get_env_name()
            }
            EnvStringVar::MongoFifthHandleUrlPart => EnvVar::MongoFifthHandleUrlPart.get_env_name(),

            EnvStringVar::MongoLogin => EnvVar::MongoLogin.get_env_name(),
            EnvStringVar::MongoPassword => EnvVar::MongoPassword.get_env_name(),
            EnvStringVar::MongoIp => EnvVar::MongoIp.get_env_name(),
            EnvStringVar::MongoPort => EnvVar::MongoPort.get_env_name(),
            EnvStringVar::MongoParams => EnvVar::MongoParams.get_env_name(),

            EnvStringVar::PostgresFirstHandleUrlPart => {
                EnvVar::PostgresFirstHandleUrlPart.get_env_name()
            }
            EnvStringVar::PostgresSecondHandleUrlPart => {
                EnvVar::PostgresSecondHandleUrlPart.get_env_name()
            }
            EnvStringVar::PostgresThirdHandleUrlPart => {
                EnvVar::PostgresThirdHandleUrlPart.get_env_name()
            }
            EnvStringVar::PostgresFourthHandleUrlPart => {
                EnvVar::PostgresFourthHandleUrlPart.get_env_name()
            }
            EnvStringVar::PostgresFifthHandleUrlPart => {
                EnvVar::PostgresFifthHandleUrlPart.get_env_name()
            }

            EnvStringVar::PostgresLogin => EnvVar::PostgresLogin.get_env_name(),
            EnvStringVar::PostgresPassword => EnvVar::PostgresPassword.get_env_name(),
            EnvStringVar::PostgresIp => EnvVar::PostgresIp.get_env_name(),
            EnvStringVar::PostgresPort => EnvVar::PostgresPort.get_env_name(),
            EnvStringVar::PostgresDb => EnvVar::PostgresDb.get_env_name(),

            EnvStringVar::ArxivLink => EnvVar::ArxivLink.get_env_name(),
            EnvStringVar::BiorxivLink => EnvVar::BiorxivLink.get_env_name(),
            EnvStringVar::GithubLink => EnvVar::GithubLink.get_env_name(),
            EnvStringVar::HabrLink => EnvVar::HabrLink.get_env_name(),
            EnvStringVar::MedrxivLink => EnvVar::MedrxivLink.get_env_name(),
            EnvStringVar::RedditLink => EnvVar::RedditLink.get_env_name(),
            EnvStringVar::TwitterLink => EnvVar::TwitterLink.get_env_name(),
        }
    }
}
