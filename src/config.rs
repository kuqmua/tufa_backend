pub const ENABLE_ALL_PROVIDERS: bool = true;
pub const ENABLE_REDDIT: bool = ENABLE_ALL_PROVIDERS & false | false;
pub const ENABLE_ARXIV: bool = ENABLE_ALL_PROVIDERS & false | false;
pub const ENABLE_BIORXIV: bool = ENABLE_ALL_PROVIDERS & false | false;
pub const ENABLE_MEDRXIV: bool = ENABLE_ALL_PROVIDERS & false | false;
pub const ENABLE_TWITTER: bool = ENABLE_ALL_PROVIDERS & false | false;
pub const ENABLE_HABR: bool = ENABLE_ALL_PROVIDERS & true | false;

pub const STARTING_CHECK_LINK: &str = "https://www.google.com/";
pub const REDDIT_LINK: &str = "https://www.reddit.com/";
pub const ARXIV_LINK: &str = "https://www.google.com/"; //https://arxiv.org/ //"http://export.arxiv.org/rss/"
pub const BIORXIV_LINK: &str = "https://www.google.com/"; //http://connect.biorxiv.org/
pub const MEDRXIV_LINK: &str = "https://www.google.com/"; //http://connect.medrxiv.org/
pub const TWITTER_LINK: &str = "https://www.google.com/"; ////must be not only 1 str but many - twitter and many nitters
pub const HABR_LINK: &str = "https://www.google.com/"; ////must be not only 1 str but many - twitter and many nitters

pub const ENABLE_ALL_PROVIDERS_PRINTS: bool = true;
pub const ENABLE_PRINTS_REDDIT: bool = ENABLE_ALL_PROVIDERS_PRINTS & true | false;
pub const ENABLE_PRINTS_ARXIV: bool = ENABLE_ALL_PROVIDERS_PRINTS & true | false;
pub const ENABLE_PRINTS_BIORXIV: bool = ENABLE_ALL_PROVIDERS_PRINTS & true | false;
pub const ENABLE_PRINTS_MEDRXIV: bool = ENABLE_ALL_PROVIDERS_PRINTS & true | false;
pub const ENABLE_PRINTS_TWITTER: bool = ENABLE_ALL_PROVIDERS_PRINTS & true | false;
pub const ENABLE_PRINTS_HABR: bool = ENABLE_ALL_PROVIDERS_PRINTS & true | false;

pub const ENABLE_ALL_PROVIDERS_WARNING_PRINTS: bool = true;
pub const ENABLE_WARNING_PRINTS_REDDIT: bool = ENABLE_ALL_PROVIDERS_WARNING_PRINTS & true | false;
pub const ENABLE_WARNING_PRINTS_ARXIV: bool = ENABLE_ALL_PROVIDERS_WARNING_PRINTS & true | false;
pub const ENABLE_WARNING_PRINTS_BIORXIV: bool = ENABLE_ALL_PROVIDERS_WARNING_PRINTS & true | false;
pub const ENABLE_WARNING_PRINTS_MEDRXIV: bool = ENABLE_ALL_PROVIDERS_WARNING_PRINTS & true | false;
pub const ENABLE_WARNING_PRINTS_TWITTER: bool = ENABLE_ALL_PROVIDERS_WARNING_PRINTS & true | false;
pub const ENABLE_WARNING_PRINTS_HABR: bool = ENABLE_ALL_PROVIDERS_WARNING_PRINTS & true | false;

pub const ENABLE_ALL_PROVIDERS_ERROR_PRINTS: bool = true;
pub const ENABLE_ERROR_PRINTS_REDDIT: bool = ENABLE_ALL_PROVIDERS_ERROR_PRINTS & true | false;
pub const ENABLE_ERROR_PRINTS_ARXIV: bool = ENABLE_ALL_PROVIDERS_ERROR_PRINTS & true | false;
pub const ENABLE_ERROR_PRINTS_BIORXIV: bool = ENABLE_ALL_PROVIDERS_ERROR_PRINTS & true | false;
pub const ENABLE_ERROR_PRINTS_MEDRXIV: bool = ENABLE_ALL_PROVIDERS_ERROR_PRINTS & true | false;
pub const ENABLE_ERROR_PRINTS_TWITTER: bool = ENABLE_ALL_PROVIDERS_ERROR_PRINTS & true | false;
pub const ENABLE_ERROR_PRINTS_HABR: bool = ENABLE_ALL_PROVIDERS_ERROR_PRINTS & true | false;

pub const ENABLE_ALL_CLEANING_WARNING_LOGS_DIRECTORY: bool = true;
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_REDDIT: bool =
    ENABLE_ALL_CLEANING_WARNING_LOGS_DIRECTORY & true | false;
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_ARXIV: bool =
    ENABLE_ALL_CLEANING_WARNING_LOGS_DIRECTORY & true | false;
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_BIORXIV: bool =
    ENABLE_ALL_CLEANING_WARNING_LOGS_DIRECTORY & true | false;
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_MEDRXIV: bool =
    ENABLE_ALL_CLEANING_WARNING_LOGS_DIRECTORY & true | false;
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_TWITTER: bool =
    ENABLE_ALL_CLEANING_WARNING_LOGS_DIRECTORY & true | false;
pub const ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_HABR: bool =
    ENABLE_ALL_CLEANING_WARNING_LOGS_DIRECTORY & true | false;

pub const ENABLE_PRINTS_HANDLE: bool = true;
pub const ENABLE_ERROR_PRINTS_HANDLE: bool = true;

pub const WARNING_LOGS_DIRECTORY_NAME: &str = "warning_logs";
// pub const ERROR_LOGS_DIRECTORY_NAME: &str = "error_logs";

pub const ENABLE_ALL_TIME_MEASUREMENT: bool = true;
pub const ENABLE_COMMON_TIME_MEASUREMENT: bool = ENABLE_ALL_TIME_MEASUREMENT & true | false;
pub const ENABLE_REDDIT_TIME_MEASUREMENT: bool = ENABLE_ALL_TIME_MEASUREMENT & true | false;
pub const ENABLE_ARXIV_TIME_MEASUREMENT: bool = ENABLE_ALL_TIME_MEASUREMENT & true | false;
pub const ENABLE_BIORXIV_TIME_MEASUREMENT: bool = ENABLE_ALL_TIME_MEASUREMENT & true | false;
pub const ENABLE_MEDRXIV_TIME_MEASUREMENT: bool = ENABLE_ALL_TIME_MEASUREMENT & true | false;
pub const ENABLE_TWITTER_TIME_MEASUREMENT: bool = ENABLE_ALL_TIME_MEASUREMENT & true | false;
pub const ENABLE_HABR_TIME_MEASUREMENT: bool = ENABLE_ALL_TIME_MEASUREMENT & true | false;
