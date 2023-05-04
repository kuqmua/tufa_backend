pub async fn init_postgres<'a>(
    providers_json_local_data_hashmap: std::collections::HashMap<tufa_common::repositories_types::tufa_server::providers::provider_kind::provider_kind_enum::ProviderKind, Vec<String>>,
) -> Result<(), Box<tufa_common::repositories_types::tufa_server::init_dbs_logic::init_postgres::PostgresInitErrorNamed<'a>>> {
    match crate::postgres_integration::postgres_establish_connection::postgres_establish_connection(providers_json_local_data_hashmap.len() as u32).await {
        Err(e) => Err(Box::new(
            tufa_common::repositories_types::tufa_server::init_dbs_logic::init_postgres::PostgresInitErrorNamed::EstablishConnection { 
                establish_connection: *e, 
                code_occurence: tufa_common::code_occurence!()
            }
        )),
        Ok(pool) => {
            if let Err(e) = crate::postgres_integration::postgres_create_providers_tables_if_not_exists::postgres_create_providers_tables_if_not_exists(
                &providers_json_local_data_hashmap,
                &pool,
            )
            .await
            {
                return Err(Box::new(
                    tufa_common::repositories_types::tufa_server::init_dbs_logic::init_postgres::PostgresInitErrorNamed::CreateTableQueries { 
                        create_table_queries: *e,
                        code_occurence: tufa_common::code_occurence!() 
                    }
                ));
            }
            if let Err(e) = crate::postgres_integration::postgres_check_providers_link_parts_tables_are_empty::postgres_check_providers_link_parts_tables_are_empty(
                &providers_json_local_data_hashmap,
                &pool,
            )
            .await
            {
                return Err(Box::new(
                    tufa_common::repositories_types::tufa_server::init_dbs_logic::init_postgres::PostgresInitErrorNamed::CheckProviderLinksTablesAreEmpty { 
                        check_provider_links_tables_are_empty: *e, 
                        code_occurence: tufa_common::code_occurence!()  
                    }
                ));
            }
            if let Err(e) = crate::postgres_integration::postgres_delete_all_from_providers_link_parts_tables::postgres_delete_all_from_providers_link_parts_tables(
                &providers_json_local_data_hashmap,
                &pool,
                false,
            )
            .await
            {
                return Err(Box::new(
                    tufa_common::repositories_types::tufa_server::init_dbs_logic::init_postgres::PostgresInitErrorNamed::DeleteAllFromProvidersTables { 
                        delete_all_from_providers_tables: *e, 
                        code_occurence: tufa_common::code_occurence!()   
                    }
                ));
            }
            if let Err(e) = crate::postgres_integration::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length::postgres_check_providers_links_tables_length_rows_equal_initialization_data_length(
                &providers_json_local_data_hashmap,
                &pool,
                false,
            )
            .await {
                return Err(Box::new(
                    tufa_common::repositories_types::tufa_server::init_dbs_logic::init_postgres::PostgresInitErrorNamed::CheckProvidersLinksTablesLengthRowsEqualInitializationDataLength {          
                        check_providers_links_tables_length_rows_equal_initialization_data_length: *e, 
                        code_occurence: tufa_common::code_occurence!()   
                    }
            ));
            }
            if let Err(e) = crate::postgres_integration::postgres_insert_link_parts_into_providers_tables::postgres_insert_link_parts_into_providers_tables(
                &providers_json_local_data_hashmap,
                &pool,
                false,
            )
            .await
            {
                return Err(Box::new(
                    tufa_common::repositories_types::tufa_server::init_dbs_logic::init_postgres::PostgresInitErrorNamed::InsertLinkPartsIntoProvidersTables { 
                        insert_link_parts_into_providers_tables: *e, 
                        code_occurence: tufa_common::code_occurence!()    
                    }
                ));
            }
            Ok(())
        }
    }
}
