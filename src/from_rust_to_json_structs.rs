#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Root {
    pub kind: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Data {
    pub modhash: String,
    pub dist: i64,
    pub children: Vec<Children>,
    pub after: String,
    pub before: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Children {
    pub kind: String,
    pub data: Data2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Data2 {
    pub approved_at_utc: ::serde_json::Value,
    pub subreddit: String,
    pub selftext: String,
    pub author_fullname: String,
    pub saved: bool,
    pub mod_reason_title: ::serde_json::Value,
    pub gilded: i64,
    pub clicked: bool,
    pub title: String,
    pub link_flair_richtext: Vec<LinkFlairRichtext>,
    pub subreddit_name_prefixed: String,
    #[serde(default)]
    pub collections: Vec<Collection>,
    pub hidden: bool,
    pub pwls: i64,
    pub link_flair_css_class: Option<String>,
    pub downs: i64,
    pub thumbnail_height: Option<i64>,
    pub top_awarded_type: ::serde_json::Value,
    pub hide_score: bool,
    pub name: String,
    pub quarantine: bool,
    pub link_flair_text_color: String,
    pub upvote_ratio: f64,
    pub author_flair_background_color: Option<String>,
    pub subreddit_type: String,
    pub ups: i64,
    pub total_awards_received: i64,
    pub media_embed: MediaEmbed,
    pub thumbnail_width: Option<i64>,
    pub author_flair_template_id: Option<String>,
    pub is_original_content: bool,
    pub user_reports: Vec<::serde_json::Value>,
    pub secure_media: ::serde_json::Value,
    pub is_reddit_media_domain: bool,
    pub is_meta: bool,
    pub category: ::serde_json::Value,
    pub secure_media_embed: SecureMediaEmbed,
    pub link_flair_text: Option<String>,
    pub can_mod_post: bool,
    pub score: i64,
    pub approved_by: ::serde_json::Value,
    pub author_premium: bool,
    pub thumbnail: String,
    pub edited: ::serde_json::Value,
    pub author_flair_css_class: Option<String>,
    pub author_flair_richtext: Vec<AuthorFlairRichtext>,
    pub gildings: Gildings,
    pub post_hint: Option<String>,
    pub content_categories: ::serde_json::Value,
    pub is_self: bool,
    pub mod_note: ::serde_json::Value,
    pub created: f64,
    pub link_flair_type: String,
    pub wls: i64,
    pub removed_by_category: ::serde_json::Value,
    pub banned_by: ::serde_json::Value,
    pub author_flair_type: String,
    pub domain: String,
    pub allow_live_comments: bool,
    pub selftext_html: Option<String>,
    pub likes: ::serde_json::Value,
    pub suggested_sort: Option<String>,
    pub banned_at_utc: ::serde_json::Value,
    pub view_count: ::serde_json::Value,
    pub archived: bool,
    pub no_follow: bool,
    pub is_crosspostable: bool,
    pub pinned: bool,
    #[serde(rename = "over_18")]
    pub over18: bool,
    pub preview: Option<Preview>,
    pub all_awardings: Vec<AllAwarding>,
    pub awarders: Vec<::serde_json::Value>,
    pub media_only: bool,
    pub can_gild: bool,
    pub spoiler: bool,
    pub locked: bool,
    pub author_flair_text: Option<String>,
    pub treatment_tags: Vec<::serde_json::Value>,
    pub visited: bool,
    pub removed_by: ::serde_json::Value,
    pub num_reports: ::serde_json::Value,
    pub distinguished: Option<String>,
    pub subreddit_id: String,
    pub mod_reason_by: ::serde_json::Value,
    pub removal_reason: ::serde_json::Value,
    pub link_flair_background_color: String,
    pub id: String,
    pub is_robot_indexable: bool,
    pub report_reasons: ::serde_json::Value,
    pub author: String,
    pub discussion_type: ::serde_json::Value,
    pub num_comments: i64,
    pub send_replies: bool,
    pub whitelist_status: String,
    pub contest_mode: bool,
    pub mod_reports: Vec<::serde_json::Value>,
    pub author_patreon_flair: bool,
    pub author_flair_text_color: Option<String>,
    pub permalink: String,
    pub parent_whitelist_status: String,
    pub stickied: bool,
    pub url: String,
    pub subreddit_subscribers: i64,
    pub created_utc: f64,
    pub num_crossposts: i64,
    pub media: ::serde_json::Value,
    pub is_video: bool,
    pub url_overridden_by_dest: Option<String>,
    pub link_flair_template_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct LinkFlairRichtext {
    pub e: String,
    pub t: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Collection {
    pub permalink: String,
    pub link_ids: Vec<String>,
    pub description: String,
    pub title: String,
    pub created_at_utc: f64,
    pub subreddit_id: String,
    pub author_name: String,
    pub collection_id: String,
    pub author_id: String,
    pub last_update_utc: f64,
    pub display_layout: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct MediaEmbed {}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct SecureMediaEmbed {}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AuthorFlairRichtext {
    pub a: Option<String>,
    pub e: String,
    pub u: Option<String>,
    pub t: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Gildings {
    #[serde(rename = "gid_1")]
    pub gid1: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Preview {
    pub images: Vec<Image>,
    pub enabled: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Image {
    pub source: Source,
    pub resolutions: Vec<Resolution>,
    pub variants: Variants,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Source {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Resolution {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Variants {}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct AllAwarding {
    pub giver_coin_reward: ::serde_json::Value,
    pub subreddit_id: ::serde_json::Value,
    pub is_new: bool,
    pub days_of_drip_extension: i64,
    pub coin_price: i64,
    pub id: String,
    pub penny_donate: ::serde_json::Value,
    pub award_sub_type: String,
    pub coin_reward: i64,
    pub icon_url: String,
    pub days_of_premium: i64,
    pub resized_icons: Vec<ResizedIcon>,
    pub icon_width: i64,
    pub static_icon_width: i64,
    pub start_date: ::serde_json::Value,
    pub is_enabled: bool,
    pub description: String,
    pub end_date: ::serde_json::Value,
    pub subreddit_coin_reward: i64,
    pub count: i64,
    pub static_icon_height: i64,
    pub name: String,
    pub resized_static_icons: Vec<ResizedStaticIcon>,
    pub icon_format: ::serde_json::Value,
    pub icon_height: i64,
    pub penny_price: ::serde_json::Value,
    pub award_type: String,
    pub static_icon_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ResizedIcon {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ResizedStaticIcon {
    pub url: String,
    pub width: i64,
    pub height: i64,
}
