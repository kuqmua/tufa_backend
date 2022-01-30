use crate::providers::provider_kind_enum::ProviderKind;
use crate::providers::providers_info::providers_init_json_schema::ProvidersInitJsonSchema;
use crate::traits::provider_kind_trait::ProviderKindTrait;

#[derive(Debug)]
pub enum GetLinkPartsFromLocalJsonFileErrorEnum {
    TokioFsFileOpen {
        source: std::io::Error,
        line: String,
    },
    TokioIoAsyncReadExtReadToEnd {
        source: std::io::Error,
        line: String,
    },
    SerdeJsonFromSlice {
        source: serde_json::Error,
        line: String,
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
                    line: format!("{}:{}:{}", file!(), line!(), column!()),
                },
            )),
            Ok(mut file) => {
                let mut content = Vec::new();
                if let Err(e) = tokio::io::AsyncReadExt::read_to_end(&mut file, &mut content).await
                {
                    return Err(Box::new(
                        GetLinkPartsFromLocalJsonFileErrorEnum::TokioIoAsyncReadExtReadToEnd {
                            source: e,
                            line: format!("{}:{}:{}", file!(), line!(), column!()),
                        },
                    ));
                }
                match serde_json::from_slice::<ProvidersInitJsonSchema>(&content) {
                    Err(e) => Err(Box::new(
                        GetLinkPartsFromLocalJsonFileErrorEnum::SerdeJsonFromSlice {
                            source: e,
                            line: format!("{}:{}:{}", file!(), line!(), column!()),
                        },
                    )),
                    Ok(file_content_as_struct) => Ok(file_content_as_struct.data),
                }
            }
        }
    }
}
