use crate::postgres_integration::schemas::habr_link_parts_schema::habr_link_parts;

#[derive(Insertable)]
#[table_name = "habr_link_parts"] //meaning in postgres should exists habr_link_parts table
pub struct InsertableHabrLinkPart {
    pub link_part: String,
}