use std::fmt;
use std::path::Path;

use tokio::io::AsyncWriteExt;
//
#[derive(thiserror::Error, Debug, ImplDisplayDerive)]
pub struct WriteStringIntoFileWithTokioError {
    pub source: Box<WriteStringIntoFileWithTokioErrorEnum>,
}

#[derive(thiserror::Error, Debug, ImplDisplayDerive)]
pub enum WriteStringIntoFileWithTokioErrorEnum {
    StdFsCreateDirAll{
        source: std::io::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
    TokioFsFileOpen {
        source: std::io::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
    FileWriteAll {
        source: std::io::Error,
        file: &'static str,
        line: u32,
        column: u32,
    },
}

pub async fn write_string_into_file_with_tokio(
    path: &Path,
    stringified_json: String,
) -> Result<(), WriteStringIntoFileWithTokioError> {
    if let Some(prefix) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(prefix) {
            return Err(WriteStringIntoFileWithTokioError {
                source: Box::new(WriteStringIntoFileWithTokioErrorEnum::StdFsCreateDirAll {
                        source: e,
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                ),
            });
        }
    }
    match tokio::fs::File::open(path).await {
        Err(e) => Err(WriteStringIntoFileWithTokioError {
            source: Box::new(WriteStringIntoFileWithTokioErrorEnum::TokioFsFileOpen {
                    source: e,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            ),
        }),
        Ok(mut file) => {
            if let Err(e) = file.write_all(stringified_json.as_bytes()).await {
                return Err(WriteStringIntoFileWithTokioError {
                    source: Box::new(WriteStringIntoFileWithTokioErrorEnum::FileWriteAll {
                            source: e,
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    ),
                });
            }
            Ok(())
        }
    }
}