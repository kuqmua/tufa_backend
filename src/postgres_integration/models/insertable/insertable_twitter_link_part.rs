use crate::postgres_integration::schemas::twitter_link_parts_schema::twitter_link_parts;

#[derive(Insertable)]
#[table_name = "twitter_link_parts"] //meaning in postgres should exists twitter_link_parts table
pub struct InsertableTwitterLinkPart {
    pub link_part: String,
}