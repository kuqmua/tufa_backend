use std::collections::HashMap;
use tufa_common::common::code_occurence::CodeOccurence;
use tufa_common::common::code_occurence::TimeFileLineColumnIncrement;
use tufa_common::common::where_was::GitInfoForWhereWas;

fn main() {
    // let mapp = HashMap::<GitInfoForWhereWas, TimeFileLineColumnIncrement>::new();
    // let f = CodeOccurence {
    //     where_was_hashmap: mapp,
    // };
    // println!("f {:#?}", f);
    tufa_server::entry::entry();
}
