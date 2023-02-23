fn main() {
    // tufa_server::entry::entry();
    tufa_server::dev::dev();
    println!("-----------");
    tufa_server::dev::dev_with_deserialize();
}
