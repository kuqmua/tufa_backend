use crate::postgres_integration::schemas::github_link_parts_schema::github_link_parts;

#[derive(Insertable)]
#[table_name = "github_link_parts"] //meaning in postgres should exists github_link_parts table
pub struct InsertableGithubLinkPart {
    pub link_part: String,
}