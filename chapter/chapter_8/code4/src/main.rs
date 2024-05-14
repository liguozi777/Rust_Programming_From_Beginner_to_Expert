use futures::executor::block_on;
async fn hello_async(){
    println!("Hello, async!");
}
fn main() {
    let future = hello_async();
    block_on(future);
}
