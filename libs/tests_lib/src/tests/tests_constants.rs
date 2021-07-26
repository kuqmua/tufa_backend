pub const CONFIG_FILE_EXTENSION: &str = ".toml";
pub static VECTOR_OF_MODES: &[&str] = &["Development", "Production", "Testing"];

//its important to have EXACT copy without spaces or Line feed character
pub const CONFIG_CONTENT: &str = r#"[params]
vec_of_provider_names = ["arxiv", "biorxiv", "github", "habr", "medrxiv", "reddit", "twitter"]
#it can be only one of them: "arxiv", "biorxiv", "github", "habr", "medrxiv", "reddit", "twitter" - check project_constants.rs
user_credentials_dummy_handle = "example"
warning_logs_directory_name = "warning_logs"
unhandled_success_handled_success_are_there_items_initialized_posts_dir = "unhandled_success_handled_success_are_there_items_initialized_posts"
# // pub const ERROR_LOGS_DIRECTORY_NAME: &str = "error_logs";
enable_providers = true
enable_cleaning_warning_logs_directory = true
enable_time_measurement = true
enable_provider_links_limit = true
enable_common_providers_links_limit = true
common_providers_links_limit = 10 # i64 
enable_prints = true
enable_error_prints = true
enable_warning_high_prints = true
enable_warning_low_prints = true
enable_success_prints = true
enable_partial_success_prints = true
enable_time_measurement_prints = true
enable_cleaning_warning_logs_directory_prints = true
enable_all_providers_prints = true
enable_error_prints_for_all_providers = true
enable_warning_high_prints_for_all_providers = true
enable_warning_low_prints_for_all_providers = true
enable_success_prints_for_all_providers = true
enable_partial_success_prints_for_all_providers = true
enable_time_measurement_prints_for_all_providers = true
enable_cleaning_warning_logs_directory_prints_for_all_providers = true

[mongo_params]
is_cloud = true
db_name_handle = "provider_links"
db_collection_handle_second_part = "_link_parts"
db_collection_document_field_name_handle = "link_part"
path_to_provider_link_parts_folder = "./providers_link_parts/"
file_extension = ".json"
mongo_own_first_handle_url_part = "mongodb://"
mongo_own_second_handle_url_part = ":"
mongo_own_third_handle_url_part = "@"
mongo_own_fourth_handle_url_part = ":"
mongo_cloud_first_handle_url_part = "mongodb+srv://"
mongo_cloud_second_handle_url_part = ":"
mongo_cloud_third_handle_url_part = "@"
mongo_cloud_fourth_handle_url_part = "/"

[enable_providers]
enable_arxiv = true
enable_biorxiv = true
enable_github = true
enable_habr = true
enable_medrxiv = true
enable_reddit = true
enable_twitter = true

[links]
starting_check_link = "https://www.google.com/"
arxiv_link = "https://www.google.com/" # https://arxiv.org/   http://export.arxiv.org/rss/   http://export.arxiv.org/rss/astro-ph.CO
biorxiv_link = "https://www.google.com/" # http://connect.biorxiv.org/
github_link = "https://github.com"
habr_link = "https://www.google.com/" # https://habr.com/ru/
medrxiv_link = "https://www.google.com/" # http://connect.medrxiv.org/
reddit_link = "https://www.reddit.com/"
twitter_link = "https://www.google.com/" # must be not only 1 str but many - twitter and many nitters

[enable_providers_prints]
enable_prints_arxiv = true
enable_prints_biorxiv = true
enable_prints_github = true
enable_prints_habr = true
enable_prints_medrxiv = true
enable_prints_reddit = true
enable_prints_twitter = true

[enable_warning_high_providers_prints]
enable_warning_high_prints_for_arxiv = true
enable_warning_high_prints_for_biorxiv = true
enable_warning_high_prints_for_github = true
enable_warning_high_prints_for_habr = true
enable_warning_high_prints_for_medrxiv = true
enable_warning_high_prints_for_reddit = true
enable_warning_high_prints_for_twitter = true

[enable_warning_low_providers_prints]
enable_warning_low_prints_for_arxiv = true
enable_warning_low_prints_for_biorxiv = true
enable_warning_low_prints_for_github = true
enable_warning_low_prints_for_habr = true
enable_warning_low_prints_for_medrxiv = true
enable_warning_low_prints_for_reddit = true
enable_warning_low_prints_for_twitter = true

[enable_error_providers_prints]
enable_error_prints_for_arxiv = true
enable_error_prints_for_biorxiv = true
enable_error_prints_for_github = true
enable_error_prints_for_habr = true
enable_error_prints_for_medrxiv = true
enable_error_prints_for_reddit = true
enable_error_prints_for_twitter = true

[enable_success_providers_prints]
enable_success_prints_for_arxiv = true
enable_success_prints_for_biorxiv = true
enable_success_prints_for_github = true
enable_success_prints_for_habr = true
enable_success_prints_for_medrxiv = true
enable_success_prints_for_reddit = true
enable_success_prints_for_twitter = true

[enable_partial_success_providers_prints]
enable_partial_success_prints_for_arxiv = true
enable_partial_success_prints_for_biorxiv = true
enable_partial_success_prints_for_github = true
enable_partial_success_prints_for_habr = true
enable_partial_success_prints_for_medrxiv = true
enable_partial_success_prints_for_reddit = true
enable_partial_success_prints_for_twitter = true

[enable_providers_cleaning_warning_logs_directory]
enable_cleaning_warning_logs_directory_for_arxiv = true
enable_cleaning_warning_logs_directory_for_biorxiv = true
enable_cleaning_warning_logs_directory_for_github = true
enable_cleaning_warning_logs_directory_for_habr = true
enable_cleaning_warning_logs_directory_for_medrxiv = true
enable_cleaning_warning_logs_directory_for_reddit = true
enable_cleaning_warning_logs_directory_for_twitter = true

[enable_providers_time_measurement]
enable_time_measurement_for_arxiv = true
enable_time_measurement_for_biorxiv = true
enable_time_measurement_for_github = true
enable_time_measurement_for_habr = true
enable_time_measurement_for_medrxiv = true
enable_time_measurement_for_reddit = true
enable_time_measurement_for_twitter = true

[enable_providers_links_limits]
enable_links_limit_for_arxiv = true
enable_links_limit_for_biorxiv = true
enable_links_limit_for_github = true
enable_links_limit_for_habr = true
enable_links_limit_for_medrxiv = true
enable_links_limit_for_reddit = true
enable_links_limit_for_twitter = true

[providers_links_limits]
links_limit_for_arxiv = 10 # i64
links_limit_for_biorxiv = 10 # i64
links_limit_for_github = 10 # i64
links_limit_for_habr = 10 # i64
links_limit_for_medrxiv = 10 # i64
links_limit_for_reddit = 10 # i64
links_limit_for_twitter = 10 # i64

[print_colors]
error_red = 255 # u8
error_green = 0 # u8
error_blue = 0 # u8
warning_high_red = 255 # u8
warning_high_green = 165 # u8
warning_high_blue = 0 # u8
warning_low_red = 255 # u8
warning_low_green = 255 # u8
warning_low_blue = 0 # u8
success_red = 0 # u8
success_green = 255 # u8
success_blue = 0 # u8
partial_success_red = 0 # u8
partial_success_green = 200 # u8
partial_success_blue = 155 # u8
cleaning_red = 230 # u8
cleaning_green = 234 # u8
cleaning_blue = 255 # u8
time_measurement_red = 255 # u8
time_measurement_green = 153 # u8
time_measurement_blue = 170 # u8"#;
//its important to have EXACT copy without spaces or Line feed character
pub const USER_CREDENTIALS_CONTENT: &str = r#"[github_authorization]
github_name = "example"
github_token = "example"

[reddit_authorization]
reddit_user_agent = "example"
reddit_client_id = "example"
reddit_client_secret = "example"
reddit_username = "example"
reddit_password = "example"

[mongo_own_authorization]
mongo_own_login = "example"
mongo_own_password = "example"
mongo_own_ip = "example"
mongo_own_port = "example"

[mongo_cloud_authorization]
mongo_cloud_login = "example"
mongo_cloud_password = "example"
mongo_cloud_cluster_name = "example"
mongo_cloud_cluster_params = "example""#;
