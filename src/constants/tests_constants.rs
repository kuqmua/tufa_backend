pub const _PATH_TO_ENV_FILE: &str = "./";
pub const _DOCKER_COMPOSE_FILE_NAME: &str = "docker-compose.yml";
pub const _USER_CREDENTIALS_DUMMY_HANDLE: &str = "example";
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//todo
//its important to have EXACT copy without spaces or Line feed character
pub const _DOCKER_COMPOSE_CONTENT: &str = r#"version: '3.8'
services:
  postgres_service:
    container_name: postgres_container
    image: postgres:latest
    restart: always
    environment:
      POSTGRES_USER: ${POSTGRES_LOCAL_DOCKER_USERNAME}
      POSTGRES_PASSWORD: ${POSTGRES_LOCAL_DOCKER_PASSWORD}
      POSTGRES_DB: postgres
    volumes:
      - ./postgresql_volume:/data/db
    ports:
      - 5432:5432
  mongodb_service:
    container_name: mongodb_container
    image: mongo:latest
    restart: always
    environment:
      ME_CONFIG_MONGODB_ADMINUSERNAME: ${MONGO_LOCAL_DOCKER_USERNAME}
      ME_CONFIG_MONGODB_ADMINPASSWORD: ${MONGO_LOCAL_DOCKER_PASSWORD}
    ports:
      - 27017:27017
    volumes:
      - ./mongodb_volume:/data/db
  tufa_backend_service:
    container_name: tufa_backend-container
    image: tufa_backend-image
    restart: always
    ports:
      - 8000:8000
    env_file:
      - ./.env
    environment: 
      GITHUB_NAME: ${GITHUB_NAME}
      GITHUB_TOKEN: ${GITHUB_TOKEN}
      REDDIT_USER_AGENT: ${REDDIT_USER_AGENT}
      REDDIT_CLIENT_ID: ${REDDIT_CLIENT_ID}
      REDDIT_CLIENT_SECRET: ${REDDIT_CLIENT_SECRET}
      REDDIT_USERNAME: ${REDDIT_USERNAME}
      REDDIT_PASSWORD: ${REDDIT_PASSWORD}
      STARTING_CHECK_LINK: ${STARTING_CHECK_LINK}
      WARNING_LOGS_DIRECTORY_NAME: ${WARNING_LOGS_DIRECTORY_NAME}
      UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR: ${UNHANDLED_SUCCESS_HANDLED_SUCCESS_ARE_THERE_ITEMS_INITIALIZED_POSTS_DIR}
      ENABLE_PROVIDERS: ${ENABLE_PROVIDERS}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY}
      ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO: ${ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO}
      ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO: ${ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO}
      ENABLE_TIME_MEASUREMENT: ${ENABLE_TIME_MEASUREMENT}
      ENABLE_PROVIDER_LINKS_LIMIT: ${ENABLE_PROVIDER_LINKS_LIMIT}
      ENABLE_COMMON_PROVIDERS_LINKS_LIMIT: ${ENABLE_COMMON_PROVIDERS_LINKS_LIMIT}
      COMMON_PROVIDERS_LINKS_LIMIT: ${COMMON_PROVIDERS_LINKS_LIMIT} # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
      ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO: ${ENABLE_RANDOMIZE_ORDER_FOR_PROVIDERS_LINK_PARTS_FOR_MONGO}
      ENABLE_PRINTS: ${ENABLE_PRINTS}
      ENABLE_ERROR_PRINTS: ${ENABLE_ERROR_PRINTS}
      ENABLE_WARNING_HIGH_PRINTS: ${ENABLE_WARNING_HIGH_PRINTS}
      ENABLE_WARNING_LOW_PRINTS: ${ENABLE_WARNING_LOW_PRINTS}
      ENABLE_SUCCESS_PRINTS: ${ENABLE_SUCCESS_PRINTS}
      ENABLE_PARTIAL_SUCCESS_PRINTS: ${ENABLE_PARTIAL_SUCCESS_PRINTS}
      ENABLE_TIME_MEASUREMENT_PRINTS: ${ENABLE_TIME_MEASUREMENT_PRINTS}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS}
      ENABLE_INFO_PRINTS: ${ENABLE_INFO_PRINTS}
      ENABLE_ALL_PROVIDERS_PRINTS: ${ENABLE_ALL_PROVIDERS_PRINTS}
      ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS: ${ENABLE_ERROR_PRINTS_FOR_ALL_PROVIDERS}
      ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS: ${ENABLE_WARNING_HIGH_PRINTS_FOR_ALL_PROVIDERS}
      ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS: ${ENABLE_WARNING_LOW_PRINTS_FOR_ALL_PROVIDERS}
      ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS: ${ENABLE_SUCCESS_PRINTS_FOR_ALL_PROVIDERS}
      ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS: ${ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ALL_PROVIDERS}
      ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS: ${ENABLE_TIME_MEASUREMENT_PRINTS_FOR_ALL_PROVIDERS}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_PRINTS_FOR_ALL_PROVIDERS}
      ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS: ${ENABLE_INFO_PRINTS_FOR_ALL_PROVIDERS}
      ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER: ${ENABLE_WRITE_ERROR_LOGS_IN_LOCAL_FOLDER}
      ENABLE_WRITE_ERROR_LOGS_IN_MONGO: ${ENABLE_WRITE_ERROR_LOGS_IN_MONGO}
      ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS: ${ENABLE_INITIALIZE_MONGO_WITH_PROVIDERS_LINK_PARTS}
      
      PROVIDERS_DB_NAME_HANDLE: ${PROVIDERS_DB_NAME_HANDLE}
      PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART: ${PROVIDERS_DB_COLLECTION_HANDLE_SECOND_PART}
      PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE: ${PROVIDERS_DB_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE}
      PATH_TO_PROVIDER_LINK_PARTS_FOLDER: ${PATH_TO_PROVIDER_LINK_PARTS_FOLDER}
      LOG_FILE_EXTENSION: ${LOG_FILE_EXTENSION}
      DB_PROVIDERS_LOGS_NAME_HANDLE: ${DB_PROVIDERS_LOGS_NAME_HANDLE}
      DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART: ${DB_PROVIDERS_LOGS_COLLECTION_HANDLE_SECOND_PART}
      DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE: ${DB_PROVIDERS_LOGS_COLLECTION_DOCUMENT_FIELD_NAME_HANDLE}
    
      ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS: ${ENABLE_INITIALIZE_MONGO_WITH_ARXIV_LINK_PARTS}
      ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS: ${ENABLE_INITIALIZE_MONGO_WITH_BIORXIV_LINK_PARTS}
      ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS: ${ENABLE_INITIALIZE_MONGO_WITH_GITHUB_LINK_PARTS}
      ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS: ${ENABLE_INITIALIZE_MONGO_WITH_HABR_LINK_PARTS}
      ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS: ${ENABLE_INITIALIZE_MONGO_WITH_MEDRXIV_LINK_PARTS}
      ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS: ${ENABLE_INITIALIZE_MONGO_WITH_REDDIT_LINK_PARTS}
      ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS: ${ENABLE_INITIALIZE_MONGO_WITH_TWITTER_LINK_PARTS}

      MONGO_FIRST_HANDLE_URL_PART: ${MONGO_FIRST_HANDLE_URL_PART}
      MONGO_SECOND_HANDLE_URL_PART: ${MONGO_SECOND_HANDLE_URL_PART}
      MONGO_THIRD_HANDLE_URL_PART: ${MONGO_THIRD_HANDLE_URL_PART}
      MONGO_FOURTH_HANDLE_URL_PART: ${MONGO_FOURTH_HANDLE_URL_PART}
      MONGO_FIFTH_HANDLE_URL_PART: ${MONGO_FIFTH_HANDLE_URL_PART}

      MONGO_LOGIN: ${MONGO_LOGIN}
      MONGO_PASSWORD: ${MONGO_PASSWORD}
      MONGO_IP: ${MONGO_IP}
      MONGO_PORT: ${MONGO_PORT}
      MONGO_PARAMS: ${MONGO_PARAMS}
 
      POSTGRES_FIRST_HANDLE_URL_PART: ${POSTGRES_FIRST_HANDLE_URL_PART}
      POSTGRES_SECOND_HANDLE_URL_PART: ${POSTGRES_SECOND_HANDLE_URL_PART}
      POSTGRES_THIRD_HANDLE_URL_PART: ${POSTGRES_THIRD_HANDLE_URL_PART}
      POSTGRES_FOURTH_HANDLE_URL_PART: ${POSTGRES_FOURTH_HANDLE_URL_PART}
      POSTGRES_FIFTH_HANDLE_URL_PART: ${POSTGRES_FIFTH_HANDLE_URL_PART}

      POSTGRES_LOGIN: ${POSTGRES_LOGIN}
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_IP: ${POSTGRES_IP}
      POSTGRES_PORT: ${POSTGRES_PORT}
      POSTGRES_DB: ${POSTGRES_DB}

      ENABLE_ARXIV: ${ENABLE_ARXIV}
      ENABLE_BIORXIV: ${ENABLE_BIORXIV}
      ENABLE_GITHUB: ${ENABLE_GITHUB}
      ENABLE_HABR: ${ENABLE_HABR}
      ENABLE_MEDRXIV: ${ENABLE_MEDRXIV}
      ENABLE_REDDIT: ${ENABLE_REDDIT}
      ENABLE_TWITTER: ${ENABLE_TWITTER}

      ARXIV_LINK: ${ARXIV_LINK} # https://arxiv.org/   http://export.arxiv.org/rss/   http://export.arxiv.org/rss/astro-ph.CO
      BIORXIV_LINK: ${BIORXIV_LINK} # http://connect.biorxiv.org/
      GITHUB_LINK: ${GITHUB_LINK}
      HABR_LINK: ${HABR_LINK} # https://habr.com/ru/
      MEDRXIV_LINK: ${MEDRXIV_LINK} # http://connect.medrxiv.org/
      REDDIT_LINK: ${REDDIT_LINK}
      TWITTER_LINK: ${TWITTER_LINK} # must be not only 1 str but many - twitter and many nitters

      ENABLE_PRINTS_ARXIV: ${ENABLE_PRINTS_ARXIV}
      ENABLE_PRINTS_BIORXIV: ${ENABLE_PRINTS_BIORXIV}
      ENABLE_PRINTS_GITHUB: ${ENABLE_PRINTS_GITHUB}
      ENABLE_PRINTS_HABR: ${ENABLE_PRINTS_HABR}
      ENABLE_PRINTS_MEDRXIV: ${ENABLE_PRINTS_MEDRXIV}
      ENABLE_PRINTS_REDDIT: ${ENABLE_PRINTS_REDDIT}
      ENABLE_PRINTS_TWITTER: ${ENABLE_PRINTS_TWITTER}
 
      ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV: ${ENABLE_WARNING_HIGH_PRINTS_FOR_ARXIV}
      ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV: ${ENABLE_WARNING_HIGH_PRINTS_FOR_BIORXIV}
      ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB: ${ENABLE_WARNING_HIGH_PRINTS_FOR_GITHUB}
      ENABLE_WARNING_HIGH_PRINTS_FOR_HABR: ${ENABLE_WARNING_HIGH_PRINTS_FOR_HABR}
      ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV: ${ENABLE_WARNING_HIGH_PRINTS_FOR_MEDRXIV}
      ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT: ${ENABLE_WARNING_HIGH_PRINTS_FOR_REDDIT}
      ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER: ${ENABLE_WARNING_HIGH_PRINTS_FOR_TWITTER}

      ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV: ${ENABLE_WARNING_LOW_PRINTS_FOR_ARXIV}
      ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV: ${ENABLE_WARNING_LOW_PRINTS_FOR_BIORXIV}
      ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB: ${ENABLE_WARNING_LOW_PRINTS_FOR_GITHUB}
      ENABLE_WARNING_LOW_PRINTS_FOR_HABR: ${ENABLE_WARNING_LOW_PRINTS_FOR_HABR}
      ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV: ${ENABLE_WARNING_LOW_PRINTS_FOR_MEDRXIV}
      ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT: ${ENABLE_WARNING_LOW_PRINTS_FOR_REDDIT}
      ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER: ${ENABLE_WARNING_LOW_PRINTS_FOR_TWITTER}

      ENABLE_ERROR_PRINTS_FOR_ARXIV: ${ENABLE_ERROR_PRINTS_FOR_ARXIV}
      ENABLE_ERROR_PRINTS_FOR_BIORXIV: ${ENABLE_ERROR_PRINTS_FOR_BIORXIV}
      ENABLE_ERROR_PRINTS_FOR_GITHUB: ${ENABLE_ERROR_PRINTS_FOR_GITHUB}
      ENABLE_ERROR_PRINTS_FOR_HABR: ${ENABLE_ERROR_PRINTS_FOR_HABR}
      ENABLE_ERROR_PRINTS_FOR_MEDRXIV: ${ENABLE_ERROR_PRINTS_FOR_MEDRXIV}
      ENABLE_ERROR_PRINTS_FOR_REDDIT: ${ENABLE_ERROR_PRINTS_FOR_REDDIT}
      ENABLE_ERROR_PRINTS_FOR_TWITTER: ${ENABLE_ERROR_PRINTS_FOR_TWITTER}

      ENABLE_SUCCESS_PRINTS_FOR_ARXIV: ${ENABLE_SUCCESS_PRINTS_FOR_ARXIV}
      ENABLE_SUCCESS_PRINTS_FOR_BIORXIV: ${ENABLE_SUCCESS_PRINTS_FOR_BIORXIV}
      ENABLE_SUCCESS_PRINTS_FOR_GITHUB: ${ENABLE_SUCCESS_PRINTS_FOR_GITHUB}
      ENABLE_SUCCESS_PRINTS_FOR_HABR: ${ENABLE_SUCCESS_PRINTS_FOR_HABR}
      ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV: ${ENABLE_SUCCESS_PRINTS_FOR_MEDRXIV}
      ENABLE_SUCCESS_PRINTS_FOR_REDDIT: ${ENABLE_SUCCESS_PRINTS_FOR_REDDIT}
      ENABLE_SUCCESS_PRINTS_FOR_TWITTER: ${ENABLE_SUCCESS_PRINTS_FOR_TWITTER}
  
      ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV: ${ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_ARXIV}
      ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV: ${ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_BIORXIV}
      ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB: ${ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_GITHUB}
      ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR: ${ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_HABR}
      ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV: ${ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_MEDRXIV}
      ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT: ${ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_REDDIT}
      ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER: ${ENABLE_PARTIAL_SUCCESS_PRINTS_FOR_TWITTER}

      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_ARXIV}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_BIORXIV}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_GITHUB}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_HABR}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_MEDRXIV}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_REDDIT}
      ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER: ${ENABLE_CLEANING_WARNING_LOGS_DIRECTORY_FOR_TWITTER}

      ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV: ${ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_ARXIV}
      ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV: ${ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_BIORXIV}
      ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB: ${ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_GITHUB}
      ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR: ${ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_HABR}
      ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV: ${ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_MEDRXIV}
      ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT: ${ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_REDDIT}
      ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER: ${ENABLE_CLEANING_WARNING_LOGS_DB_IN_MONGO_FOR_TWITTER}

      ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV: ${ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_ARXIV}
      ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV: ${ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_BIORXIV}
      ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB: ${ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_GITHUB}
      ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR: ${ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_HABR}
      ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV: ${ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_MEDRXIV}
      ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT: ${ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_REDDIT}
      ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER: ${ENABLE_CLEANING_WARNING_LOGS_DB_COLLECTIONS_IN_MONGO_FOR_TWITTER}

      ENABLE_TIME_MEASUREMENT_FOR_ARXIV: ${ENABLE_TIME_MEASUREMENT_FOR_ARXIV}
      ENABLE_TIME_MEASUREMENT_FOR_BIORXIV: ${ENABLE_TIME_MEASUREMENT_FOR_BIORXIV}
      ENABLE_TIME_MEASUREMENT_FOR_GITHUB: ${ENABLE_TIME_MEASUREMENT_FOR_GITHUB}
      ENABLE_TIME_MEASUREMENT_FOR_HABR: ${ENABLE_TIME_MEASUREMENT_FOR_HABR}
      ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV: ${ENABLE_TIME_MEASUREMENT_FOR_MEDRXIV}
      ENABLE_TIME_MEASUREMENT_FOR_REDDIT: ${ENABLE_TIME_MEASUREMENT_FOR_REDDIT}
      ENABLE_TIME_MEASUREMENT_FOR_TWITTER: ${ENABLE_TIME_MEASUREMENT_FOR_TWITTER}

      ENABLE_INFO_FOR_ARXIV: ${ENABLE_INFO_FOR_ARXIV}
      ENABLE_INFO_FOR_BIORXIV: ${ENABLE_INFO_FOR_BIORXIV}
      ENABLE_INFO_FOR_GITHUB: ${ENABLE_INFO_FOR_GITHUB}
      ENABLE_INFO_FOR_HABR: ${ENABLE_INFO_FOR_HABR}
      ENABLE_INFO_FOR_MEDRXIV: ${ENABLE_INFO_FOR_MEDRXIV}
      ENABLE_INFO_FOR_REDDIT: ${ENABLE_INFO_FOR_REDDIT}
      ENABLE_INFO_FOR_TWITTER: ${ENABLE_INFO_FOR_TWITTER}

      ENABLE_LINKS_LIMIT_FOR_ARXIV: ${ENABLE_LINKS_LIMIT_FOR_ARXIV}
      ENABLE_LINKS_LIMIT_FOR_BIORXIV: ${ENABLE_LINKS_LIMIT_FOR_BIORXIV}
      ENABLE_LINKS_LIMIT_FOR_GITHUB: ${ENABLE_LINKS_LIMIT_FOR_GITHUB}
      ENABLE_LINKS_LIMIT_FOR_HABR: ${ENABLE_LINKS_LIMIT_FOR_HABR}
      ENABLE_LINKS_LIMIT_FOR_MEDRXIV: ${ENABLE_LINKS_LIMIT_FOR_MEDRXIV}
      ENABLE_LINKS_LIMIT_FOR_REDDIT: ${ENABLE_LINKS_LIMIT_FOR_REDDIT}
      ENABLE_LINKS_LIMIT_FOR_TWITTER: ${ENABLE_LINKS_LIMIT_FOR_TWITTER}

      ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO: ${ENABLE_RANDOMIZE_ORDER_FOR_ARXIV_LINK_PARTS_FOR_MONGO}
      ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO: ${ENABLE_RANDOMIZE_ORDER_FOR_BIORXIV_LINK_PARTS_FOR_MONGO}
      ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO: ${ENABLE_RANDOMIZE_ORDER_FOR_GITHUB_LINK_PARTS_FOR_MONGO}
      ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO: ${ENABLE_RANDOMIZE_ORDER_FOR_HABR_LINK_PARTS_FOR_MONGO}
      ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO: ${ENABLE_RANDOMIZE_ORDER_FOR_MEDRXIV_LINK_PARTS_FOR_MONGO}
      ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO: ${ENABLE_RANDOMIZE_ORDER_FOR_REDDIT_LINK_PARTS_FOR_MONGO}
      ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO: ${ENABLE_RANDOMIZE_ORDER_FOR_TWITTER_LINK_PARTS_FOR_MONGO}

      LINKS_LIMIT_FOR_ARXIV: ${LINKS_LIMIT_FOR_ARXIV} # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
      LINKS_LIMIT_FOR_BIORXIV: ${LINKS_LIMIT_FOR_BIORXIV} # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
      LINKS_LIMIT_FOR_GITHUB: ${LINKS_LIMIT_FOR_GITHUB} # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
      LINKS_LIMIT_FOR_HABR: ${LINKS_LIMIT_FOR_HABR} # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
      LINKS_LIMIT_FOR_MEDRXIV: ${LINKS_LIMIT_FOR_MEDRXIV} # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
      LINKS_LIMIT_FOR_REDDIT: ${LINKS_LIMIT_FOR_REDDIT} # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)
      LINKS_LIMIT_FOR_TWITTER: ${LINKS_LIMIT_FOR_TWITTER} # i64, but cannot be 0 or negative value (disable link limits instead of using 0(zero) for this parameter)

      ERROR_RED: ${ERROR_RED} # u8
      ERROR_GREEN: ${ERROR_GREEN} # u8
      ERROR_BLUE: ${ERROR_BLUE} # u8
      WARNING_HIGH_RED: ${WARNING_HIGH_RED} # u8
      WARNING_HIGH_GREEN: ${WARNING_HIGH_GREEN} # u8
      WARNING_HIGH_BLUE: ${WARNING_HIGH_BLUE} # u8
      WARNING_LOW_RED: ${WARNING_LOW_RED} # u8
      WARNING_LOW_GREEN: ${WARNING_LOW_GREEN} # u8
      WARNING_LOW_BLUE: ${WARNING_LOW_BLUE} # u8
      SUCCESS_RED: ${SUCCESS_RED} # u8
      SUCCESS_GREEN: ${SUCCESS_GREEN} # u8
      SUCCESS_BLUE: ${SUCCESS_BLUE}  # u8
      PARTIAL_SUCCESS_RED: ${PARTIAL_SUCCESS_RED} # u8
      PARTIAL_SUCCESS_GREEN: ${PARTIAL_SUCCESS_GREEN} # u8
      PARTIAL_SUCCESS_BLUE: ${PARTIAL_SUCCESS_BLUE} # u8
      CLEANING_RED: ${CLEANING_RED} # u8
      CLEANING_GREEN: ${CLEANING_GREEN} # u8
      CLEANING_BLUE: ${CLEANING_BLUE} # u8
      TIME_MEASUREMENT_RED: ${TIME_MEASUREMENT_RED} # u8
      TIME_MEASUREMENT_GREEN: ${TIME_MEASUREMENT_GREEN} # u8
      TIME_MEASUREMENT_BLUE: ${TIME_MEASUREMENT_BLUE} # u8
      INFO_RED: ${INFO_RED} # u8
      INFO_GREEN: ${INFO_GREEN} # u8
      INFO_BLUE: ${INFO_BLUE} # u8
volumes:
  postgresql_volume:
  mongodb_volume:"#;
