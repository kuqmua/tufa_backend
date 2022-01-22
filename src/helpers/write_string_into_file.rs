use std::path::Path;
use std::{fs::File, io::Write};
use std::fmt;

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
    source: std::io::Error,
}

#[derive(Debug)]
pub struct TokioFsFileOpenStruct {
    source: std::io::Error,
}

#[derive(Debug)]
pub struct FileWriteAllStruct {
    source: std::io::Error,
}

//
///
pub async fn write_string_into_file_with_tokio(path: &Path, stringified_json: String) -> Result<(), WriteStringIntoFileWithTokioError> {
    if let Some(prefix) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(prefix) {
            return Err(WriteStringIntoFileWithTokioError {
                source: Box::new(WriteStringIntoFileWithTokioErrorEnum::StdFsCreateDirAll(
                    StdFsCreateDirAllStruct { source: e },
                )),
            });
        }
    }
    match tokio::fs::File::open(path).await {
        Err(e) => Err(WriteStringIntoFileWithTokioError {
            source: Box::new(WriteStringIntoFileWithTokioErrorEnum::TokioFsFileOpen(
                TokioFsFileOpenStruct { source: e },
            )),
        }),
        Ok(mut file) => {
            if let Err(e) = file.write_all(stringified_json.as_bytes()).await {
                return Err(WriteStringIntoFileWithTokioError {
                    source: Box::new(
                        WriteStringIntoFileWithTokioErrorEnum::FileWriteAll(
                            FileWriteAllStruct {
                                source: e,
                            },
                        ),
                    ),
                });
            }
            Ok(())
        }
    }
}