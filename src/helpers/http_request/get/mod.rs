pub mod headers;
pub mod headers_mut;
pub mod status;
// pub mod cookies;
pub mod bytes;
pub mod bytes_stream;
pub mod chunk;
pub mod content_length;
pub mod copy_to; //sync only
pub mod error_for_status;
pub mod extensions;
pub mod extensions_mut;
pub mod json;
pub mod remote_addr;
pub mod text;
pub mod text_with_charset;
pub mod url;
pub mod version; //async only //async only
                 // pub mod upgrade;//async only
                 // pub mod error_for_status_ref;//dont need it i think
