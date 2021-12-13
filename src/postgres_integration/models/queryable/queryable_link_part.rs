#[derive(Queryable, Debug)]
pub struct QueryableLinkPart {
    pub id: i32,
    pub provider_kind: String,
    pub link_part: String,
}
