use crate::postgres_integration::models::queryable::queryable_link_part::QueryableLinkPart;
use crate::postgres_integration::schema::providers_link_parts;

use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "providers_link_parts"] //meaning in poostgres should exists providers_link_parts table
pub struct InsertableLinkPart<'a> {
    pub link_part: &'a str,
}

impl<'a> InsertableLinkPart<'a> {
    pub fn insert_into_postgres(
        connection: &PgConnection,
        new_post: Self,
    ) -> Result<QueryableLinkPart, diesel::result::Error> {
        diesel::insert_into(providers_link_parts::table)
            .values(&new_post)
            .get_result(connection)
    }
}
