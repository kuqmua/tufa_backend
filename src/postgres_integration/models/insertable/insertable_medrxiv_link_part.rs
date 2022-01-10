use crate::postgres_integration::schemas::medrxiv_link_parts_schema::medrxiv_link_parts;

#[derive(Insertable)]
#[table_name = "medrxiv_link_parts"] //meaning in postgres should exists medrxiv_link_parts table
pub struct InsertableMedrxivLinkPart {
    pub link_part: String,
}