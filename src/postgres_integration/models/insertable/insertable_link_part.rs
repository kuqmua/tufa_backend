use crate::postgres_integration::models::queryable::queryable_link_part::QueryableLinkPart;
use crate::postgres_integration::schema::providers_link_parts;

use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "providers_link_parts"] //meaning in poostgres should exists providers_link_parts table
pub struct InsertableLinkPart {
    pub provider_kind: String,
    pub link_part: String,
}

impl InsertableLinkPart {
    pub fn insert_into_postgres(
        connection: &PgConnection,
        post: Self,
    ) -> Result<QueryableLinkPart, diesel::result::Error> {
        diesel::insert_into(providers_link_parts::table)
            .values(&post)
            .get_result(connection)
    }
    pub fn insert_vec_into_postgres(
        connection: &PgConnection,
        posts: Vec<Self>,
    ) -> Result<QueryableLinkPart, diesel::result::Error> {
        diesel::insert_into(providers_link_parts::table)
            .values(&posts)
            .get_result(connection)
    }
}
