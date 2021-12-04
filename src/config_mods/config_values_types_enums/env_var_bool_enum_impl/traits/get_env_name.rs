use crate::config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar;
use crate::config_mods::env_var_enum::EnvVar;

use crate::traits::get_env_name_trait::GetEnvName;

impl GetEnvName for EnvBoolVar {
    fn get_env_name(self: &EnvBoolVar) -> &'static str {
        match self {
            EnvBoolVar::EnableProviders => EnvVar::EnableProviders.get_env_name(),
            EnvBoolVar::EnableCleaningWarningLogsDirectory => {
                EnvVar::EnableCleaningWarningLogsDirectory.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDbInMongo => {
                EnvVar::EnableCleaningWarningLogsDbInMongo.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongo => {
                EnvVar::EnableCleaningWarningLogsDbCollectionsInMongo.get_env_name()
            }
            EnvBoolVar::EnableTimeMeasurement => EnvVar::EnableTimeMeasurement.get_env_name(),
            EnvBoolVar::EnableProviderLinksLimit => EnvVar::EnableProviderLinksLimit.get_env_name(),
            EnvBoolVar::EnableCommonProvidersLinksLimit => {
                EnvVar::EnableCommonProvidersLinksLimit.get_env_name()
            }
            EnvBoolVar::EnableRandomizeOrderForProvidersLinkPartsForMongo => {
                EnvVar::EnableRandomizeOrderForProvidersLinkPartsForMongo.get_env_name()
            }
            EnvBoolVar::EnablePrints => EnvVar::EnablePrints.get_env_name(),
            EnvBoolVar::EnableErrorPrints => EnvVar::EnableErrorPrints.get_env_name(),
            EnvBoolVar::EnableWarningHighPrints => EnvVar::EnableWarningHighPrints.get_env_name(),
            EnvBoolVar::EnableWarningLowPrints => EnvVar::EnableWarningLowPrints.get_env_name(),
            EnvBoolVar::EnableSuccessPrints => EnvVar::EnableSuccessPrints.get_env_name(),
            EnvBoolVar::EnablePartialSuccessPrints => {
                EnvVar::EnablePartialSuccessPrints.get_env_name()
            }
            EnvBoolVar::EnableTimeMeasurementPrints => {
                EnvVar::EnableTimeMeasurementPrints.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDirectoryPrints => {
                EnvVar::EnableCleaningWarningLogsDirectoryPrints.get_env_name()
            }
            EnvBoolVar::EnableInfoPrints => EnvVar::EnableInfoPrints.get_env_name(),
            EnvBoolVar::EnableAllProvidersPrints => EnvVar::EnableAllProvidersPrints.get_env_name(),
            EnvBoolVar::EnableErrorPrintsForAllProviders => {
                EnvVar::EnableErrorPrintsForAllProviders.get_env_name()
            }
            EnvBoolVar::EnableWarningHighPrintsForAllProviders => {
                EnvVar::EnableWarningHighPrintsForAllProviders.get_env_name()
            }
            EnvBoolVar::EnableWarningLowPrintsForAllProviders => {
                EnvVar::EnableWarningLowPrintsForAllProviders.get_env_name()
            }
            EnvBoolVar::EnableSuccessPrintsForAllProviders => {
                EnvVar::EnableSuccessPrintsForAllProviders.get_env_name()
            }
            EnvBoolVar::EnablePartialSuccessPrintsForAllProviders => {
                EnvVar::EnablePartialSuccessPrintsForAllProviders.get_env_name()
            }
            EnvBoolVar::EnableTimeMeasurementPrintsForAllProviders => {
                EnvVar::EnableTimeMeasurementPrintsForAllProviders.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDirectoryPrintsForAllProviders => {
                EnvVar::EnableCleaningWarningLogsDirectoryPrintsForAllProviders.get_env_name()
            }
            EnvBoolVar::EnableInfoPrintsForAllProviders => {
                EnvVar::EnableInfoPrintsForAllProviders.get_env_name()
            }
            EnvBoolVar::EnableWriteErrorLogsInLocalFolder => {
                EnvVar::EnableWriteErrorLogsInLocalFolder.get_env_name()
            }
            EnvBoolVar::EnableWriteErrorLogsInMongo => {
                EnvVar::EnableWriteErrorLogsInMongo.get_env_name()
            }
            EnvBoolVar::EnableInitializeMongoWithProvidersLinkParts => {
                EnvVar::EnableInitializeMongoWithProvidersLinkParts.get_env_name()
            }

            EnvBoolVar::EnableInitializeMongoWithArxivLinkParts => {
                EnvVar::EnableInitializeMongoWithArxivLinkParts.get_env_name()
            }
            EnvBoolVar::EnableInitializeMongoWithBiorxivLinkParts => {
                EnvVar::EnableInitializeMongoWithBiorxivLinkParts.get_env_name()
            }
            EnvBoolVar::EnableInitializeMongoWithGithubLinkParts => {
                EnvVar::EnableInitializeMongoWithGithubLinkParts.get_env_name()
            }
            EnvBoolVar::EnableInitializeMongoWithHabrLinkParts => {
                EnvVar::EnableInitializeMongoWithHabrLinkParts.get_env_name()
            }
            EnvBoolVar::EnableInitializeMongoWithMedrxivLinkParts => {
                EnvVar::EnableInitializeMongoWithMedrxivLinkParts.get_env_name()
            }
            EnvBoolVar::EnableInitializeMongoWithRedditLinkParts => {
                EnvVar::EnableInitializeMongoWithRedditLinkParts.get_env_name()
            }
            EnvBoolVar::EnableInitializeMongoWithTwitterLinkParts => {
                EnvVar::EnableInitializeMongoWithTwitterLinkParts.get_env_name()
            }

            EnvBoolVar::EnableArxiv => EnvVar::EnableArxiv.get_env_name(),
            EnvBoolVar::EnableBiorxiv => EnvVar::EnableBiorxiv.get_env_name(),
            EnvBoolVar::EnableGithub => EnvVar::EnableGithub.get_env_name(),
            EnvBoolVar::EnableHabr => EnvVar::EnableHabr.get_env_name(),
            EnvBoolVar::EnableMedrxiv => EnvVar::EnableMedrxiv.get_env_name(),
            EnvBoolVar::EnableReddit => EnvVar::EnableReddit.get_env_name(),
            EnvBoolVar::EnableTwitter => EnvVar::EnableTwitter.get_env_name(),

            EnvBoolVar::EnablePrintsArxiv => EnvVar::EnablePrintsArxiv.get_env_name(),
            EnvBoolVar::EnablePrintsBiorxiv => EnvVar::EnablePrintsBiorxiv.get_env_name(),
            EnvBoolVar::EnablePrintsGithub => EnvVar::EnablePrintsGithub.get_env_name(),
            EnvBoolVar::EnablePrintsHabr => EnvVar::EnablePrintsHabr.get_env_name(),
            EnvBoolVar::EnablePrintsMedrxiv => EnvVar::EnablePrintsMedrxiv.get_env_name(),
            EnvBoolVar::EnablePrintsReddit => EnvVar::EnablePrintsReddit.get_env_name(),
            EnvBoolVar::EnablePrintsTwitter => EnvVar::EnablePrintsTwitter.get_env_name(),

            EnvBoolVar::EnableWarningHighPrintsForArxiv => {
                EnvVar::EnableWarningHighPrintsForArxiv.get_env_name()
            }
            EnvBoolVar::EnableWarningHighPrintsForBiorxiv => {
                EnvVar::EnableWarningHighPrintsForBiorxiv.get_env_name()
            }
            EnvBoolVar::EnableWarningHighPrintsForGithub => {
                EnvVar::EnableWarningHighPrintsForGithub.get_env_name()
            }
            EnvBoolVar::EnableWarningHighPrintsForHabr => {
                EnvVar::EnableWarningHighPrintsForHabr.get_env_name()
            }
            EnvBoolVar::EnableWarningHighPrintsForMedrxiv => {
                EnvVar::EnableWarningHighPrintsForMedrxiv.get_env_name()
            }
            EnvBoolVar::EnableWarningHighPrintsForReddit => {
                EnvVar::EnableWarningHighPrintsForReddit.get_env_name()
            }
            EnvBoolVar::EnableWarningHighPrintsForTwitter => {
                EnvVar::EnableWarningHighPrintsForTwitter.get_env_name()
            }

            EnvBoolVar::EnableWarningLowPrintsForArxiv => {
                EnvVar::EnableWarningLowPrintsForArxiv.get_env_name()
            }
            EnvBoolVar::EnableWarningLowPrintsForBiorxiv => {
                EnvVar::EnableWarningLowPrintsForBiorxiv.get_env_name()
            }
            EnvBoolVar::EnableWarningLowPrintsForGithub => {
                EnvVar::EnableWarningLowPrintsForGithub.get_env_name()
            }
            EnvBoolVar::EnableWarningLowPrintsForHabr => {
                EnvVar::EnableWarningLowPrintsForHabr.get_env_name()
            }
            EnvBoolVar::EnableWarningLowPrintsForMedrxiv => {
                EnvVar::EnableWarningLowPrintsForMedrxiv.get_env_name()
            }
            EnvBoolVar::EnableWarningLowPrintsForReddit => {
                EnvVar::EnableWarningLowPrintsForReddit.get_env_name()
            }
            EnvBoolVar::EnableWarningLowPrintsForTwitter => {
                EnvVar::EnableWarningLowPrintsForTwitter.get_env_name()
            }

            EnvBoolVar::EnableErrorPrintsForArxiv => {
                EnvVar::EnableErrorPrintsForArxiv.get_env_name()
            }
            EnvBoolVar::EnableErrorPrintsForBiorxiv => {
                EnvVar::EnableErrorPrintsForBiorxiv.get_env_name()
            }
            EnvBoolVar::EnableErrorPrintsForGithub => {
                EnvVar::EnableErrorPrintsForGithub.get_env_name()
            }
            EnvBoolVar::EnableErrorPrintsForHabr => EnvVar::EnableErrorPrintsForHabr.get_env_name(),
            EnvBoolVar::EnableErrorPrintsForMedrxiv => {
                EnvVar::EnableErrorPrintsForMedrxiv.get_env_name()
            }
            EnvBoolVar::EnableErrorPrintsForReddit => {
                EnvVar::EnableErrorPrintsForReddit.get_env_name()
            }
            EnvBoolVar::EnableErrorPrintsForTwitter => {
                EnvVar::EnableErrorPrintsForTwitter.get_env_name()
            }

            EnvBoolVar::EnableSuccessPrintsForArxiv => {
                EnvVar::EnableSuccessPrintsForArxiv.get_env_name()
            }
            EnvBoolVar::EnableSuccessPrintsForBiorxiv => {
                EnvVar::EnableSuccessPrintsForBiorxiv.get_env_name()
            }
            EnvBoolVar::EnableSuccessPrintsForGithub => {
                EnvVar::EnableSuccessPrintsForGithub.get_env_name()
            }
            EnvBoolVar::EnableSuccessPrintsForHabr => {
                EnvVar::EnableSuccessPrintsForHabr.get_env_name()
            }
            EnvBoolVar::EnableSuccessPrintsForMedrxiv => {
                EnvVar::EnableSuccessPrintsForMedrxiv.get_env_name()
            }
            EnvBoolVar::EnableSuccessPrintsForReddit => {
                EnvVar::EnableSuccessPrintsForReddit.get_env_name()
            }
            EnvBoolVar::EnableSuccessPrintsForTwitter => {
                EnvVar::EnableSuccessPrintsForTwitter.get_env_name()
            }

            EnvBoolVar::EnablePartialSuccessPrintsForArxiv => {
                EnvVar::EnablePartialSuccessPrintsForArxiv.get_env_name()
            }
            EnvBoolVar::EnablePartialSuccessPrintsForBiorxiv => {
                EnvVar::EnablePartialSuccessPrintsForBiorxiv.get_env_name()
            }
            EnvBoolVar::EnablePartialSuccessPrintsForGithub => {
                EnvVar::EnablePartialSuccessPrintsForGithub.get_env_name()
            }
            EnvBoolVar::EnablePartialSuccessPrintsForHabr => {
                EnvVar::EnablePartialSuccessPrintsForHabr.get_env_name()
            }
            EnvBoolVar::EnablePartialSuccessPrintsForMedrxiv => {
                EnvVar::EnablePartialSuccessPrintsForMedrxiv.get_env_name()
            }
            EnvBoolVar::EnablePartialSuccessPrintsForReddit => {
                EnvVar::EnablePartialSuccessPrintsForReddit.get_env_name()
            }
            EnvBoolVar::EnablePartialSuccessPrintsForTwitter => {
                EnvVar::EnablePartialSuccessPrintsForTwitter.get_env_name()
            }

            EnvBoolVar::EnableCleaningWarningLogsDirectoryForArxiv => {
                EnvVar::EnableCleaningWarningLogsDirectoryForArxiv.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForBiorxiv => {
                EnvVar::EnableCleaningWarningLogsDirectoryForBiorxiv.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForGithub => {
                EnvVar::EnableCleaningWarningLogsDirectoryForGithub.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForHabr => {
                EnvVar::EnableCleaningWarningLogsDirectoryForHabr.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForMedrxiv => {
                EnvVar::EnableCleaningWarningLogsDirectoryForMedrxiv.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForReddit => {
                EnvVar::EnableCleaningWarningLogsDirectoryForReddit.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDirectoryForTwitter => {
                EnvVar::EnableCleaningWarningLogsDirectoryForTwitter.get_env_name()
            }

            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForArxiv => {
                EnvVar::EnableCleaningWarningLogsDbInMongoForArxiv.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForBiorxiv => {
                EnvVar::EnableCleaningWarningLogsDbInMongoForBiorxiv.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForGithub => {
                EnvVar::EnableCleaningWarningLogsDbInMongoForGithub.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForHabr => {
                EnvVar::EnableCleaningWarningLogsDbInMongoForHabr.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForMedrxiv => {
                EnvVar::EnableCleaningWarningLogsDbInMongoForMedrxiv.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForReddit => {
                EnvVar::EnableCleaningWarningLogsDbInMongoForReddit.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDbInMongoForTwitter => {
                EnvVar::EnableCleaningWarningLogsDbInMongoForTwitter.get_env_name()
            }

            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForArxiv => {
                EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForArxiv.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxiv => {
                EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxiv.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForGithub => {
                EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForGithub.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForHabr => {
                EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForHabr.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxiv => {
                EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxiv.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForReddit => {
                EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForReddit.get_env_name()
            }
            EnvBoolVar::EnableCleaningWarningLogsDbCollectionsInMongoForTwitter => {
                EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForTwitter.get_env_name()
            }

            EnvBoolVar::EnableTimeMeasurementForArxiv => {
                EnvVar::EnableTimeMeasurementForArxiv.get_env_name()
            }
            EnvBoolVar::EnableTimeMeasurementForBiorxiv => {
                EnvVar::EnableTimeMeasurementForBiorxiv.get_env_name()
            }
            EnvBoolVar::EnableTimeMeasurementForGithub => {
                EnvVar::EnableTimeMeasurementForGithub.get_env_name()
            }
            EnvBoolVar::EnableTimeMeasurementForHabr => {
                EnvVar::EnableTimeMeasurementForHabr.get_env_name()
            }
            EnvBoolVar::EnableTimeMeasurementForMedrxiv => {
                EnvVar::EnableTimeMeasurementForMedrxiv.get_env_name()
            }
            EnvBoolVar::EnableTimeMeasurementForReddit => {
                EnvVar::EnableTimeMeasurementForReddit.get_env_name()
            }
            EnvBoolVar::EnableTimeMeasurementForTwitter => {
                EnvVar::EnableTimeMeasurementForTwitter.get_env_name()
            }

            EnvBoolVar::EnableInfoForArxiv => EnvVar::EnableInfoForArxiv.get_env_name(),
            EnvBoolVar::EnableInfoForBiorxiv => EnvVar::EnableInfoForBiorxiv.get_env_name(),
            EnvBoolVar::EnableInfoForGithub => EnvVar::EnableInfoForGithub.get_env_name(),
            EnvBoolVar::EnableInfoForHabr => EnvVar::EnableInfoForHabr.get_env_name(),
            EnvBoolVar::EnableInfoForMedrxiv => EnvVar::EnableInfoForMedrxiv.get_env_name(),
            EnvBoolVar::EnableInfoForReddit => EnvVar::EnableInfoForReddit.get_env_name(),
            EnvBoolVar::EnableInfoForTwitter => EnvVar::EnableInfoForTwitter.get_env_name(),

            EnvBoolVar::EnableLinksLimitForArxiv => EnvVar::EnableLinksLimitForArxiv.get_env_name(),
            EnvBoolVar::EnableLinksLimitForBiorxiv => {
                EnvVar::EnableLinksLimitForBiorxiv.get_env_name()
            }
            EnvBoolVar::EnableLinksLimitForGithub => {
                EnvVar::EnableLinksLimitForGithub.get_env_name()
            }
            EnvBoolVar::EnableLinksLimitForHabr => EnvVar::EnableLinksLimitForHabr.get_env_name(),
            EnvBoolVar::EnableLinksLimitForMedrxiv => {
                EnvVar::EnableLinksLimitForMedrxiv.get_env_name()
            }
            EnvBoolVar::EnableLinksLimitForReddit => {
                EnvVar::EnableLinksLimitForReddit.get_env_name()
            }
            EnvBoolVar::EnableLinksLimitForTwitter => {
                EnvVar::EnableLinksLimitForTwitter.get_env_name()
            }

            EnvBoolVar::EnableRandomizeOrderForArxivLinkPartsForMongo => {
                EnvVar::EnableRandomizeOrderForArxivLinkPartsForMongo.get_env_name()
            }
            EnvBoolVar::EnableRandomizeOrderForBiorxivLinkPartsForMongo => {
                EnvVar::EnableRandomizeOrderForBiorxivLinkPartsForMongo.get_env_name()
            }
            EnvBoolVar::EnableRandomizeOrderForGithubLinkPartsForMongo => {
                EnvVar::EnableRandomizeOrderForGithubLinkPartsForMongo.get_env_name()
            }
            EnvBoolVar::EnableRandomizeOrderForHabrLinkPartsForMongo => {
                EnvVar::EnableRandomizeOrderForHabrLinkPartsForMongo.get_env_name()
            }
            EnvBoolVar::EnableRandomizeOrderForMedrxivLinkPartsForMongo => {
                EnvVar::EnableRandomizeOrderForMedrxivLinkPartsForMongo.get_env_name()
            }
            EnvBoolVar::EnableRandomizeOrderForRedditLinkPartsForMongo => {
                EnvVar::EnableRandomizeOrderForRedditLinkPartsForMongo.get_env_name()
            }
            EnvBoolVar::EnableRandomizeOrderForTwitterLinkPartsForMongo => {
                EnvVar::EnableRandomizeOrderForTwitterLinkPartsForMongo.get_env_name()
            }
        }
    }
    fn into_array() -> &'static [Self] {
        EnvBoolVar::all_variants()
    }
}
