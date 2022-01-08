use crate::postgres_integration::schemas::providers_link_parts_schema::providers_link_parts;

#[derive(Insertable)]
#[table_name = "providers_link_parts"] //meaning in poostgres should exists providers_link_parts table
pub struct InsertableLinkPart {
    pub provider_kind: String,
    pub link_part: String,
}
