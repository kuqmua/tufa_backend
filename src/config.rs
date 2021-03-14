pub const ENABLE_ALL_PROVIDERS: bool = true;
// pub const ENABLE_REDDIT: bool = if ENABLE_ALL_PROVIDERS { false } else { false };
pub const ENABLE_ARXIV: bool = ENABLE_ALL_PROVIDERS & false | false; //ENABLE_ALL_PROVIDERS ? true :
pub const ENABLE_BIORXIV: bool = ENABLE_ALL_PROVIDERS & false | false;
pub const ENABLE_MEDRXIV: bool = ENABLE_ALL_PROVIDERS & true | false;
//-------------------------------------------------------
pub const STARTING_CHECK_URL: &str = "https://www.google.com/";
// pub const REDDIT_URL: &str = "https://www.reddit.com/";
pub const ARXIV_URL: &str = "https://www.google.com/"; //https://arxiv.org/ //"http://export.arxiv.org/rss/"
pub const BIORXIV_URL: &str = "https://www.google.com/"; //http://connect.biorxiv.org/
pub const MEDRXIV_URL: &str = "https://www.google.com/"; //http://connect.medrxiv.org/

//-------------------------------------------------------
pub const ENABLE_ALL_PROVIDERS_PRINTS: bool = true;
// pub const ENABLE_PRINTS_REDDIT: bool = ENABLE_ALL_PROVIDERS_PRINTS & true | false ;
pub const ENABLE_PRINTS_ARXIV: bool = ENABLE_ALL_PROVIDERS_PRINTS & false | false;
pub const ENABLE_PRINTS_BIORXIV: bool = ENABLE_ALL_PROVIDERS_PRINTS & false | false;
pub const ENABLE_PRINTS_MEDRXIV: bool = ENABLE_ALL_PROVIDERS_PRINTS & true | false;
//-------------------------------------------------------
pub const ENABLE_ALL_PROVIDERS_WARNING_PRINTS: bool = true;
// pub const ENABLE_WARNING_PRINTS_REDDIT: bool = ENABLE_ALL_PROVIDERS_WARNING_PRINTS & true | false;
pub const ENABLE_WARNING_PRINTS_ARXIV: bool = ENABLE_ALL_PROVIDERS_WARNING_PRINTS & true | false;
pub const ENABLE_WARNING_PRINTS_BIORXIV: bool = ENABLE_ALL_PROVIDERS_WARNING_PRINTS & true | false;
pub const ENABLE_WARNING_PRINTS_MEDRXIV: bool = ENABLE_ALL_PROVIDERS_WARNING_PRINTS & true | false;
//-------------------------------------------------------
pub const ENABLE_ALL_PROVIDERS_ERROR_PRINTS: bool = true;
// pub const ENABLE_ERROR_PRINTS_REDDIT: bool = ENABLE_ALL_PROVIDERS_ERROR_PRINTS & true | false;
pub const ENABLE_ERROR_PRINTS_ARXIV: bool = ENABLE_ALL_PROVIDERS_ERROR_PRINTS & true | false;
pub const ENABLE_ERROR_PRINTS_BIORXIV: bool = ENABLE_ALL_PROVIDERS_ERROR_PRINTS & true | false;
pub const ENABLE_ERROR_PRINTS_MEDRXIV: bool = ENABLE_ALL_PROVIDERS_ERROR_PRINTS & true | false;
//-------------------------------------------------------
pub const ENABLE_PRINTS_HANDLE: bool = true;
pub const ENABLE_ERROR_PRINTS_HANDLE: bool = true;
