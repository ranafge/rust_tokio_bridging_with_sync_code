

#[tokio::main] // start at run time project run in main asyncronously.
async fn main() {
    println!("Hello, world!");
}


// async fn main turn into this below
/*

fn main() {
    tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
        println!("Hello world")
    })
}

 */