use crate::config_mods::lazy_static_config::CONFIG;
use crate::helpers::git_info::GIT_INFO;
use crate::preparation::prepare_server::prepare_server;
use crate::prints::print_colorful_message::print_colorful_message;
use crate::prints::print_type_enum::PrintType;
use crate::project_constants::PROJECT_NAME;
use crate::server_wrapper::server_wrapper;
use crate::telemetry::get_subscriber::get_subscriber;
use crate::telemetry::init_subscriber::init_subscriber;

#[cfg(tracing_unstable)]
use tracing::field::valuable;
use valuable::Valuable;

#[derive(Clone, Debug, Valuable)]
struct User {
    name: String,
    age: u32,
    something: Vec<bool>,
    address: Address,
}

#[derive(Clone, Debug, Valuable)]
struct Address {
    country: String,
    city: String,
    street: String,
}
// use valuable::{NamedValues, Valuable, Value, Visit};

// #[derive(Valuable)]
// struct HelloWorld {
//     message: Message,
// }

// #[derive(Valuable)]
// enum Message {
//     HelloWorld,
//     Custom(String),
// }

// struct Print;

// impl Visit for Print {
//     fn visit_value(&mut self, value: Value<'_>) {
//         match value {
//             Value::Structable(v) => {
//                 println!("struct {}", v.definition().name());
//                 v.visit(self);
//             }
//             Value::Enumerable(v) => {
//                 println!("enum {}::{}", v.definition().name(), v.variant().name());
//                 v.visit(self);
//             }
//             Value::Listable(v) => {
//                 println!("list");
//                 v.visit(self);
//             }
//             Value::Mappable(v) => {
//                 println!("map");
//                 v.visit(self);
//             }
//             _ => {
//                 println!("value {:?}", value);
//             }
//         }
//     }

//     fn visit_named_fields(&mut self, named_fields: &NamedValues<'_>) {
//         for (field, value) in named_fields.iter() {
//             println!("named field {}", field.name());
//             value.visit(self);
//         }
//     }

//     fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
//         for value in values {
//             value.visit(self);
//         }
//     }

//     fn visit_entry(&mut self, key: Value<'_>, value: Value<'_>) {
//         println!("key / value");
//         key.visit(self);
//         value.visit(self);
//     }
// }
// #[derive(Debug, Valuable)]
// pub struct Something {
//     source: Vec<bool>,
// }

// #[derive(Valuable)]
// enum SomethingEnum {
//     Something,
//     Custom(String),
// }

// impl Visit for Something {
//     fn visit_value(&mut self, value: Value<'_>) {
//         match value {
//             Value::Structable(v) => {
//                 // println!("struct {}", v.definition().name());
//                 v.visit(self);
//             }
//             Value::Enumerable(v) => {
//                 // println!("enum {}::{}", v.definition().name(), v.variant().name());
//                 v.visit(self);
//             }
//             Value::Listable(v) => {
//                 // println!("list");
//                 v.visit(self);
//             }
//             Value::Mappable(v) => {
//                 // println!("map");
//                 v.visit(self);
//             }
//             _ => {
//                 // println!("value {:?}", value);
//             }
//         }
//     }

//     fn visit_named_fields(&mut self, named_fields: &NamedValues<'_>) {
//         for (field, value) in named_fields.iter() {
//             // println!("named field {}", field.name());
//             value.visit(self);
//         }
//     }

//     fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
//         for value in values {
//             value.visit(self);
//         }
//     }

//     fn visit_entry(&mut self, key: Value<'_>, value: Value<'_>) {
//         println!("key / value");
//         key.visit(self);
//         value.visit(self);
//     }
// }

// // #[cfg(all(tracing_unstable, feature = "valuable"))]
// // #[cfg_attr(docsrs, doc(cfg(all(tracing_unstable, feature = "valuable"))))]
// // impl tracing::Value for Something {
// //     fn record(&self, key: &tracing::field::Field, visitor: &mut dyn Visit) {
// //         visitor.record_value(key, self.as_value())
// //     }
// // }

// // impl tracing::sealed::Sealed for Something {}

// // impl tracing::Value for Something {
// //     fn record(
// //         &self,
// //         key: &tracing::field::Field,
// //         visitor: &mut dyn tracing_subscriber::field::Visit,
// //     ) {
// //         visitor.record_str(key, self.source[0])
// //     }
// // }

// // impl Valuable for Something {
// //     // fn as_value(&self) -> Value<'_>;
// //     // fn visit(&self, visit: &mut dyn Visit);

// //     // fn visit_slice(slice: &[Self], visit: &mut dyn Visit)
// //     // where
// //     //     Self: Sized,
// // }

#[deny(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::integer_arithmetic,
    clippy::float_arithmetic
)]
pub fn entry() {
    match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .enable_all()
        .build()
    {
        Err(e) => {
            print_colorful_message(
                None,
                PrintType::Error,
                vec![format!("{}:{}:{}", file!(), line!(), column!())],
                vec![GIT_INFO.get_git_source_file_link(file!(), line!())],
                format!("Cannot build tokio runtime {e:#?}"),
            );
        }
        Ok(runtime) => {
            if let true = CONFIG.is_tracing_enabled {
                if let Err(e) = init_subscriber(get_subscriber(
                    PROJECT_NAME.into(),
                    CONFIG.tracing_type.to_lower_snake_case(),
                    std::io::stdout,
                )) {
                    print_colorful_message(
                        None,
                        PrintType::Error,
                        vec![format!("{}:{}:{}", file!(), line!(), column!())],
                        vec![GIT_INFO.get_git_source_file_link(file!(), line!())],
                        format!("tracing init_subscriber error: {:#?}", e),
                    );
                    return;
                };
            }
            // let hello_world = HelloWorld {
            //     message: Message::HelloWorld,
            // };
            // hello_world.visit(&mut Print);
            // let f = hello_world.as_value();
            // let custom_error = std::io::Error::new(std::io::ErrorKind::Other, "oh no!");
            // tracing::error!(error = custom_error);
            //as &dyn Valuable
            // // let f = Something {
            // // source: vec![true, false],
            // // };

            //
            let user = User {
                name: "Arwen Undomiel".to_string(),
                age: 3000,
                something: vec![true, false],
                address: Address {
                    country: "Middle Earth".to_string(),
                    city: "Rivendell".to_string(),
                    street: "leafy lane".to_string(),
                },
            };

            // for comparison, record `user` without using its `Valuable`
            // implementation:
            tracing::error!(valuable = false, user = ?user);

            // If the `valuable` feature is enabled, record `user` using its'
            // `valuable::Valuable` implementation:
            #[cfg(tracing_unstable)]
            tracing::error!(valuable = true, user = valuable(&user));

            // #[cfg(not(tracing_unstable))]
            // tracing::error!(
            //     "note: this example was run without `valuable` support enabled!\n\
            // rerun with `RUSTFLAGS=\"--cfg tracing_unstable\" to enable `valuable`",
            // );

            if let true = CONFIG.is_preparation_enabled {
                if runtime.block_on(prepare_server()).is_err() {
                    return;
                }
            }
            if let Err(e) = server_wrapper() {
                print_colorful_message(
                    None,
                    PrintType::Error,
                    vec![format!("{}:{}:{}", file!(), line!(), column!())],
                    vec![GIT_INFO.get_git_source_file_link(file!(), line!())],
                    format!("Cannot run actix-web HttpServer, error: {:#?}", e),
                );
            }
        }
    }
}
