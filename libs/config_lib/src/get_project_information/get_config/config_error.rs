
use serde::de;
use serde::ser;
use std::error::Error;
use std::fmt;

/// Represents all possible errors that can occur when working with
/// configuration.
#[derive(Debug)] 
pub enum ConfigError {
    /// Configuration is frozen and no further mutations can be made.
    Frozen,

    /// Configuration property was not found
    NotFound(String),

    /// Configuration could not be parsed from file.
    FileParse {
        /// The URI used to access the file (if not loaded from a string).
        /// Example: `/path/to/config.json`
        uri: Option<String>,

        /// The captured error from attempting to parse the file in its desired format.
        /// This is the actual error object from the library used for the parsing.
        cause: Box<Error + Send + Sync>,
    },

    /// Custom message
    Message(String),

    /// Unadorned error from a foreign origin.
    Foreign(Box<Error + Send + Sync>),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::Frozen  => write!(f, "{}", self.description()),
            ConfigError::Message(ref s) => write!(f, "{}", s),
            ConfigError::Foreign(ref cause) => write!(f, "{}", cause),
            ConfigError::NotFound(ref key) => {
                write!(f, "configuration property {:?} not found", key)
            }

            ConfigError::FileParse { ref cause, ref uri } => {
                write!(f, "{}", cause)?;

                if let Some(ref uri) = *uri {
                    write!(f, " in {}", uri)?;
                }

                Ok(())
            }
        }
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::Frozen => "configuration is frozen",
            ConfigError::NotFound(_) => "configuration property not found",
            ConfigError::Foreign(ref cause) | ConfigError::FileParse { ref cause, .. } => {
                cause.description()
            }
            _ => "configuration error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ConfigError::Foreign(ref cause) | ConfigError::FileParse { ref cause, .. } => {
                Some(cause.as_ref())
            }

            _ => None,
        }
    }
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

    