use crate::postgres_integration::schemas::biorxiv_link_parts_schema::biorxiv_link_parts;

#[derive(Insertable)]
#[table_name = "biorxiv_link_parts"] //meaning in postgres should exists biorxiv_link_parts table
pub struct InsertableBiorxivLinkPart {
    pub link_part: String,
}