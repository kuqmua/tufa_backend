use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use tokio;

#[tokio::main]
pub async fn do_something() -> HashMap<String, String> {
    let hsmap: HashMap<String, String> = HashMap::new();
    let arcmap = Arc::new(Mutex::new(hsmap));
    let handle = {
        let arcmap = arcmap.clone();
        tokio::spawn(async move {
            arcmap
                .lock()
                .unwrap()
                .insert("something".to_string(), "something".to_string());
        });
    };
    // let handle = ;
    // handle.await.unwrapbig();
    let arcmap: HashMap<String, String> = Arc::try_unwrap(arcmap).unwrap().into_inner().unwrap();
    arcmap
}



-------
fn parent() {
    let mut vec = Vec::new();
    // fill the vector
    thread::spawn(|| {
        print_vec(&vec)
    })
}


let arcmapt = Arc::clone(&arcmap);