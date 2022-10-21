fn main() {
    tufa_server::entry::entry();
    let l = location();

    let g = wrapper();
}

#[track_caller]
fn location() -> core::panic::Location<'static> {
    *core::panic::Location::caller()
}

fn wrapper() -> Result<(), FF> {
    one()?;
    two()?;
    Ok(())
}

enum FF {
    One(bool),
    Two(u32),
}

impl From<bool> for FF {
    fn from(e: bool) -> Self {
        println!("1{:#?}", location());
        FF::One(e)
    }
}

impl From<u32> for FF {
    fn from(e: u32) -> Self {
        println!("2{:#?}", location());
        FF::Two(e)
    }
}

struct Two {}

fn one() -> Result<(), bool> {
    Err(true)
}

fn two() -> Result<(), u32> {
    Err(32)
}
