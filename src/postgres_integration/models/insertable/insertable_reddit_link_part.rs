use crate::postgres_integration::schemas::reddit_link_parts_schema::reddit_link_parts;

#[derive(Insertable)]
#[table_name = "reddit_link_parts"] //meaning in postgres should exists reddit_link_parts table
pub struct InsertableRedditLinkPart {
    pub link_part: String,
}