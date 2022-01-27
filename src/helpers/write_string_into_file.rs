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
    line: String
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
    line: String
}

#[derive(Debug)]
pub struct TokioFsFileOpenStruct {
    source: std::io::Error,
    line: String
}

#[derive(Debug)]
pub struct FileWriteAllStruct {
    source: std::io::Error,
    line: String
}

//
///
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
                        line: format!("{} {}", line!().to_string(), file!().to_string())
                    },
                )),
                line: format!("{} {}", line!().to_string(), file!().to_string())
            });
        }
    }
    match tokio::fs::File::open(path).await {
        Err(e) => Err(WriteStringIntoFileWithTokioError {
            source: Box::new(WriteStringIntoFileWithTokioErrorEnum::TokioFsFileOpen(
                TokioFsFileOpenStruct { 
                    source: e,
                    line: format!("{} {}", line!().to_string(), file!().to_string())
                },
            )),
            line: format!("{} {}", line!().to_string(), file!().to_string())
        }),
        Ok(mut file) => {
            if let Err(e) = file.write_all(stringified_json.as_bytes()).await {
                return Err(WriteStringIntoFileWithTokioError {
                    source: Box::new(WriteStringIntoFileWithTokioErrorEnum::FileWriteAll(
                        FileWriteAllStruct { 
                            source: e,
                            line: format!("{} {}", line!().to_string(), file!().to_string())
                        },
                    )),
                    line: format!("{} {}", line!().to_string(), file!().to_string())
                });
            }
            Ok(())
        }
    }
}
