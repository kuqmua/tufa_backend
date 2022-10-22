// use chrono::DateTime;
// use chrono::FixedOffset;
// use chrono::Local;
// use chrono::Utc;

// #[derive(Debug, Clone)]
// pub struct WhereWass {
//     pub time: DateTime<Utc>,
//     pub location: core::panic::Location<'static>,
// }
// // 1666425934
// // 1651227581881
fn main() {
    // let now = Utc::now();
    // let ts: i64 = now.timestamp();

    // println!("ts{}ts", ts);
    // let b = std::time::SystemTime::now();
    // println!("systemtime, {:?}", b);
    // match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
    //     Ok(n) => {
    //         // n.
    //         // let f = n.as_micros();
    //         n.as_nanos();
    //         println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs());

    //         let dt = DateTime::<chrono::Utc>::from_utc(
    //             // chrono::NaiveDateTime::from_timestamp(n.as_secs().try_into().unwrap(), 0),
    //             chrono::DateTime::chrono::Utc,
    //         );
    //         println!("dt {}", dt);
    //     }
    //     Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    // }
    // let o = DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc).with_timezone(
    //     &FixedOffset::east(tufa_server::lazy_static::config::CONFIG.timezone),
    // );
    // //
    // // let x = DateTime::<Utc>::from_utc();
    // //
    // //     let f = tufa_common::where_was::WhereWas {
    // //     time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc).with_timezone(
    // //         &FixedOffset::east(tufa_server::lazy_static::config::CONFIG.timezone),
    // //     ),
    // //     location: *core::panic::Location::caller(),
    // // };
    // let f = WhereWass {
    //     // time: DateTime::<Utc>::from_utc(Local::now().naive_utc(), Utc).with_timezone(
    //     //     &FixedOffset::east(tufa_server::lazy_static::config::CONFIG.timezone),
    //     // ),
    //     time: Utc::now(),
    //     location: *core::panic::Location::caller(),
    // };
    // println!("{}", f.time);
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
