//import the necessary modules
/*
   连接句柄
   生成的线程可能没有机会运行或完全运行。这是因为主线程完成得很快。
   函数spawn<F, T>(f: F) -> JoinHandlelt;T>
   返回一个 JoinHandle。JoinHandle 上的join()方法等待关联的线程完成。
*/
use std::thread;
use std::time::Duration;
fn main() {
    //create a new thread
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    //code executed by the main thread
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(2000));
    }
}
