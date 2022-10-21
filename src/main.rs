fn main() {
    tufa_server::entry::entry();
    println!("{:?}", location());
}

#[track_caller]
fn location() -> core::panic::Location<'static> {
    *core::panic::Location::caller()
}

// fn main() {
//     println!("{:?}", location());
//     println!("{:?}", location());
// }
