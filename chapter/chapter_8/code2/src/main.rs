use std::thread;
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("{:?}", v);
    });
    handle.join().unwrap();
    // println!("{:?}", v); // error[E0382]: borrow of moved value: `v`
}
