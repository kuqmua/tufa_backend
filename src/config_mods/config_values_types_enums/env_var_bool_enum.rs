use std::collections::HashMap;

use procedural_macros_lib::EnumVariantCount;

use strum::IntoEnumIterator;

use strum_macros::EnumIter;

use dotenv::dotenv;

use crate::config_mods::config_error::ConfigError;
use crate::config_mods::config_error_inner_type_enum::ConfigErrorInnerType;
use crate::config_mods::var_or_bool_parse_error_enum::VarOrBoolParseError;

use crate::config_mods::config_env_var_error_type_enum::ConfigEnvVarErrorType;

use crate::constants::project_constants::ENV_FILE_NAME;

use crate::config_mods::env_var_enum::EnvVar;

#[derive(
    EnumVariantCount,
    EnumIter,
    Clone,
    Debug,
    serde_derive::Serialize,
    serde_derive::Deserialize,
    PartialEq,
    Eq,
    Hash,
    Copy,
)]
pub enum EnvBoolVar {
    EnableProviders,
    EnableCleaningWarningLogsDirectory,
    EnableCleaningWarningLogsDbInMongo,
    EnableCleaningWarningLogsDbCollectionsInMongo,
    EnableTimeMeasurement,
    EnableProviderLinksLimit,
    EnableCommonProvidersLinksLimit,
    EnableRandomizeOrderForProvidersLinkPartsForMongo,
    EnablePrints,
    EnableErrorPrints,
    EnableWarningHighPrints,
    EnableWarningLowPrints,
    EnableSuccessPrints,
    EnablePartialSuccessPrints,
    EnableTimeMeasurementPrints,
    EnableCleaningWarningLogsDirectoryPrints,
    EnableInfoPrints,
    EnableAllProvidersPrints,
    EnableErrorPrintsForAllProviders,
    EnableWarningHighPrintsForAllProviders,
    EnableWarningLowPrintsForAllProviders,
    EnableSuccessPrintsForAllProviders,
    EnablePartialSuccessPrintsForAllProviders,
    EnableTimeMeasurementPrintsForAllProviders,
    EnableCleaningWarningLogsDirectoryPrintsForAllProviders,
    EnableInfoPrintsForAllProviders,
    EnableWriteErrorLogsInLocalFolder,
    EnableWriteErrorLogsInMongo,
    EnableInitializeMongoWithProvidersLinkParts,
    EnableInitializeMongoWithArxivLinkParts,
    EnableInitializeMongoWithBiorxivLinkParts,
    EnableInitializeMongoWithGithubLinkParts,
    EnableInitializeMongoWithHabrLinkParts,
    EnableInitializeMongoWithMedrxivLinkParts,
    EnableInitializeMongoWithRedditLinkParts,
    EnableInitializeMongoWithTwitterLinkParts,
    EnableArxiv,
    EnableBiorxiv,
    EnableGithub,
    EnableHabr,
    EnableMedrxiv,
    EnableReddit,
    EnableTwitter,
    EnablePrintsArxiv,
    EnablePrintsBiorxiv,
    EnablePrintsGithub,
    EnablePrintsHabr,
    EnablePrintsMedrxiv,
    EnablePrintsReddit,
    EnablePrintsTwitter,
    EnableWarningHighPrintsForArxiv,
    EnableWarningHighPrintsForBiorxiv,
    EnableWarningHighPrintsForGithub,
    EnableWarningHighPrintsForHabr,
    EnableWarningHighPrintsForMedrxiv,
    EnableWarningHighPrintsForReddit,
    EnableWarningHighPrintsForTwitter,
    EnableWarningLowPrintsForArxiv,
    EnableWarningLowPrintsForBiorxiv,
    EnableWarningLowPrintsForGithub,
    EnableWarningLowPrintsForHabr,
    EnableWarningLowPrintsForMedrxiv,
    EnableWarningLowPrintsForReddit,
    EnableWarningLowPrintsForTwitter,
    EnableErrorPrintsForArxiv,
    EnableErrorPrintsForBiorxiv,
    EnableErrorPrintsForGithub,
    EnableErrorPrintsForHabr,
    EnableErrorPrintsForMedrxiv,
    EnableErrorPrintsForReddit,
    EnableErrorPrintsForTwitter,
    EnableSuccessPrintsForArxiv,
    EnableSuccessPrintsForBiorxiv,
    EnableSuccessPrintsForGithub,
    EnableSuccessPrintsForHabr,
    EnableSuccessPrintsForMedrxiv,
    EnableSuccessPrintsForReddit,
    EnableSuccessPrintsForTwitter,
    EnablePartialSuccessPrintsForArxiv,
    EnablePartialSuccessPrintsForBiorxiv,
    EnablePartialSuccessPrintsForGithub,
    EnablePartialSuccessPrintsForHabr,
    EnablePartialSuccessPrintsForMedrxiv,
    EnablePartialSuccessPrintsForReddit,
    EnablePartialSuccessPrintsForTwitter,
    EnableCleaningWarningLogsDirectoryForArxiv,
    EnableCleaningWarningLogsDirectoryForBiorxiv,
    EnableCleaningWarningLogsDirectoryForGithub,
    EnableCleaningWarningLogsDirectoryForHabr,
    EnableCleaningWarningLogsDirectoryForMedrxiv,
    EnableCleaningWarningLogsDirectoryForReddit,
    EnableCleaningWarningLogsDirectoryForTwitter,
    EnableCleaningWarningLogsDbInMongoForArxiv,
    EnableCleaningWarningLogsDbInMongoForBiorxiv,
    EnableCleaningWarningLogsDbInMongoForGithub,
    EnableCleaningWarningLogsDbInMongoForHabr,
    EnableCleaningWarningLogsDbInMongoForMedrxiv,
    EnableCleaningWarningLogsDbInMongoForReddit,
    EnableCleaningWarningLogsDbInMongoForTwitter,
    EnableCleaningWarningLogsDbCollectionsInMongoForArxiv,
    EnableCleaningWarningLogsDbCollectionsInMongoForBiorxiv,
    EnableCleaningWarningLogsDbCollectionsInMongoForGithub,
    EnableCleaningWarningLogsDbCollectionsInMongoForHabr,
    EnableCleaningWarningLogsDbCollectionsInMongoForMedrxiv,
    EnableCleaningWarningLogsDbCollectionsInMongoForReddit,
    EnableCleaningWarningLogsDbCollectionsInMongoForTwitter,
    EnableTimeMeasurementForArxiv,
    EnableTimeMeasurementForBiorxiv,
    EnableTimeMeasurementForGithub,
    EnableTimeMeasurementForHabr,
    EnableTimeMeasurementForMedrxiv,
    EnableTimeMeasurementForReddit,
    EnableTimeMeasurementForTwitter,
    EnableInfoForArxiv,
    EnableInfoForBiorxiv,
    EnableInfoForGithub,
    EnableInfoForHabr,
    EnableInfoForMedrxiv,
    EnableInfoForReddit,
    EnableInfoForTwitter,
    EnableLinksLimitForArxiv,
    EnableLinksLimitForBiorxiv,
    EnableLinksLimitForGithub,
    EnableLinksLimitForHabr,
    EnableLinksLimitForMedrxiv,
    EnableLinksLimitForReddit,
    EnableLinksLimitForTwitter,
    EnableRandomizeOrderForArxivLinkPartsForMongo,
    EnableRandomizeOrderForBiorxivLinkPartsForMongo,
    EnableRandomizeOrderForGithubLinkPartsForMongo,
    EnableRandomizeOrderForHabrLinkPartsForMongo,
    EnableRandomizeOrderForMedrxivLinkPartsForMongo,
    EnableRandomizeOrderForRedditLinkPartsForMongo,
    EnableRandomizeOrderForTwitterLinkPartsForMongo,
}

impl EnvBoolVar {
    pub fn get_env_name(env_var_name_kind: EnvBoolVar) -> &'static str {
        match env_var_name_kind {
            EnvBoolVar::EnableProviders => EnvVar::get_env_name(EnvVar::EnableProviders),
            EnvBoolVar::EnableCleaningWarningLogsDirectory => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDirectory)
            }
            EnvBoolVar::EnableCleaningWarningLogsDbInMongo => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDbInMongo)
            }
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongo => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDbCollectionsInMongo)
            }
            EnvBoolVar::EnableTimeMeasurement => {
                EnvVar::get_env_name(EnvVar::EnableTimeMeasurement)
            }
            EnvBoolVar::EnableProviderLinksLimit => {
                EnvVar::get_env_name(EnvVar::EnableProviderLinksLimit)
            }
            EnvBoolVar::EnableCommonProvidersLinksLimit => {
                EnvVar::get_env_name(EnvVar::EnableCommonProvidersLinksLimit)
            }
            EnvBoolVar::EnableRandomizeOrderForProvidersLinkPartsForMongo => {
                EnvVar::get_env_name(EnvVar::EnableRandomizeOrderForProvidersLinkPartsForMongo)
            }
            EnvBoolVar::EnablePrints => EnvVar::get_env_name(EnvVar::EnablePrints),
            EnvBoolVar::EnableErrorPrints => EnvVar::get_env_name(EnvVar::EnableErrorPrints),
            EnvBoolVar::EnableWarningHighPrints => {
                EnvVar::get_env_name(EnvVar::EnableWarningHighPrints)
            }
            EnvBoolVar::EnableWarningLowPrints => {
                EnvVar::get_env_name(EnvVar::EnableWarningLowPrints)
            }
            EnvBoolVar::EnableSuccessPrints => EnvVar::get_env_name(EnvVar::EnableSuccessPrints),
            EnvBoolVar::EnablePartialSuccessPrints => {
                EnvVar::get_env_name(EnvVar::EnablePartialSuccessPrints)
            }
            EnvBoolVar::EnableTimeMeasurementPrints => {
                EnvVar::get_env_name(EnvVar::EnableTimeMeasurementPrints)
            }
            EnvBoolVar::EnableCleaningWarningLogsDirectoryPrints => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDirectoryPrints)
            }
            EnvBoolVar::EnableInfoPrints => EnvVar::get_env_name(EnvVar::EnableInfoPrints),
            EnvBoolVar::EnableAllProvidersPrints => {
                EnvVar::get_env_name(EnvVar::EnableAllProvidersPrints)
            }
            EnvBoolVar::EnableErrorPrintsForAllProviders => {
                EnvVar::get_env_name(EnvVar::EnableErrorPrintsForAllProviders)
            }
            EnvBoolVar::EnableWarningHighPrintsForAllProviders => {
                EnvVar::get_env_name(EnvVar::EnableWarningHighPrintsForAllProviders)
            }
            EnvBoolVar::EnableWarningLowPrintsForAllProviders => {
                EnvVar::get_env_name(EnvVar::EnableWarningLowPrintsForAllProviders)
            }
            EnvBoolVar::EnableSuccessPrintsForAllProviders => {
                EnvVar::get_env_name(EnvVar::EnableSuccessPrintsForAllProviders)
            }
            EnvBoolVar::EnablePartialSuccessPrintsForAllProviders => {
                EnvVar::get_env_name(EnvVar::EnablePartialSuccessPrintsForAllProviders)
            }
            EnvBoolVar::EnableTimeMeasurementPrintsForAllProviders => {
                EnvVar::get_env_name(EnvVar::EnableTimeMeasurementPrintsForAllProviders)
            }
            EnvBoolVar::EnableCleaningWarningLogsDirectoryPrintsForAllProviders => {
                EnvVar::get_env_name(
                    EnvVar::EnableCleaningWarningLogsDirectoryPrintsForAllProviders,
                )
            }
            EnvBoolVar::EnableInfoPrintsForAllProviders => {
                EnvVar::get_env_name(EnvVar::EnableInfoPrintsForAllProviders)
            }
            EnvBoolVar::EnableWriteErrorLogsInLocalFolder => {
                EnvVar::get_env_name(EnvVar::EnableWriteErrorLogsInLocalFolder)
            }
            EnvBoolVar::EnableWriteErrorLogsInMongo => {
                EnvVar::get_env_name(EnvVar::EnableWriteErrorLogsInMongo)
            }
            EnvBoolVar::EnableInitializeMongoWithProvidersLinkParts => {
                EnvVar::get_env_name(EnvVar::EnableInitializeMongoWithProvidersLinkParts)
            }

            EnvBoolVar::EnableInitializeMongoWithArxivLinkParts => {
                EnvVar::get_env_name(EnvVar::EnableInitializeMongoWithArxivLinkParts)
            }
            EnvBoolVar::EnableInitializeMongoWithBiorxivLinkParts => {
                EnvVar::get_env_name(EnvVar::EnableInitializeMongoWithBiorxivLinkParts)
            }
            EnvBoolVar::EnableInitializeMongoWithGithubLinkParts => {
                EnvVar::get_env_name(EnvVar::EnableInitializeMongoWithGithubLinkParts)
            }
            EnvBoolVar::EnableInitializeMongoWithHabrLinkParts => {
                EnvVar::get_env_name(EnvVar::EnableInitializeMongoWithHabrLinkParts)
            }
            EnvBoolVar::EnableInitializeMongoWithMedrxivLinkParts => {
                EnvVar::get_env_name(EnvVar::EnableInitializeMongoWithMedrxivLinkParts)
            }
            EnvBoolVar::EnableInitializeMongoWithRedditLinkParts => {
                EnvVar::get_env_name(EnvVar::EnableInitializeMongoWithRedditLinkParts)
            }
            EnvBoolVar::EnableInitializeMongoWithTwitterLinkParts => {
                EnvVar::get_env_name(EnvVar::EnableInitializeMongoWithTwitterLinkParts)
            }

            EnvBoolVar::EnableArxiv => EnvVar::get_env_name(EnvVar::EnableArxiv),
            EnvBoolVar::EnableBiorxiv => EnvVar::get_env_name(EnvVar::EnableBiorxiv),
            EnvBoolVar::EnableGithub => EnvVar::get_env_name(EnvVar::EnableGithub),
            EnvBoolVar::EnableHabr => EnvVar::get_env_name(EnvVar::EnableHabr),
            EnvBoolVar::EnableMedrxiv => EnvVar::get_env_name(EnvVar::EnableMedrxiv),
            EnvBoolVar::EnableReddit => EnvVar::get_env_name(EnvVar::EnableReddit),
            EnvBoolVar::EnableTwitter => EnvVar::get_env_name(EnvVar::EnableTwitter),

            EnvBoolVar::EnablePrintsArxiv => EnvVar::get_env_name(EnvVar::EnablePrintsArxiv),
            EnvBoolVar::EnablePrintsBiorxiv => EnvVar::get_env_name(EnvVar::EnablePrintsBiorxiv),
            EnvBoolVar::EnablePrintsGithub => EnvVar::get_env_name(EnvVar::EnablePrintsGithub),
            EnvBoolVar::EnablePrintsHabr => EnvVar::get_env_name(EnvVar::EnablePrintsHabr),
            EnvBoolVar::EnablePrintsMedrxiv => EnvVar::get_env_name(EnvVar::EnablePrintsMedrxiv),
            EnvBoolVar::EnablePrintsReddit => EnvVar::get_env_name(EnvVar::EnablePrintsReddit),
            EnvBoolVar::EnablePrintsTwitter => EnvVar::get_env_name(EnvVar::EnablePrintsTwitter),

            EnvBoolVar::EnableWarningHighPrintsForArxiv => {
                EnvVar::get_env_name(EnvVar::EnableWarningHighPrintsForArxiv)
            }
            EnvBoolVar::EnableWarningHighPrintsForBiorxiv => {
                EnvVar::get_env_name(EnvVar::EnableWarningHighPrintsForBiorxiv)
            }
            EnvBoolVar::EnableWarningHighPrintsForGithub => {
                EnvVar::get_env_name(EnvVar::EnableWarningHighPrintsForGithub)
            }
            EnvBoolVar::EnableWarningHighPrintsForHabr => {
                EnvVar::get_env_name(EnvVar::EnableWarningHighPrintsForHabr)
            }
            EnvBoolVar::EnableWarningHighPrintsForMedrxiv => {
                EnvVar::get_env_name(EnvVar::EnableWarningHighPrintsForMedrxiv)
            }
            EnvBoolVar::EnableWarningHighPrintsForReddit => {
                EnvVar::get_env_name(EnvVar::EnableWarningHighPrintsForReddit)
            }
            EnvBoolVar::EnableWarningHighPrintsForTwitter => {
                EnvVar::get_env_name(EnvVar::EnableWarningHighPrintsForTwitter)
            }

            EnvBoolVar::EnableWarningLowPrintsForArxiv => {
                EnvVar::get_env_name(EnvVar::EnableWarningLowPrintsForArxiv)
            }
            EnvBoolVar::EnableWarningLowPrintsForBiorxiv => {
                EnvVar::get_env_name(EnvVar::EnableWarningLowPrintsForBiorxiv)
            }
            EnvBoolVar::EnableWarningLowPrintsForGithub => {
                EnvVar::get_env_name(EnvVar::EnableWarningLowPrintsForGithub)
            }
            EnvBoolVar::EnableWarningLowPrintsForHabr => {
                EnvVar::get_env_name(EnvVar::EnableWarningLowPrintsForHabr)
            }
            EnvBoolVar::EnableWarningLowPrintsForMedrxiv => {
                EnvVar::get_env_name(EnvVar::EnableWarningLowPrintsForMedrxiv)
            }
            EnvBoolVar::EnableWarningLowPrintsForReddit => {
                EnvVar::get_env_name(EnvVar::EnableWarningLowPrintsForReddit)
            }
            EnvBoolVar::EnableWarningLowPrintsForTwitter => {
                EnvVar::get_env_name(EnvVar::EnableWarningLowPrintsForTwitter)
            }

            EnvBoolVar::EnableErrorPrintsForArxiv => {
                EnvVar::get_env_name(EnvVar::EnableErrorPrintsForArxiv)
            }
            EnvBoolVar::EnableErrorPrintsForBiorxiv => {
                EnvVar::get_env_name(EnvVar::EnableErrorPrintsForBiorxiv)
            }
            EnvBoolVar::EnableErrorPrintsForGithub => {
                EnvVar::get_env_name(EnvVar::EnableErrorPrintsForGithub)
            }
            EnvBoolVar::EnableErrorPrintsForHabr => {
                EnvVar::get_env_name(EnvVar::EnableErrorPrintsForHabr)
            }
            EnvBoolVar::EnableErrorPrintsForMedrxiv => {
                EnvVar::get_env_name(EnvVar::EnableErrorPrintsForMedrxiv)
            }
            EnvBoolVar::EnableErrorPrintsForReddit => {
                EnvVar::get_env_name(EnvVar::EnableErrorPrintsForReddit)
            }
            EnvBoolVar::EnableErrorPrintsForTwitter => {
                EnvVar::get_env_name(EnvVar::EnableErrorPrintsForTwitter)
            }

            EnvBoolVar::EnableSuccessPrintsForArxiv => {
                EnvVar::get_env_name(EnvVar::EnableSuccessPrintsForArxiv)
            }
            EnvBoolVar::EnableSuccessPrintsForBiorxiv => {
                EnvVar::get_env_name(EnvVar::EnableSuccessPrintsForBiorxiv)
            }
            EnvBoolVar::EnableSuccessPrintsForGithub => {
                EnvVar::get_env_name(EnvVar::EnableSuccessPrintsForGithub)
            }
            EnvBoolVar::EnableSuccessPrintsForHabr => {
                EnvVar::get_env_name(EnvVar::EnableSuccessPrintsForHabr)
            }
            EnvBoolVar::EnableSuccessPrintsForMedrxiv => {
                EnvVar::get_env_name(EnvVar::EnableSuccessPrintsForMedrxiv)
            }
            EnvBoolVar::EnableSuccessPrintsForReddit => {
                EnvVar::get_env_name(EnvVar::EnableSuccessPrintsForReddit)
            }
            EnvBoolVar::EnableSuccessPrintsForTwitter => {
                EnvVar::get_env_name(EnvVar::EnableSuccessPrintsForTwitter)
            }

            EnvBoolVar::EnablePartialSuccessPrintsForArxiv => {
                EnvVar::get_env_name(EnvVar::EnablePartialSuccessPrintsForArxiv)
            }
            EnvBoolVar::EnablePartialSuccessPrintsForBiorxiv => {
                EnvVar::get_env_name(EnvVar::EnablePartialSuccessPrintsForBiorxiv)
            }
            EnvBoolVar::EnablePartialSuccessPrintsForGithub => {
                EnvVar::get_env_name(EnvVar::EnablePartialSuccessPrintsForGithub)
            }
            EnvBoolVar::EnablePartialSuccessPrintsForHabr => {
                EnvVar::get_env_name(EnvVar::EnablePartialSuccessPrintsForHabr)
            }
            EnvBoolVar::EnablePartialSuccessPrintsForMedrxiv => {
                EnvVar::get_env_name(EnvVar::EnablePartialSuccessPrintsForMedrxiv)
            }
            EnvBoolVar::EnablePartialSuccessPrintsForReddit => {
                EnvVar::get_env_name(EnvVar::EnablePartialSuccessPrintsForReddit)
            }
            EnvBoolVar::EnablePartialSuccessPrintsForTwitter => {
                EnvVar::get_env_name(EnvVar::EnablePartialSuccessPrintsForTwitter)
            }

            EnvBoolVar::EnableCleaningWarningLogsDirectoryForArxiv => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDirectoryForArxiv)
            }
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForBiorxiv => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDirectoryForBiorxiv)
            }
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForGithub => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDirectoryForGithub)
            }
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForHabr => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDirectoryForHabr)
            }
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForMedrxiv => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDirectoryForMedrxiv)
            }
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForReddit => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDirectoryForReddit)
            }
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForTwitter => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDirectoryForTwitter)
            }

            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForArxiv => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDbInMongoForArxiv)
            }
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForBiorxiv => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDbInMongoForBiorxiv)
            }
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForGithub => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDbInMongoForGithub)
            }
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForHabr => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDbInMongoForHabr)
            }
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForMedrxiv => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDbInMongoForMedrxiv)
            }
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForReddit => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDbInMongoForReddit)
            }
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForTwitter => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDbInMongoForTwitter)
            }

            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForArxiv => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForArxiv)
            }
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxiv => {
                EnvVar::get_env_name(
                    EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxiv,
                )
            }
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForGithub => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForGithub)
            }
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForHabr => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForHabr)
            }
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxiv => {
                EnvVar::get_env_name(
                    EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxiv,
                )
            }
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForReddit => {
                EnvVar::get_env_name(EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForReddit)
            }
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForTwitter => {
                EnvVar::get_env_name(
                    EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForTwitter,
                )
            }

            EnvBoolVar::EnableTimeMeasurementForArxiv => {
                EnvVar::get_env_name(EnvVar::EnableTimeMeasurementForArxiv)
            }
            EnvBoolVar::EnableTimeMeasurementForBiorxiv => {
                EnvVar::get_env_name(EnvVar::EnableTimeMeasurementForBiorxiv)
            }
            EnvBoolVar::EnableTimeMeasurementForGithub => {
                EnvVar::get_env_name(EnvVar::EnableTimeMeasurementForGithub)
            }
            EnvBoolVar::EnableTimeMeasurementForHabr => {
                EnvVar::get_env_name(EnvVar::EnableTimeMeasurementForHabr)
            }
            EnvBoolVar::EnableTimeMeasurementForMedrxiv => {
                EnvVar::get_env_name(EnvVar::EnableTimeMeasurementForMedrxiv)
            }
            EnvBoolVar::EnableTimeMeasurementForReddit => {
                EnvVar::get_env_name(EnvVar::EnableTimeMeasurementForReddit)
            }
            EnvBoolVar::EnableTimeMeasurementForTwitter => {
                EnvVar::get_env_name(EnvVar::EnableTimeMeasurementForTwitter)
            }

            EnvBoolVar::EnableInfoForArxiv => EnvVar::get_env_name(EnvVar::EnableInfoForArxiv),
            EnvBoolVar::EnableInfoForBiorxiv => EnvVar::get_env_name(EnvVar::EnableInfoForBiorxiv),
            EnvBoolVar::EnableInfoForGithub => EnvVar::get_env_name(EnvVar::EnableInfoForGithub),
            EnvBoolVar::EnableInfoForHabr => EnvVar::get_env_name(EnvVar::EnableInfoForHabr),
            EnvBoolVar::EnableInfoForMedrxiv => EnvVar::get_env_name(EnvVar::EnableInfoForMedrxiv),
            EnvBoolVar::EnableInfoForReddit => EnvVar::get_env_name(EnvVar::EnableInfoForReddit),
            EnvBoolVar::EnableInfoForTwitter => EnvVar::get_env_name(EnvVar::EnableInfoForTwitter),

            EnvBoolVar::EnableLinksLimitForArxiv => {
                EnvVar::get_env_name(EnvVar::EnableLinksLimitForArxiv)
            }
            EnvBoolVar::EnableLinksLimitForBiorxiv => {
                EnvVar::get_env_name(EnvVar::EnableLinksLimitForBiorxiv)
            }
            EnvBoolVar::EnableLinksLimitForGithub => {
                EnvVar::get_env_name(EnvVar::EnableLinksLimitForGithub)
            }
            EnvBoolVar::EnableLinksLimitForHabr => {
                EnvVar::get_env_name(EnvVar::EnableLinksLimitForHabr)
            }
            EnvBoolVar::EnableLinksLimitForMedrxiv => {
                EnvVar::get_env_name(EnvVar::EnableLinksLimitForMedrxiv)
            }
            EnvBoolVar::EnableLinksLimitForReddit => {
                EnvVar::get_env_name(EnvVar::EnableLinksLimitForReddit)
            }
            EnvBoolVar::EnableLinksLimitForTwitter => {
                EnvVar::get_env_name(EnvVar::EnableLinksLimitForTwitter)
            }

            EnvBoolVar::EnableRandomizeOrderForArxivLinkPartsForMongo => {
                EnvVar::get_env_name(EnvVar::EnableRandomizeOrderForArxivLinkPartsForMongo)
            }
            EnvBoolVar::EnableRandomizeOrderForBiorxivLinkPartsForMongo => {
                EnvVar::get_env_name(EnvVar::EnableRandomizeOrderForBiorxivLinkPartsForMongo)
            }
            EnvBoolVar::EnableRandomizeOrderForGithubLinkPartsForMongo => {
                EnvVar::get_env_name(EnvVar::EnableRandomizeOrderForGithubLinkPartsForMongo)
            }
            EnvBoolVar::EnableRandomizeOrderForHabrLinkPartsForMongo => {
                EnvVar::get_env_name(EnvVar::EnableRandomizeOrderForHabrLinkPartsForMongo)
            }
            EnvBoolVar::EnableRandomizeOrderForMedrxivLinkPartsForMongo => {
                EnvVar::get_env_name(EnvVar::EnableRandomizeOrderForMedrxivLinkPartsForMongo)
            }
            EnvBoolVar::EnableRandomizeOrderForRedditLinkPartsForMongo => {
                EnvVar::get_env_name(EnvVar::EnableRandomizeOrderForRedditLinkPartsForMongo)
            }
            EnvBoolVar::EnableRandomizeOrderForTwitterLinkPartsForMongo => {
                EnvVar::get_env_name(EnvVar::EnableRandomizeOrderForTwitterLinkPartsForMongo)
            }
        }
    }
    pub fn get_length() -> usize {
        ENUM_LENGTH
    }
    pub fn into_vec() -> Vec<EnvBoolVar> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvBoolVar::get_length());
        for env_var_name_kind in EnvBoolVar::iter() {
            env_var_name_kind_vec.push(env_var_name_kind);
        }
        env_var_name_kind_vec
    }
    pub fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, EnvBoolVar)> {
        let mut env_var_name_kind_vec = Vec::with_capacity(EnvBoolVar::get_length());
        for env_var_name_kind in EnvBoolVar::iter() {
            env_var_name_kind_vec.push((
                EnvBoolVar::get_env_name(env_var_name_kind),
                env_var_name_kind,
            ));
        }
        env_var_name_kind_vec
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, EnvBoolVar> {
        let mut config_env_var_name_kind_string_to_enum_struct_hasmap: HashMap<
            &'static str,
            EnvBoolVar,
        > = HashMap::with_capacity(EnvBoolVar::get_length());
        for env_var_name_kind_kind in EnvBoolVar::iter() {
            config_env_var_name_kind_string_to_enum_struct_hasmap.insert(
                EnvBoolVar::get_env_name(env_var_name_kind_kind),
                env_var_name_kind_kind,
            );
        }
        config_env_var_name_kind_string_to_enum_struct_hasmap
    }
    pub fn get_string_from_env_var(
        env_var_name_kind: EnvBoolVar,
        was_dotenv_enable: bool,
    ) -> Result<String, ConfigError<'static>> {
        let string_name = EnvBoolVar::get_env_name(env_var_name_kind);
        match std::env::var(string_name) {
            Ok(handle) => Ok(handle),
            Err(e) => {
                return Err(ConfigError {
                    env_var_name_kind: ConfigEnvVarErrorType::Bool(env_var_name_kind),
                    was_dotenv_enable,
                    env_name: string_name,
                    env_error: ConfigErrorInnerType::VarErrorHandle(e),
                })
            }
        }
    }
    pub fn get_env_values_hashmap() -> Result<HashMap<EnvBoolVar, bool>, ConfigError<'static>> {
        let was_dotenv_enable: bool;
        match dotenv() {
            Ok(_) => {
                was_dotenv_enable = true;
            }
            Err(e) => {
                was_dotenv_enable = false;
                println!(
                    "dotenv() failed, trying without {} error: {:?}",
                    ENV_FILE_NAME, e
                );
            }
        }
        let mut hmap: HashMap<EnvBoolVar, bool> = HashMap::new();
        let mut error_option: Option<ConfigError> = None;
        for env_var_name_kind in EnvBoolVar::iter() {
            match EnvBoolVar::get_string_from_env_var(env_var_name_kind, was_dotenv_enable) {
                Ok(env_var_string) => match env_var_string.parse::<bool>() {
                    Ok(handle) => {
                        hmap.insert(env_var_name_kind, handle);
                    }
                    Err(e) => {
                        error_option = Some(ConfigError {
                            env_var_name_kind: ConfigEnvVarErrorType::Bool(env_var_name_kind),
                            was_dotenv_enable,
                            env_name: EnvBoolVar::get_env_name(env_var_name_kind),
                            env_error: ConfigErrorInnerType::VarOrBoolParseErrorHandle(
                                VarOrBoolParseError::Bool(e),
                            ),
                        });
                        break;
                    }
                },
                Err(e) => {
                    error_option = Some(e);
                    break;
                }
            }
        }
        if let Some(error) = error_option {
            return Err(error);
        }
        Ok(hmap)
    }
}
