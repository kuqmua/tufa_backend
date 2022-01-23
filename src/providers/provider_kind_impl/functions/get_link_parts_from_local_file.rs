use std::fmt;
use std::string::FromUtf8Error;

use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::providers_info::providers_init_json_schema::ProvidersInitJsonSchema;
use crate::traits::provider_kind_trait::ProviderKindTrait;

#[derive(Debug, BoxErrFromErrDerive, ImplDisplayDerive)]
pub struct ProviderGetLocalDataError {
    pub source: Box<ProviderGetLocalDataErrorEnum>,
}

#[derive(Debug)] //, ImplFromForUpperStruct
pub enum ProviderGetLocalDataErrorEnum {
    TokioFsFileOpen(ProviderGetLocalDataTokioFsFileOpenErrorStruct),
    TokioIoAsyncReadExtReadBuf(ProviderGetLocalDataTokioIoAsyncReadExtReadBufErrorStruct),
    StringFromUtf8(FromUtf8Error),
    SerdeJsonFromStr(serde_json::Error),
}

#[derive(Debug)]
pub struct ProviderGetLocalDataTokioFsFileOpenErrorStruct {
    source: std::io::Error,
}

#[derive(Debug)]
pub struct ProviderGetLocalDataTokioIoAsyncReadExtReadBufErrorStruct {
    source: std::io::Error,
}

impl ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub async fn get_link_parts_from_local_file(
        self,
    ) -> Result<Vec<String>, ProviderGetLocalDataError> {
        match tokio::fs::File::open(&self.get_init_local_data_file_path()).await {
            Err(e) => Err(ProviderGetLocalDataError {
                source: Box::new(ProviderGetLocalDataErrorEnum::TokioFsFileOpen(
                    ProviderGetLocalDataTokioFsFileOpenErrorStruct { source: e },
                )),
            }),
            Ok(mut file) => {
                let mut content = Vec::new();
                if let Err(e) = tokio::io::AsyncReadExt::read_to_end(&mut file, &mut content).await {
                    return Err(ProviderGetLocalDataError {
                        source: Box::new(
                            ProviderGetLocalDataErrorEnum::TokioIoAsyncReadExtReadBuf(
                                ProviderGetLocalDataTokioIoAsyncReadExtReadBufErrorStruct {
                                    source: e,
                                },
                            ),
                        ),
                    });
                }
                match serde_json::from_slice::<ProvidersInitJsonSchema>(&content) {
                    Err(e) => Err(ProviderGetLocalDataError {
                        source: Box::new(ProviderGetLocalDataErrorEnum::SerdeJsonFromStr(e)),
                    }),
                    Ok(file_content_as_struct) => {
                        Ok(file_content_as_struct.data)
                    },
                }
            }
        }
    }
}
