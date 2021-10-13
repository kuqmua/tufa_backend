
use serde::de;
use serde::ser;
use std::error::Error;
use std::fmt;

#[derive(Debug)] 
pub enum ConfigError {
    Frozen,
    Message(String),
    //for all env variable in .env realize type
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::Frozen  => write!(f, "{}", self.to_string()),
            ConfigError::Message(ref s) => write!(f, "{}", s),
        }
    }
}

impl Error for ConfigError {
    // is it deprecated?
    // fn description(&self) -> &str {
    //     match *self {
    //         ConfigError::Frozen => "configuration is frozen",
    //         _ => "configuration error",
    //     }
    // }
    //dont know about it yet
    // fn cause(&self) -> Option<&Error> {
    //     match *self {
    //         ConfigError::Foreign(ref cause) | ConfigError::FileParse { ref cause, .. } => {
    //             Some(cause.as_ref())
    //         }
    //         ConfigError::Frozen => "configuration is frozen",
    //         _ => None,
    //     }
    // }
}

impl de::Error for ConfigError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        ConfigError::Message(msg.to_string())
    }
}

impl ser::Error for ConfigError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        ConfigError::Message(msg.to_string())
    }
}

    