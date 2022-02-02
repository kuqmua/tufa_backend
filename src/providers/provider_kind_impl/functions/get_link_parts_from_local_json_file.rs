use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::providers_info::providers_init_json_schema::ProvidersInitJsonSchema;
use crate::traits::provider_kind_trait::ProviderKindTrait;

#[derive(Debug)]
pub enum GetLinkPartsFromLocalJsonFileErrorEnum {
    TokioFsFileOpen {
        source: std::io::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
    TokioIoAsyncReadExtReadToEnd {
        source: std::io::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
    SerdeJsonFromSlice {
        source: serde_json::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
}

impl ProviderKind {
    #[deny(clippy::indexing_slicing, clippy::unwrap_used)]
    pub async fn get_link_parts_from_local_json_file(
        self,
    ) -> Result<Vec<String>, Box<GetLinkPartsFromLocalJsonFileErrorEnum>> {
        match tokio::fs::File::open(&self.get_init_local_data_file_path()).await {
            Err(e) => Err(Box::new(
                GetLinkPartsFromLocalJsonFileErrorEnum::TokioFsFileOpen {
                    source: e,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            )),
            Ok(mut file) => {
                let mut content = Vec::new();
                if let Err(e) = tokio::io::AsyncReadExt::read_to_end(&mut file, &mut content).await
                {
                    return Err(Box::new(
                        GetLinkPartsFromLocalJsonFileErrorEnum::TokioIoAsyncReadExtReadToEnd {
                            source: e,
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    ));
                }
                match serde_json::from_slice::<ProvidersInitJsonSchema>(&content) {
                    Err(e) => Err(Box::new(
                        GetLinkPartsFromLocalJsonFileErrorEnum::SerdeJsonFromSlice {
                            source: e,
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    )),
                    Ok(file_content_as_struct) => Ok(file_content_as_struct.data),
                }
            }
        }
    }
}
