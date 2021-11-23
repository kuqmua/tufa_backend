use std::collections::HashMap;

use crate::helpers::resource::Resource;

use crate::providers::provider_kind_enum::ProviderKind;

// #[derive(Debug)]
// pub enum LocalResourceError {
//     Local {
//         serde_from_str_file: SerdeFromStrFile(StrFile, serde_json::Error), // HashMap<ProviderKind, (StrFile, serde_json::Error)>
//         FsReadToString(Path, std::io::Error) //HashMap<ProviderKind, (Path, std::io::Error)>
//     },
//     Mongodb {

//     },
//     PostgreSql, //todo
// }

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub async fn get_providers_link_parts_as_hashmap(
    resource: &Resource,
) -> HashMap<ProviderKind, Vec<String>> {
    //todo: return different type as errors or success enum
    //todo: write here converison to common return type
    match resource {
        Resource::Local => {
            let (success_hashmap, errors_hashmap) =
                ProviderKind::get_providers_json_local_data_processed();
            success_hashmap
        }
        // HashMap<ProviderKind, Result<Result<Vec<String>, serde_json::Error>, std::io::Error>>
        Resource::Mongodb => ProviderKind::mongo_get_providers_link_parts_processed().await,
        //     Result<
        //     HashMap<ProviderKind, Result<Vec<String>, mongodb::error::Error>>,
        //     mongodb::error::Error,
        // >
        Resource::PostgreSql => {
            todo!()
        }
    }
}
