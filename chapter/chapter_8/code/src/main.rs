use std::thread; // 引入线程库
use std::time::Duration; // 引入时间库
fn main() {
    // 创建第一个子线程
    let thread_1 = thread::spawn(|| {
        for i in 1..=5 {
            println!("number {} from the spawned_1 thread!", i);
            thread::sleep(Duration::from_secs(2));
        }
    });
    // 创建第二个子线程
    let thread_2 = thread::spawn(|| {
        for i in 1..=5 {
            println!("number {} from the spawned_2 thread!", i);
            thread::sleep(Duration::from_secs(4));
        }
    });
    // 主线程执行的代码
    for i in 1..=5 {
        println!("number {} from the main thread!", i);
        thread::sleep(Duration::from_secs(8));
    }
    // 阻塞主线程，等待子线程执行完毕
    thread_1.join().unwrap();
    thread_2.join().unwrap();

    
}
