#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RedditStructForParsing {
    pub data: RedditStructForParsingVector,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RedditStructForParsingVector {
    pub children: Vec<RedditStructForParsingVectorChild>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RedditStructForParsingVectorChild {
    pub data: RedditStructForParsingVectorChildData,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RedditStructForParsingVectorChildData {
    pub link: Option<String>,
    pub subreddit: String,
    pub selftext: String,
    pub id: String, // может понадобиться
    pub author: String,
    pub title: String,
    pub domain: String, //сайт хоста
    pub permalink: String,
    pub thumbnail: String, //todo
    pub created_utc: f64,  // время
    pub ups: f64,
    pub score: f64, //чем отличается score от ups
    pub num_comments: u64,
    pub over_18: bool,
    pub quarantine: bool,
    pub is_self: bool, //может понадобиться мб
    pub saved: bool,
    pub url: String,
}
// "approved_at_utc": null,
// "subreddit": "3Dprinting",
// "selftext": "",
// "author_fullname": "t2_n6r6oak",
// "saved": false,
// "mod_reason_title": null,
// "gilded": 0,
// "clicked": false,
// "title": "Experiment - load coordinates from *.stl (binary) to excel, it's crazy.",
// "link_flair_richtext": [
// {
// "e": "text",
// "t": "Image"
// }
// ],
// "subreddit_name_prefixed": "r/3Dprinting",
// "hidden": false,
// "pwls": 6,
// "link_flair_css_class": "g",
// "downs": 0,
// "thumbnail_height": 102,
// "top_awarded_type": null,
// "hide_score": true,
// "name": "t3_n5anff",
// "quarantine": false,
// "link_flair_text_color": "dark",
// "upvote_ratio": 1,
// "author_flair_background_color": null,
// "subreddit_type": "public",
// "ups": 1,
// "total_awards_received": 0,
// "media_embed": {},
// "thumbnail_width": 140,
// "author_flair_template_id": null,
// "is_original_content": false,
// "user_reports": [],
// "secure_media": null,
// "is_reddit_media_domain": true,
// "is_meta": false,
// "category": null,
// "secure_media_embed": {},
// "link_flair_text": "Image",
// "can_mod_post": false,
// "score": 1,
// "approved_by": null,
// "author_premium": false,
// "thumbnail": "https://b.thumbs.redditmedia.com/KSOgyDVFIwdwOoRjTBuiNBCO6No23i09Yc5NXLJehVk.jpg",
// "edited": false,
// "author_flair_css_class": null,
// "author_flair_richtext": [],
// "gildings": {},
// "post_hint": "image",
// "content_categories": null,
// "is_self": false,
// "mod_note": null,
// "created": 1620230469,
// "link_flair_type": "richtext",
// "wls": 6,
// "removed_by_category": null,
// "banned_by": null,
// "author_flair_type": "text",
// "domain": "i.redd.it",
// "allow_live_comments": false,
// "selftext_html": null,
// "likes": null,
// "suggested_sort": null,
// "banned_at_utc": null,
// "url_overridden_by_dest": "https://i.redd.it/so1xw3dpd9x61.png",
// "view_count": null,
// "archived": false,
// "no_follow": true,
// "is_crosspostable": true,
// "pinned": false,
// "over_18": false,
// "preview": {
// "images": [
// {
// "source": {
// "url": "https://preview.redd.it/so1xw3dpd9x61.png?auto=webp&amp;s=9cc0a791c6ccd5b7fadc78cab3acfbb596527002",
// "width": 1023,
// "height": 750
// },
// "resolutions": [
// {
// "url": "https://preview.redd.it/so1xw3dpd9x61.png?width=108&amp;crop=smart&amp;auto=webp&amp;s=8451c1bf37cd073413dc69db8db6e93649544582",
// "width": 108,
// "height": 79
// },
// {
// "url": "https://preview.redd.it/so1xw3dpd9x61.png?width=216&amp;crop=smart&amp;auto=webp&amp;s=0539def7ee7a7732d820912b13f834710803d3d5",
// "width": 216,
// "height": 158
// },
// {
// "url": "https://preview.redd.it/so1xw3dpd9x61.png?width=320&amp;crop=smart&amp;auto=webp&amp;s=3057f38b5e1aa4367d826eedb81446dc41b38f1f",
// "width": 320,
// "height": 234
// },
// {
// "url": "https://preview.redd.it/so1xw3dpd9x61.png?width=640&amp;crop=smart&amp;auto=webp&amp;s=93adfd16cf345366d0db1652d68447103433cf4a",
// "width": 640,
// "height": 469
// },
// {
// "url": "https://preview.redd.it/so1xw3dpd9x61.png?width=960&amp;crop=smart&amp;auto=webp&amp;s=5eed04d3d0074fd8bdb1990baac8f124518837d1",
// "width": 960,
// "height": 703
// }
// ],
// "variants": {},
// "id": "aaJ1jfcNT4SwBGhciv45q_v9Wy3VchZY-UAP5pbc94c"
// }
// ],
// "enabled": true
// },
// "all_awardings": [],
// "awarders": [],
// "media_only": false,
// "can_gild": true,
// "spoiler": false,
// "locked": false,
// "author_flair_text": null,
// "treatment_tags": [],
// "visited": false,
// "removed_by": null,
// "num_reports": null,
// "distinguished": null,
// "subreddit_id": "t5_2rk5q",
// "mod_reason_by": null,
// "removal_reason": null,
// "link_flair_background_color": "",
// "id": "n5anff",
// "is_robot_indexable": true,
// "report_reasons": null,
// "author": "mochr91",
// "discussion_type": null,
// "num_comments": 0,
// "send_replies": true,
// "whitelist_status": "all_ads",
// "contest_mode": false,
// "mod_reports": [],
// "author_patreon_flair": false,
// "author_flair_text_color": null,
// "permalink": "/r/3Dprinting/comments/n5anff/experiment_load_coordinates_from_stl_binary_to/",
// "parent_whitelist_status": "all_ads",
// "stickied": false,
// "url": "https://i.redd.it/so1xw3dpd9x61.png",
// "subreddit_subscribers": 700555,
// "created_utc": 1620201669,
// "num_crossposts": 0,
// "media": null,
// "is_video": false
