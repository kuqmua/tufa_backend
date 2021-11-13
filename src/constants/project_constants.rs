pub const ENV_FILE_NAME: &str = ".env";

pub const LOAD_CONFIG_FILE_ERROR_MESSAGE: &str = "cannot create config";

// pub const LOGS_COMMON_FOLDER_NAME: &str = "common_folder";

pub const ARXIV_NAME_TO_CHECK: &str = "arxiv";
pub const BIORXIV_NAME_TO_CHECK: &str = "biorxiv";
pub const GITHUB_NAME_TO_CHECK: &str = "github";
pub const HABR_NAME_TO_CHECK: &str = "habr";
pub const MEDRXIV_NAME_TO_CHECK: &str = "medrxiv";
pub const REDDIT_NAME_TO_CHECK: &str = "reddit";
pub const TWITTER_NAME_TO_CHECK: &str = "twitter";

pub const COMMON_PROVIDER_ITEM_HANDLE: &str = "</item>";
pub const GITHUB_PROVIDER_ITEM_HANDLE: &str = "</entry>";

pub const TWITTER_FILTER_HANDLE_TO_REMOVE_1: &str = "<dc:creator>";
pub const TWITTER_FILTER_HANDLE_TO_REPLACE_REMOVED_1: &str = "bbb<creator>";
pub const TWITTER_FILTER_HANDLE_TO_REMOVE_2: &str = "</dc:creator>";
pub const TWITTER_FILTER_HANDLE_TO_REPLACE_REMOVED_2: &str = "bbb</creator>";
pub const TWITTER_FILTER_HANDLE_TO_REMOVE_3: &str = "<atom:link";
pub const TWITTER_FILTER_HANDLE_TO_REPLACE_REMOVED_3: &str = "<atomllink";

pub const MEDRXIV_FILTER_HANDLE_TO_REMOVE_1: &str = "<dc:title>";
pub const MEDRXIV_FILTER_HANDLE_TO_REPLACE_REMOVED_1: &str = "<dccfifle>";
pub const MEDRXIV_FILTER_HANDLE_TO_REMOVE_2: &str = "</dc:title>";
pub const MEDRXIV_FILTER_HANDLE_TO_REPLACE_REMOVED_2: &str = "</dccfifle>";

pub const BIORXIV_FILTER_HANDLE_TO_REMOVE_1: &str = "<dc:title>";
pub const BIORXIV_FILTER_HANDLE_TO_REPLACE_REMOVED_1: &str = "<dcstitle>";
pub const BIORXIV_FILTER_HANDLE_TO_REMOVE_2: &str = "</dc:title>";
pub const BIORXIV_FILTER_HANDLE_TO_REPLACE_REMOVED_2: &str = "</dcstitle>";

pub const HABR_FILTER_HANDLE_TO_REMOVE_1: &str = "<channel>";
pub const HABR_FILTER_HANDLE_TO_REPLACE_REMOVED_1: &str = "         ";
pub const HABR_FILTER_HANDLE_TO_REMOVE_2: &str = "</channel>";
pub const HABR_FILTER_HANDLE_TO_REPLACE_REMOVED_2: &str = "         ";

pub const ARXIV_LINK_FIRST_PART: &str = "http://export.arxiv.org/rss/";
