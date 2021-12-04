use std::collections::HashMap;

use strum::IntoEnumIterator;

use crate::config_mods::config_error_mods::config_env_var_error_type_enum::ConfigEnvVarErrorType;
use crate::config_mods::config_error_mods::config_error::ConfigError;
use crate::config_mods::config_error_mods::config_error_inner_type_enum::ConfigErrorInnerType;
use crate::config_mods::config_values_types_enums::env_var_bool_enum::EnvBoolVar;
use crate::config_mods::env_var_enum::EnvVar;

use crate::traits::env_var_trait::EnvVarTrait;

impl EnvVarTrait for EnvBoolVar {
    fn get_env_name(&self) -> &'static str {
        match self {
            Self::EnableProviders => EnvVar::EnableProviders.get_env_name(),
            Self::EnableCleaningWarningLogsDirectory => {
                EnvVar::EnableCleaningWarningLogsDirectory.get_env_name()
            }
            Self::EnableCleaningWarningLogsDbInMongo => {
                EnvVar::EnableCleaningWarningLogsDbInMongo.get_env_name()
            }
            Self::EnableCleaningWarningLogsDbCollectionsInMongo => {
                EnvVar::EnableCleaningWarningLogsDbCollectionsInMongo.get_env_name()
            }
            Self::EnableTimeMeasurement => EnvVar::EnableTimeMeasurement.get_env_name(),
            Self::EnableProviderLinksLimit => EnvVar::EnableProviderLinksLimit.get_env_name(),
            Self::EnableCommonProvidersLinksLimit => {
                EnvVar::EnableCommonProvidersLinksLimit.get_env_name()
            }
            Self::EnableRandomizeOrderForProvidersLinkPartsForMongo => {
                EnvVar::EnableRandomizeOrderForProvidersLinkPartsForMongo.get_env_name()
            }
            Self::EnablePrints => EnvVar::EnablePrints.get_env_name(),
            Self::EnableErrorPrints => EnvVar::EnableErrorPrints.get_env_name(),
            Self::EnableWarningHighPrints => EnvVar::EnableWarningHighPrints.get_env_name(),
            Self::EnableWarningLowPrints => EnvVar::EnableWarningLowPrints.get_env_name(),
            Self::EnableSuccessPrints => EnvVar::EnableSuccessPrints.get_env_name(),
            Self::EnablePartialSuccessPrints => EnvVar::EnablePartialSuccessPrints.get_env_name(),
            Self::EnableTimeMeasurementPrints => EnvVar::EnableTimeMeasurementPrints.get_env_name(),
            Self::EnableCleaningWarningLogsDirectoryPrints => {
                EnvVar::EnableCleaningWarningLogsDirectoryPrints.get_env_name()
            }
            Self::EnableInfoPrints => EnvVar::EnableInfoPrints.get_env_name(),
            Self::EnableAllProvidersPrints => EnvVar::EnableAllProvidersPrints.get_env_name(),
            Self::EnableErrorPrintsForAllProviders => {
                EnvVar::EnableErrorPrintsForAllProviders.get_env_name()
            }
            Self::EnableWarningHighPrintsForAllProviders => {
                EnvVar::EnableWarningHighPrintsForAllProviders.get_env_name()
            }
            Self::EnableWarningLowPrintsForAllProviders => {
                EnvVar::EnableWarningLowPrintsForAllProviders.get_env_name()
            }
            Self::EnableSuccessPrintsForAllProviders => {
                EnvVar::EnableSuccessPrintsForAllProviders.get_env_name()
            }
            Self::EnablePartialSuccessPrintsForAllProviders => {
                EnvVar::EnablePartialSuccessPrintsForAllProviders.get_env_name()
            }
            Self::EnableTimeMeasurementPrintsForAllProviders => {
                EnvVar::EnableTimeMeasurementPrintsForAllProviders.get_env_name()
            }
            Self::EnableCleaningWarningLogsDirectoryPrintsForAllProviders => {
                EnvVar::EnableCleaningWarningLogsDirectoryPrintsForAllProviders.get_env_name()
            }
            Self::EnableInfoPrintsForAllProviders => {
                EnvVar::EnableInfoPrintsForAllProviders.get_env_name()
            }
            Self::EnableWriteErrorLogsInLocalFolder => {
                EnvVar::EnableWriteErrorLogsInLocalFolder.get_env_name()
            }
            Self::EnableWriteErrorLogsInMongo => EnvVar::EnableWriteErrorLogsInMongo.get_env_name(),
            Self::EnableInitializeMongoWithProvidersLinkParts => {
                EnvVar::EnableInitializeMongoWithProvidersLinkParts.get_env_name()
            }

            Self::EnableInitializeMongoWithArxivLinkParts => {
                EnvVar::EnableInitializeMongoWithArxivLinkParts.get_env_name()
            }
            Self::EnableInitializeMongoWithBiorxivLinkParts => {
                EnvVar::EnableInitializeMongoWithBiorxivLinkParts.get_env_name()
            }
            Self::EnableInitializeMongoWithGithubLinkParts => {
                EnvVar::EnableInitializeMongoWithGithubLinkParts.get_env_name()
            }
            Self::EnableInitializeMongoWithHabrLinkParts => {
                EnvVar::EnableInitializeMongoWithHabrLinkParts.get_env_name()
            }
            Self::EnableInitializeMongoWithMedrxivLinkParts => {
                EnvVar::EnableInitializeMongoWithMedrxivLinkParts.get_env_name()
            }
            Self::EnableInitializeMongoWithRedditLinkParts => {
                EnvVar::EnableInitializeMongoWithRedditLinkParts.get_env_name()
            }
            Self::EnableInitializeMongoWithTwitterLinkParts => {
                EnvVar::EnableInitializeMongoWithTwitterLinkParts.get_env_name()
            }

            Self::EnableArxiv => EnvVar::EnableArxiv.get_env_name(),
            Self::EnableBiorxiv => EnvVar::EnableBiorxiv.get_env_name(),
            Self::EnableGithub => EnvVar::EnableGithub.get_env_name(),
            Self::EnableHabr => EnvVar::EnableHabr.get_env_name(),
            Self::EnableMedrxiv => EnvVar::EnableMedrxiv.get_env_name(),
            Self::EnableReddit => EnvVar::EnableReddit.get_env_name(),
            Self::EnableTwitter => EnvVar::EnableTwitter.get_env_name(),

            Self::EnablePrintsArxiv => EnvVar::EnablePrintsArxiv.get_env_name(),
            Self::EnablePrintsBiorxiv => EnvVar::EnablePrintsBiorxiv.get_env_name(),
            Self::EnablePrintsGithub => EnvVar::EnablePrintsGithub.get_env_name(),
            Self::EnablePrintsHabr => EnvVar::EnablePrintsHabr.get_env_name(),
            Self::EnablePrintsMedrxiv => EnvVar::EnablePrintsMedrxiv.get_env_name(),
            Self::EnablePrintsReddit => EnvVar::EnablePrintsReddit.get_env_name(),
            Self::EnablePrintsTwitter => EnvVar::EnablePrintsTwitter.get_env_name(),

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

            Self::EnableErrorPrintsForArxiv => EnvVar::EnableErrorPrintsForArxiv.get_env_name(),
            Self::EnableErrorPrintsForBiorxiv => EnvVar::EnableErrorPrintsForBiorxiv.get_env_name(),
            Self::EnableErrorPrintsForGithub => EnvVar::EnableErrorPrintsForGithub.get_env_name(),
            Self::EnableErrorPrintsForHabr => EnvVar::EnableErrorPrintsForHabr.get_env_name(),
            Self::EnableErrorPrintsForMedrxiv => EnvVar::EnableErrorPrintsForMedrxiv.get_env_name(),
            Self::EnableErrorPrintsForReddit => EnvVar::EnableErrorPrintsForReddit.get_env_name(),
            Self::EnableErrorPrintsForTwitter => EnvVar::EnableErrorPrintsForTwitter.get_env_name(),

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

            Self::EnableCleaningWarningLogsDbInMongoForArxiv => {
                EnvVar::EnableCleaningWarningLogsDbInMongoForArxiv.get_env_name()
            }
            Self::EnableCleaningWarningLogsDbInMongoForBiorxiv => {
                EnvVar::EnableCleaningWarningLogsDbInMongoForBiorxiv.get_env_name()
            }
            Self::EnableCleaningWarningLogsDbInMongoForGithub => {
                EnvVar::EnableCleaningWarningLogsDbInMongoForGithub.get_env_name()
            }
            Self::EnableCleaningWarningLogsDbInMongoForHabr => {
                EnvVar::EnableCleaningWarningLogsDbInMongoForHabr.get_env_name()
            }
            Self::EnableCleaningWarningLogsDbInMongoForMedrxiv => {
                EnvVar::EnableCleaningWarningLogsDbInMongoForMedrxiv.get_env_name()
            }
            Self::EnableCleaningWarningLogsDbInMongoForReddit => {
                EnvVar::EnableCleaningWarningLogsDbInMongoForReddit.get_env_name()
            }
            Self::EnableCleaningWarningLogsDbInMongoForTwitter => {
                EnvVar::EnableCleaningWarningLogsDbInMongoForTwitter.get_env_name()
            }

            Self::EnableCleaningWarningLogsDbCollectionsInMongoForArxiv => {
                EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForArxiv.get_env_name()
            }
            Self::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxiv => {
                EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForBiorxiv.get_env_name()
            }
            Self::EnableCleaningWarningLogsDbCollectionsInMongoForGithub => {
                EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForGithub.get_env_name()
            }
            Self::EnableCleaningWarningLogsDbCollectionsInMongoForHabr => {
                EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForHabr.get_env_name()
            }
            Self::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxiv => {
                EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForMedrxiv.get_env_name()
            }
            Self::EnableCleaningWarningLogsDbCollectionsInMongoForReddit => {
                EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForReddit.get_env_name()
            }
            Self::EnableCleaningWarningLogsDbCollectionsInMongoForTwitter => {
                EnvVar::EnableCleaningWarningLogsDbCollectionsInMongoForTwitter.get_env_name()
            }

            Self::EnableTimeMeasurementForArxiv => {
                EnvVar::EnableTimeMeasurementForArxiv.get_env_name()
            }
            Self::EnableTimeMeasurementForBiorxiv => {
                EnvVar::EnableTimeMeasurementForBiorxiv.get_env_name()
            }
            Self::EnableTimeMeasurementForGithub => {
                EnvVar::EnableTimeMeasurementForGithub.get_env_name()
            }
            Self::EnableTimeMeasurementForHabr => {
                EnvVar::EnableTimeMeasurementForHabr.get_env_name()
            }
            Self::EnableTimeMeasurementForMedrxiv => {
                EnvVar::EnableTimeMeasurementForMedrxiv.get_env_name()
            }
            Self::EnableTimeMeasurementForReddit => {
                EnvVar::EnableTimeMeasurementForReddit.get_env_name()
            }
            Self::EnableTimeMeasurementForTwitter => {
                EnvVar::EnableTimeMeasurementForTwitter.get_env_name()
            }

            Self::EnableInfoForArxiv => EnvVar::EnableInfoForArxiv.get_env_name(),
            Self::EnableInfoForBiorxiv => EnvVar::EnableInfoForBiorxiv.get_env_name(),
            Self::EnableInfoForGithub => EnvVar::EnableInfoForGithub.get_env_name(),
            Self::EnableInfoForHabr => EnvVar::EnableInfoForHabr.get_env_name(),
            Self::EnableInfoForMedrxiv => EnvVar::EnableInfoForMedrxiv.get_env_name(),
            Self::EnableInfoForReddit => EnvVar::EnableInfoForReddit.get_env_name(),
            Self::EnableInfoForTwitter => EnvVar::EnableInfoForTwitter.get_env_name(),

            Self::EnableLinksLimitForArxiv => EnvVar::EnableLinksLimitForArxiv.get_env_name(),
            Self::EnableLinksLimitForBiorxiv => EnvVar::EnableLinksLimitForBiorxiv.get_env_name(),
            Self::EnableLinksLimitForGithub => EnvVar::EnableLinksLimitForGithub.get_env_name(),
            Self::EnableLinksLimitForHabr => EnvVar::EnableLinksLimitForHabr.get_env_name(),
            Self::EnableLinksLimitForMedrxiv => EnvVar::EnableLinksLimitForMedrxiv.get_env_name(),
            Self::EnableLinksLimitForReddit => EnvVar::EnableLinksLimitForReddit.get_env_name(),
            Self::EnableLinksLimitForTwitter => EnvVar::EnableLinksLimitForTwitter.get_env_name(),

            Self::EnableRandomizeOrderForArxivLinkPartsForMongo => {
                EnvVar::EnableRandomizeOrderForArxivLinkPartsForMongo.get_env_name()
            }
            Self::EnableRandomizeOrderForBiorxivLinkPartsForMongo => {
                EnvVar::EnableRandomizeOrderForBiorxivLinkPartsForMongo.get_env_name()
            }
            Self::EnableRandomizeOrderForGithubLinkPartsForMongo => {
                EnvVar::EnableRandomizeOrderForGithubLinkPartsForMongo.get_env_name()
            }
            Self::EnableRandomizeOrderForHabrLinkPartsForMongo => {
                EnvVar::EnableRandomizeOrderForHabrLinkPartsForMongo.get_env_name()
            }
            Self::EnableRandomizeOrderForMedrxivLinkPartsForMongo => {
                EnvVar::EnableRandomizeOrderForMedrxivLinkPartsForMongo.get_env_name()
            }
            Self::EnableRandomizeOrderForRedditLinkPartsForMongo => {
                EnvVar::EnableRandomizeOrderForRedditLinkPartsForMongo.get_env_name()
            }
            Self::EnableRandomizeOrderForTwitterLinkPartsForMongo => {
                EnvVar::EnableRandomizeOrderForTwitterLinkPartsForMongo.get_env_name()
            }
        }
    }
    fn into_array() -> &'static [Self] {
        Self::all_variants()
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_string_name_and_kind_hashmap() -> HashMap<&'static str, Self> {
        let mut config_env_var_name_kind_string_to_enum_struct_hasmap: HashMap<&'static str, Self> =
            HashMap::with_capacity(Self::get_length());
        for env_var_name_kind_kind in Self::iter() {
            config_env_var_name_kind_string_to_enum_struct_hasmap.insert(
                env_var_name_kind_kind.get_env_name(),
                env_var_name_kind_kind,
            );
        }
        config_env_var_name_kind_string_to_enum_struct_hasmap
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_string_name_and_kind_tuple_vec() -> Vec<(&'static str, Self)> {
        let mut env_var_name_kind_vec = Vec::with_capacity(Self::get_length());
        for env_var_name_kind in Self::iter() {
            env_var_name_kind_vec.push((env_var_name_kind.get_env_name(), env_var_name_kind));
        }
        env_var_name_kind_vec
    }
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    fn into_vec() -> Vec<Self> {
        let mut env_var_name_kind_vec = Vec::with_capacity(Self::get_length());
        for env_var_name_kind in Self::iter() {
            env_var_name_kind_vec.push(env_var_name_kind);
        }
        env_var_name_kind_vec
    }
    fn get_string_from_env_var(
        &self,
        was_dotenv_enable: bool,
    ) -> Result<String, ConfigError<'static>> {
        let string_name = self.get_env_name();
        match std::env::var(string_name) {
            Ok(handle) => Ok(handle),
            Err(e) => Err(ConfigError {
                env_var_name_kind: ConfigEnvVarErrorType::Bool(*self),
                was_dotenv_enable,
                env_name: string_name,
                env_error: ConfigErrorInnerType::VarErrorHandle(e),
            }),
        }
    }
}
