pub const LOAD_CONFIG_FILE_ERROR_MESSAGE: &str = "cannot create config";

// pub const LOGS_COMMON_FOLDER_NAME: &str = "common_folder";

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
pub const BIORXIV_LINK_FIRST_PART: &str = "http://connect.biorxiv.org/biorxiv_xml.php?subject=";
pub const GITHUB_LINK_FIRST_PART: &str = "https://github.com/";
pub const GITHUB_LINK_SECOND_PART: &str = ".private.atom?token=";
pub const HABR_LINK_FIRST_PART: &str = "https://habr.com/ru/rss/";
pub const MEDRXIV_LINK_FIRST_PART: &str = "http://connect.medrxiv.org/medrxiv_xml.php?subject=";
pub const REDDIT_LINK_FIRST_PART: &str = "https://www.reddit.com/r/";
pub const REDDIT_LINK_SECOND_PART: &str = "/new.json";
pub const TWITTER_LINK_FIRST_PART: &str = "https://";
pub const TWITTER_LINK_SECOND_PART: &str = "/";
pub const TWITTER_LINK_THIRD_PART: &str = "/rss";

//tests constants start
pub const ENV_FILE_NAME: &str = ".env";
pub const PATH_TO_ENV_FILE: &str = "./";
pub const PATH_TO_DOCKER_COMPOSE_FILE: &str = "./";
pub const DOCKER_COMPOSE_FILE_NAME: &str = "docker-compose.yml";
//tests constants end
