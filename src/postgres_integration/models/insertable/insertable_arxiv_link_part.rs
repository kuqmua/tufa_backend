use crate::postgres_integration::schemas::arxiv_link_parts_schema::arxiv_link_parts;

#[derive(Insertable)]
#[table_name = "arxiv_link_parts"] //meaning in postgres should exists arxiv_link_parts table
pub struct InsertableArxivLinkPart {
    pub link_part: String,
}
