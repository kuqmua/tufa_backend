use std::fmt;
use std::path::Path;
use std::{fs::File, io::Write};

use tokio::io::AsyncWriteExt;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn write_string_into_file(path: &Path, stringified_json: String) -> Result<(), std::io::Error> {
    if let Some(prefix) = path.parent() {
        std::fs::create_dir_all(prefix)?;
    }
    let mut log_file = File::create(path)?;
    log_file.write_all(stringified_json.as_bytes())?;
    log_file.sync_all()?;
    Ok(())
}

//
#[derive(thiserror::Error, Debug, ImplDisplayDerive)]
pub struct WriteStringIntoFileWithTokioError {
    pub source: Box<WriteStringIntoFileWithTokioErrorEnum>,
}

#[derive(thiserror::Error, Debug, ImplDisplayDerive)] //, ImplFromForUpperStruct
pub enum WriteStringIntoFileWithTokioErrorEnum {
    StdFsCreateDirAll(StdFsCreateDirAllStruct),
    TokioFsFileOpen(TokioFsFileOpenStruct),
    FileWriteAll(FileWriteAllStruct),
}

#[derive(Debug)]
pub struct StdFsCreateDirAllStruct {
    pub source: std::io::Error,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug)]
pub struct TokioFsFileOpenStruct {
    pub source: std::io::Error,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug)]
pub struct FileWriteAllStruct {
    pub source: std::io::Error,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

pub async fn write_string_into_file_with_tokio(
    path: &Path,
    stringified_json: String,
) -> Result<(), WriteStringIntoFileWithTokioError> {
    if let Some(prefix) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(prefix) {
            return Err(WriteStringIntoFileWithTokioError {
                source: Box::new(WriteStringIntoFileWithTokioErrorEnum::StdFsCreateDirAll(
                    StdFsCreateDirAllStruct {
                        source: e,
                        file: file!(),
                        line: line!(),
                        column: column!(),
                    },
                )),
            });
        }
    }
    match tokio::fs::File::open(path).await {
        Err(e) => Err(WriteStringIntoFileWithTokioError {
            source: Box::new(WriteStringIntoFileWithTokioErrorEnum::TokioFsFileOpen(
                TokioFsFileOpenStruct {
                    source: e,
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            )),
        }),
        Ok(mut file) => {
            if let Err(e) = file.write_all(stringified_json.as_bytes()).await {
                return Err(WriteStringIntoFileWithTokioError {
                    source: Box::new(WriteStringIntoFileWithTokioErrorEnum::FileWriteAll(
                        FileWriteAllStruct {
                            source: e,
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    )),
                });
            }
            Ok(())
        }
    }
}
