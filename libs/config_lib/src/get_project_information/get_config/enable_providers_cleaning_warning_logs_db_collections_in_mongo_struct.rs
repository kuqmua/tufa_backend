#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EnableProvidersCleaningWarningLogsDbCollectionsInMongo {
    pub enable_cleaning_warning_logs_db_collections_in_mongo_for_arxiv: bool,
    pub enable_cleaning_warning_logs_db_collections_in_mongo_for_biorxiv: bool,
    pub enable_cleaning_warning_logs_db_collections_in_mongo_for_github: bool,
    pub enable_cleaning_warning_logs_db_collections_in_mongo_for_habr: bool,
    pub enable_cleaning_warning_logs_db_collections_in_mongo_for_medrxiv: bool,
    pub enable_cleaning_warning_logs_db_collections_in_mongo_for_reddit: bool,
    pub enable_cleaning_warning_logs_db_collections_in_mongo_for_twitter: bool,
}
