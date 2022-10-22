fn main() {
    tufa_server::entry::entry();
}
// fn main() {
//     if let Err(e) = something() {
//         match e {
//             SomethingError::One { source, location } => println!("{} {}", source, location),
//             SomethingError::Two { source, location } => println!("{} {}", source, location),
//         }
//     }
// }
// #[track_caller]
// fn location() -> core::panic::Location<'static> {
//     *core::panic::Location::caller()
// }
// fn something() -> Result<(), SomethingError> {
//     one()?; //хотелось бы как-то получать location тут, но все еще получаю имплементацию в трейте src/main.rs:37:23
//     if let Err(e) = two() {
//         return Err(SomethingError::Two {
//             source: e,
//             location: location(), //без имплементации трейта(если закомментирую "one()?;"), при явной инициализации src/main.rs:18:23
//         });
//     }
//     Ok(())
// }
// enum SomethingError {
//     One {
//         source: bool,
//         location: core::panic::Location<'static>,
//     },
//     Two {
//         source: u32,
//         location: core::panic::Location<'static>,
//     },
// }
// impl From<bool> for SomethingError {
//     fn from(e: bool) -> Self {
//         SomethingError::One {
//             source: e,
//             location: location(),
//         }
//     }
// }
// fn one() -> Result<(), bool> {
//     Err(true)
// }
// fn two() -> Result<(), u32> {
//     Err(32)
// }
