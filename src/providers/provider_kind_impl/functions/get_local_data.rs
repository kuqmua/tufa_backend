use std::string::FromUtf8Error;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::providers_info::providers_init_json_schema::ProvidersInitJsonSchema;
use crate::traits::provider_kind_trait::ProviderKindTrait;

use async_std::fs::File;
use async_std::prelude::*;

#[derive(Debug)]
pub enum ProvidersLocalDataError {
    FileOpen(std::io::Error),
    FileReadToEnd(std::io::Error),
    StringFromUtf8(FromUtf8Error),
    SerdeJsonFromStr(serde_json::Error),
}

impl ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub async fn get_local_data(
        self,
    ) -> Result<(ProviderKind, Vec<String>), (ProviderKind, ProvidersLocalDataError)> {
        match File::open(&self.get_init_local_data_file_path()).await {
            Err(e) => Err((self, ProvidersLocalDataError::FileOpen(e))),
            Ok(mut file) => {
                let mut buf = Vec::new();
                match file.read_to_end(&mut buf).await {
                    Err(e) => Err((self, ProvidersLocalDataError::FileReadToEnd(e))),
                    Ok(_) => match String::from_utf8(buf) {
                        Err(e) => Err((self, ProvidersLocalDataError::StringFromUtf8(e))),
                        Ok(file_content) => {
                            let file_content_from_str_result: Result<
                                ProvidersInitJsonSchema,
                                serde_json::Error,
                            > = serde_json::from_str(&file_content);
                            match file_content_from_str_result {
                                Err(e) => Err((self, ProvidersLocalDataError::SerdeJsonFromStr(e))),
                                Ok(file_content_as_struct) => {
                                    Ok((self, file_content_as_struct.data))
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}
