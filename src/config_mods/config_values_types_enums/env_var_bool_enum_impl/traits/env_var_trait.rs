use crate::config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar;
use crate::config_mods::env_var_enum::EnvVar;

use crate::traits::env_var_trait::EnvVarTrait;

impl EnvVarTrait for EnvBoolVar {
    fn get_env_name(&self) -> String {
        match &self {
            Self::DbsEnableInitialization => EnvVar::DbsEnableInitialization.get_env_name(),

            Self::MongoEnableInitialization => EnvVar::MongoEnableInitialization.get_env_name(),
            Self::MongoEnableInitializationForProviders => {
                EnvVar::MongoEnableInitializationForProviders.get_env_name()
            }
            Self::MongoEnableInitializationForArxiv => {
                EnvVar::MongoEnableInitializationForArxiv.get_env_name()
            }
            Self::MongoEnableInitializationForBiorxiv => {
                EnvVar::MongoEnableInitializationForBiorxiv.get_env_name()
            }
            Self::MongoEnableInitializationForGithub => {
                EnvVar::MongoEnableInitializationForGithub.get_env_name()
            }
            Self::MongoEnableInitializationForHabr => {
                EnvVar::MongoEnableInitializationForHabr.get_env_name()
            }
            Self::MongoEnableInitializationForMedrxiv => {
                EnvVar::MongoEnableInitializationForMedrxiv.get_env_name()
            }
            Self::MongoEnableInitializationForReddit => {
                EnvVar::MongoEnableInitializationForReddit.get_env_name()
            }
            Self::MongoEnableInitializationForTwitter => {
                EnvVar::MongoEnableInitializationForTwitter.get_env_name()
            }

            Self::MongoEnableWriteErrorLogs => EnvVar::MongoEnableWriteErrorLogs.get_env_name(),
            Self::MongoEnableWriteErrorLogsForProviders => {
                EnvVar::MongoEnableWriteErrorLogsForProviders.get_env_name()
            }
            Self::MongoEnableWriteErrorLogsForArxiv => {
                EnvVar::MongoEnableWriteErrorLogsForArxiv.get_env_name()
            }
            Self::MongoEnableWriteErrorLogsForBiorxiv => {
                EnvVar::MongoEnableWriteErrorLogsForBiorxiv.get_env_name()
            }
            Self::MongoEnableWriteErrorLogsForGithub => {
                EnvVar::MongoEnableWriteErrorLogsForGithub.get_env_name()
            }
            Self::MongoEnableWriteErrorLogsForHabr => {
                EnvVar::MongoEnableWriteErrorLogsForHabr.get_env_name()
            }
            Self::MongoEnableWriteErrorLogsForMedrxiv => {
                EnvVar::MongoEnableWriteErrorLogsForMedrxiv.get_env_name()
            }
            Self::MongoEnableWriteErrorLogsForReddit => {
                EnvVar::MongoEnableWriteErrorLogsForReddit.get_env_name()
            }
            Self::MongoEnableWriteErrorLogsForTwitter => {
                EnvVar::MongoEnableWriteErrorLogsForTwitter.get_env_name()
            }

            Self::MongoEnableCleaningWarningLogsDb => {
                EnvVar::MongoEnableCleaningWarningLogsDb.get_env_name()
            }
            Self::MongoEnableCleaningWarningLogsDbForProviders => {
                EnvVar::MongoEnableCleaningWarningLogsDbForProviders.get_env_name()
            }
            Self::MongoEnableCleaningWarningLogsDbForArxiv => {
                EnvVar::MongoEnableCleaningWarningLogsDbForArxiv.get_env_name()
            }
            Self::MongoEnableCleaningWarningLogsDbForBiorxiv => {
                EnvVar::MongoEnableCleaningWarningLogsDbForBiorxiv.get_env_name()
            }
            Self::MongoEnableCleaningWarningLogsDbForGithub => {
                EnvVar::MongoEnableCleaningWarningLogsDbForGithub.get_env_name()
            }
            Self::MongoEnableCleaningWarningLogsDbForHabr => {
                EnvVar::MongoEnableCleaningWarningLogsDbForHabr.get_env_name()
            }
            Self::MongoEnableCleaningWarningLogsDbForMedrxiv => {
                EnvVar::MongoEnableCleaningWarningLogsDbForMedrxiv.get_env_name()
            }
            Self::MongoEnableCleaningWarningLogsDbForReddit => {
                EnvVar::MongoEnableCleaningWarningLogsDbForReddit.get_env_name()
            }
            Self::MongoEnableCleaningWarningLogsDbForTwitter => {
                EnvVar::MongoEnableCleaningWarningLogsDbForTwitter.get_env_name()
            }

            Self::MongoEnableCleaningWarningLogsDbCollections => {
                EnvVar::MongoEnableCleaningWarningLogsDbCollections.get_env_name()
            }
            Self::MongoEnableCleaningWarningLogsDbCollectionsForProviders => {
                EnvVar::MongoEnableCleaningWarningLogsDbCollectionsForProviders.get_env_name()
            }
            Self::MongoEnableCleaningWarningLogsDbCollectionsForArxiv => {
                EnvVar::MongoEnableCleaningWarningLogsDbCollectionsForArxiv.get_env_name()
            }
            Self::MongoEnableCleaningWarningLogsDbCollectionsForBiorxiv => {
                EnvVar::MongoEnableCleaningWarningLogsDbCollectionsForBiorxiv.get_env_name()
            }
            Self::MongoEnableCleaningWarningLogsDbCollectionsForGithub => {
                EnvVar::MongoEnableCleaningWarningLogsDbCollectionsForGithub.get_env_name()
            }
            Self::MongoEnableCleaningWarningLogsDbCollectionsForHabr => {
                EnvVar::MongoEnableCleaningWarningLogsDbCollectionsForHabr.get_env_name()
            }
            Self::MongoEnableCleaningWarningLogsDbCollectionsForMedrxiv => {
                EnvVar::MongoEnableCleaningWarningLogsDbCollectionsForMedrxiv.get_env_name()
            }
            Self::MongoEnableCleaningWarningLogsDbCollectionsForReddit => {
                EnvVar::MongoEnableCleaningWarningLogsDbCollectionsForReddit.get_env_name()
            }
            Self::MongoEnableCleaningWarningLogsDbCollectionsForTwitter => {
                EnvVar::MongoEnableCleaningWarningLogsDbCollectionsForTwitter.get_env_name()
            }

            Self::MongoEnableLinkPartsRandomizeOrder => {
                EnvVar::MongoEnableLinkPartsRandomizeOrder.get_env_name()
            }
            Self::MongoEnableLinkPartsRandomizeOrderForProviders => {
                EnvVar::MongoEnableLinkPartsRandomizeOrderForProviders.get_env_name()
            }
            Self::MongoEnableLinkPartsRandomizeOrderForArxiv => {
                EnvVar::MongoEnableLinkPartsRandomizeOrderForArxiv.get_env_name()
            }
            Self::MongoEnableLinkPartsRandomizeOrderForBiorxiv => {
                EnvVar::MongoEnableLinkPartsRandomizeOrderForBiorxiv.get_env_name()
            }
            Self::MongoEnableLinkPartsRandomizeOrderForGithub => {
                EnvVar::MongoEnableLinkPartsRandomizeOrderForGithub.get_env_name()
            }
            Self::MongoEnableLinkPartsRandomizeOrderForHabr => {
                EnvVar::MongoEnableLinkPartsRandomizeOrderForHabr.get_env_name()
            }
            Self::MongoEnableLinkPartsRandomizeOrderForMedrxiv => {
                EnvVar::MongoEnableLinkPartsRandomizeOrderForMedrxiv.get_env_name()
            }
            Self::MongoEnableLinkPartsRandomizeOrderForReddit => {
                EnvVar::MongoEnableLinkPartsRandomizeOrderForReddit.get_env_name()
            }
            Self::MongoEnableLinkPartsRandomizeOrderForTwitter => {
                EnvVar::MongoEnableLinkPartsRandomizeOrderForTwitter.get_env_name()
            }

            Self::PostgresEnableInitialization => {
                EnvVar::PostgresEnableInitialization.get_env_name()
            }
            Self::PostgresEnableInitializationForProviders => {
                EnvVar::PostgresEnableInitializationForProviders.get_env_name()
            }
            Self::PostgresEnableInitializationForArxiv => {
                EnvVar::PostgresEnableInitializationForArxiv.get_env_name()
            }
            Self::PostgresEnableInitializationForBiorxiv => {
                EnvVar::PostgresEnableInitializationForBiorxiv.get_env_name()
            }
            Self::PostgresEnableInitializationForGithub => {
                EnvVar::PostgresEnableInitializationForGithub.get_env_name()
            }
            Self::PostgresEnableInitializationForHabr => {
                EnvVar::PostgresEnableInitializationForHabr.get_env_name()
            }
            Self::PostgresEnableInitializationForMedrxiv => {
                EnvVar::PostgresEnableInitializationForMedrxiv.get_env_name()
            }
            Self::PostgresEnableInitializationForReddit => {
                EnvVar::PostgresEnableInitializationForReddit.get_env_name()
            }
            Self::PostgresEnableInitializationForTwitter => {
                EnvVar::PostgresEnableInitializationForTwitter.get_env_name()
            }

            Self::EnableWriteErrorLogsInLocalFolder => {
                EnvVar::EnableWriteErrorLogsInLocalFolder.get_env_name()
            }
            Self::EnableWriteErrorLogsInLocalFolderForProviders => {
                EnvVar::EnableWriteErrorLogsInLocalFolderForProviders.get_env_name()
            }
            Self::EnableWriteErrorLogsInLocalFolderForArxiv => {
                EnvVar::EnableWriteErrorLogsInLocalFolderForArxiv.get_env_name()
            }
            Self::EnableWriteErrorLogsInLocalFolderForBiorxiv => {
                EnvVar::EnableWriteErrorLogsInLocalFolderForBiorxiv.get_env_name()
            }
            Self::EnableWriteErrorLogsInLocalFolderForGithub => {
                EnvVar::EnableWriteErrorLogsInLocalFolderForGithub.get_env_name()
            }
            Self::EnableWriteErrorLogsInLocalFolderForHabr => {
                EnvVar::EnableWriteErrorLogsInLocalFolderForHabr.get_env_name()
            }
            Self::EnableWriteErrorLogsInLocalFolderForMedrxiv => {
                EnvVar::EnableWriteErrorLogsInLocalFolderForMedrxiv.get_env_name()
            }
            Self::EnableWriteErrorLogsInLocalFolderForReddit => {
                EnvVar::EnableWriteErrorLogsInLocalFolderForReddit.get_env_name()
            }
            Self::EnableWriteErrorLogsInLocalFolderForTwitter => {
                EnvVar::EnableWriteErrorLogsInLocalFolderForTwitter.get_env_name()
            }

            Self::EnableCleaningWarningLogsDirectory => {
                EnvVar::EnableCleaningWarningLogsDirectory.get_env_name()
            }
            Self::EnableCleaningWarningLogsDirectoryForProviders => {
                EnvVar::EnableCleaningWarningLogsDirectoryForProviders.get_env_name()
            }
            Self::EnableCleaningWarningLogsDirectoryForArxiv => {
                EnvVar::EnableCleaningWarningLogsDirectoryForArxiv.get_env_name()
            }
            Self::EnableCleaningWarningLogsDirectoryForBiorxiv => {
                EnvVar::EnableCleaningWarningLogsDirectoryForBiorxiv.get_env_name()
            }
            Self::EnableCleaningWarningLogsDirectoryForGithub => {
                EnvVar::EnableCleaningWarningLogsDirectoryForGithub.get_env_name()
            }
            Self::EnableCleaningWarningLogsDirectoryForHabr => {
                EnvVar::EnableCleaningWarningLogsDirectoryForHabr.get_env_name()
            }
            Self::EnableCleaningWarningLogsDirectoryForMedrxiv => {
                EnvVar::EnableCleaningWarningLogsDirectoryForMedrxiv.get_env_name()
            }
            Self::EnableCleaningWarningLogsDirectoryForReddit => {
                EnvVar::EnableCleaningWarningLogsDirectoryForReddit.get_env_name()
            }
            Self::EnableCleaningWarningLogsDirectoryForTwitter => {
                EnvVar::EnableCleaningWarningLogsDirectoryForTwitter.get_env_name()
            }

            Self::EnableProviders => EnvVar::EnableProviders.get_env_name(),
            Self::EnableArxiv => EnvVar::EnableArxiv.get_env_name(),
            Self::EnableBiorxiv => EnvVar::EnableBiorxiv.get_env_name(),
            Self::EnableGithub => EnvVar::EnableGithub.get_env_name(),
            Self::EnableHabr => EnvVar::EnableHabr.get_env_name(),
            Self::EnableMedrxiv => EnvVar::EnableMedrxiv.get_env_name(),
            Self::EnableReddit => EnvVar::EnableReddit.get_env_name(),
            Self::EnableTwitter => EnvVar::EnableTwitter.get_env_name(),

            Self::EnablePrints => EnvVar::EnablePrints.get_env_name(),
            Self::EnablePrintsForProviders => EnvVar::EnablePrintsForProviders.get_env_name(),
            Self::EnablePrintsArxiv => EnvVar::EnablePrintsArxiv.get_env_name(),
            Self::EnablePrintsBiorxiv => EnvVar::EnablePrintsBiorxiv.get_env_name(),
            Self::EnablePrintsGithub => EnvVar::EnablePrintsGithub.get_env_name(),
            Self::EnablePrintsHabr => EnvVar::EnablePrintsHabr.get_env_name(),
            Self::EnablePrintsMedrxiv => EnvVar::EnablePrintsMedrxiv.get_env_name(),
            Self::EnablePrintsReddit => EnvVar::EnablePrintsReddit.get_env_name(),
            Self::EnablePrintsTwitter => EnvVar::EnablePrintsTwitter.get_env_name(),

            Self::EnableWarningHighPrints => EnvVar::EnableWarningHighPrints.get_env_name(),
            Self::EnableWarningHighPrintsForProviders => {
                EnvVar::EnableWarningHighPrintsForProviders.get_env_name()
            }
            Self::EnableWarningHighPrintsForArxiv => {
                EnvVar::EnableWarningHighPrintsForArxiv.get_env_name()
            }
            Self::EnableWarningHighPrintsForBiorxiv => {
                EnvVar::EnableWarningHighPrintsForBiorxiv.get_env_name()
            }
            Self::EnableWarningHighPrintsForGithub => {
                EnvVar::EnableWarningHighPrintsForGithub.get_env_name()
            }
            Self::EnableWarningHighPrintsForHabr => {
                EnvVar::EnableWarningHighPrintsForHabr.get_env_name()
            }
            Self::EnableWarningHighPrintsForMedrxiv => {
                EnvVar::EnableWarningHighPrintsForMedrxiv.get_env_name()
            }
            Self::EnableWarningHighPrintsForReddit => {
                EnvVar::EnableWarningHighPrintsForReddit.get_env_name()
            }
            Self::EnableWarningHighPrintsForTwitter => {
                EnvVar::EnableWarningHighPrintsForTwitter.get_env_name()
            }

            Self::EnableWarningLowPrints => EnvVar::EnableWarningLowPrints.get_env_name(),
            Self::EnableWarningLowPrintsForProviders => {
                EnvVar::EnableWarningLowPrintsForProviders.get_env_name()
            }
            Self::EnableWarningLowPrintsForArxiv => {
                EnvVar::EnableWarningLowPrintsForArxiv.get_env_name()
            }
            Self::EnableWarningLowPrintsForBiorxiv => {
                EnvVar::EnableWarningLowPrintsForBiorxiv.get_env_name()
            }
            Self::EnableWarningLowPrintsForGithub => {
                EnvVar::EnableWarningLowPrintsForGithub.get_env_name()
            }
            Self::EnableWarningLowPrintsForHabr => {
                EnvVar::EnableWarningLowPrintsForHabr.get_env_name()
            }
            Self::EnableWarningLowPrintsForMedrxiv => {
                EnvVar::EnableWarningLowPrintsForMedrxiv.get_env_name()
            }
            Self::EnableWarningLowPrintsForReddit => {
                EnvVar::EnableWarningLowPrintsForReddit.get_env_name()
            }
            Self::EnableWarningLowPrintsForTwitter => {
                EnvVar::EnableWarningLowPrintsForTwitter.get_env_name()
            }

            Self::EnableSuccessPrints => EnvVar::EnableSuccessPrints.get_env_name(),
            Self::EnableSuccessPrintsForProviders => {
                EnvVar::EnableSuccessPrintsForProviders.get_env_name()
            }
            Self::EnableSuccessPrintsForArxiv => EnvVar::EnableSuccessPrintsForArxiv.get_env_name(),
            Self::EnableSuccessPrintsForBiorxiv => {
                EnvVar::EnableSuccessPrintsForBiorxiv.get_env_name()
            }
            Self::EnableSuccessPrintsForGithub => {
                EnvVar::EnableSuccessPrintsForGithub.get_env_name()
            }
            Self::EnableSuccessPrintsForHabr => EnvVar::EnableSuccessPrintsForHabr.get_env_name(),
            Self::EnableSuccessPrintsForMedrxiv => {
                EnvVar::EnableSuccessPrintsForMedrxiv.get_env_name()
            }
            Self::EnableSuccessPrintsForReddit => {
                EnvVar::EnableSuccessPrintsForReddit.get_env_name()
            }
            Self::EnableSuccessPrintsForTwitter => {
                EnvVar::EnableSuccessPrintsForTwitter.get_env_name()
            }

            Self::EnablePartialSuccessPrints => EnvVar::EnablePartialSuccessPrints.get_env_name(),
            Self::EnablePartialSuccessPrintsForProviders => {
                EnvVar::EnablePartialSuccessPrintsForProviders.get_env_name()
            }
            Self::EnablePartialSuccessPrintsForArxiv => {
                EnvVar::EnablePartialSuccessPrintsForArxiv.get_env_name()
            }
            Self::EnablePartialSuccessPrintsForBiorxiv => {
                EnvVar::EnablePartialSuccessPrintsForBiorxiv.get_env_name()
            }
            Self::EnablePartialSuccessPrintsForGithub => {
                EnvVar::EnablePartialSuccessPrintsForGithub.get_env_name()
            }
            Self::EnablePartialSuccessPrintsForHabr => {
                EnvVar::EnablePartialSuccessPrintsForHabr.get_env_name()
            }
            Self::EnablePartialSuccessPrintsForMedrxiv => {
                EnvVar::EnablePartialSuccessPrintsForMedrxiv.get_env_name()
            }
            Self::EnablePartialSuccessPrintsForReddit => {
                EnvVar::EnablePartialSuccessPrintsForReddit.get_env_name()
            }
            Self::EnablePartialSuccessPrintsForTwitter => {
                EnvVar::EnablePartialSuccessPrintsForTwitter.get_env_name()
            }

            Self::EnableErrorPrints => EnvVar::EnableErrorPrints.get_env_name(),
            Self::EnableErrorPrintsForProviders => {
                EnvVar::EnableErrorPrintsForProviders.get_env_name()
            }
            Self::EnableErrorPrintsForArxiv => EnvVar::EnableErrorPrintsForArxiv.get_env_name(),
            Self::EnableErrorPrintsForBiorxiv => EnvVar::EnableErrorPrintsForBiorxiv.get_env_name(),
            Self::EnableErrorPrintsForGithub => EnvVar::EnableErrorPrintsForGithub.get_env_name(),
            Self::EnableErrorPrintsForHabr => EnvVar::EnableErrorPrintsForHabr.get_env_name(),
            Self::EnableErrorPrintsForMedrxiv => EnvVar::EnableErrorPrintsForMedrxiv.get_env_name(),
            Self::EnableErrorPrintsForReddit => EnvVar::EnableErrorPrintsForReddit.get_env_name(),
            Self::EnableErrorPrintsForTwitter => EnvVar::EnableErrorPrintsForTwitter.get_env_name(),

            Self::EnableTimeMeasurementPrints => EnvVar::EnableTimeMeasurementPrints.get_env_name(),
            Self::EnableTimeMeasurementPrintsForProviders => {
                EnvVar::EnableTimeMeasurementPrintsForProviders.get_env_name()
            }
            Self::EnableTimeMeasurementPrintsForArxiv => {
                EnvVar::EnableTimeMeasurementPrintsForArxiv.get_env_name()
            }
            Self::EnableTimeMeasurementPrintsForBiorxiv => {
                EnvVar::EnableTimeMeasurementPrintsForBiorxiv.get_env_name()
            }
            Self::EnableTimeMeasurementPrintsForGithub => {
                EnvVar::EnableTimeMeasurementPrintsForGithub.get_env_name()
            }
            Self::EnableTimeMeasurementPrintsForHabr => {
                EnvVar::EnableTimeMeasurementPrintsForHabr.get_env_name()
            }
            Self::EnableTimeMeasurementPrintsForMedrxiv => {
                EnvVar::EnableTimeMeasurementPrintsForMedrxiv.get_env_name()
            }
            Self::EnableTimeMeasurementPrintsForReddit => {
                EnvVar::EnableTimeMeasurementPrintsForReddit.get_env_name()
            }
            Self::EnableTimeMeasurementPrintsForTwitter => {
                EnvVar::EnableTimeMeasurementPrintsForTwitter.get_env_name()
            }

            Self::EnableInfoPrints => EnvVar::EnableInfoPrints.get_env_name(),
            Self::EnableInfoPrintsForProviders => {
                EnvVar::EnableInfoPrintsForProviders.get_env_name()
            }
            Self::EnableInfoPrintsForArxiv => EnvVar::EnableInfoPrintsForArxiv.get_env_name(),
            Self::EnableInfoPrintsForBiorxiv => EnvVar::EnableInfoPrintsForBiorxiv.get_env_name(),
            Self::EnableInfoPrintsForGithub => EnvVar::EnableInfoPrintsForGithub.get_env_name(),
            Self::EnableInfoPrintsForHabr => EnvVar::EnableInfoPrintsForHabr.get_env_name(),
            Self::EnableInfoPrintsForMedrxiv => EnvVar::EnableInfoPrintsForMedrxiv.get_env_name(),
            Self::EnableInfoPrintsForReddit => EnvVar::EnableInfoPrintsForReddit.get_env_name(),
            Self::EnableInfoPrintsForTwitter => EnvVar::EnableInfoPrintsForTwitter.get_env_name(),

            Self::EnableLinksLimit => EnvVar::EnableLinksLimit.get_env_name(),
            Self::EnableLinksLimitForProviders => {
                EnvVar::EnableLinksLimitForProviders.get_env_name()
            }
            Self::EnableLinksLimitForArxiv => EnvVar::EnableLinksLimitForArxiv.get_env_name(),
            Self::EnableLinksLimitForBiorxiv => EnvVar::EnableLinksLimitForBiorxiv.get_env_name(),
            Self::EnableLinksLimitForGithub => EnvVar::EnableLinksLimitForGithub.get_env_name(),
            Self::EnableLinksLimitForHabr => EnvVar::EnableLinksLimitForHabr.get_env_name(),
            Self::EnableLinksLimitForMedrxiv => EnvVar::EnableLinksLimitForMedrxiv.get_env_name(),
            Self::EnableLinksLimitForReddit => EnvVar::EnableLinksLimitForReddit.get_env_name(),
            Self::EnableLinksLimitForTwitter => EnvVar::EnableLinksLimitForTwitter.get_env_name(),

            Self::EnableCommonProvidersLinksLimit => {
                EnvVar::EnableCommonProvidersLinksLimit.get_env_name()
            }
        }
    }
}
