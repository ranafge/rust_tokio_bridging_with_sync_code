

// #[tokio::main] // start at run time project run in main asyncronously.
// async fn main() {
//     println!("Hello, world!");
// }


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

 use std::sync::mpsc;

// Other approch to imopl the above code
 use tokio::runtime::Builder;
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

 fn main() {
     let runtime = Builder::new_multi_thread()
     .worker_threads(1)
     .enable_all()
     .build()
     .unwrap();

    let mut handles = Vec::with_capacity(10);

    for i in 0..10 {
        // spwan return a joinhandle that is a future store in vector handles
        handles.push(runtime.spawn(my_bg_task(i)));
    }

    std::thread::sleep(Duration::from_millis(10));
    println!("Finished time-consuming task.");

    for handle in handles {
        // block_on wait for all handle to complete
        runtime.block_on(handle).unwrap();
    }
 }

 async fn my_bg_task(i: u64) {
    let miliis = 1000 - 50 * i;
    println!("Task {} sleeping for {} ms", i, miliis);
    sleep(Duration::from_millis(miliis)).await;
    println!("Task {} stopping.", i);

 }


 
 pub struct Task {
    name: String
 }

 async fn handle_task(task: Task) {
    println!("Got task {}", task.name)
 }

 #[derive(Clone)]
 pub struct TaskSpawner {
    spawn: mpsc::Sender<Task>
 }

 impl TaskSpawner {
     pub fn new() -> TaskSpawner {
        let (send, mut recv) = mpsc::channel();

        let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
        
        std::thread::spawn(move || {
            rt.block_on( async move {
                while let Some(task) = recv.recv().await {
                    tokio::spawn(handle_task(task));
                }
            })
        });
        TaskSpawner {
            spwan: send
        }
     }

     pub fn spawn_task(&self, task: Task) {
        match self.spawn.blocking_send(task) {
            Ok(()) => {},
            Err(_) => panic("The shard runtime has shutdown.")
        }
     }
     todo!();
 }