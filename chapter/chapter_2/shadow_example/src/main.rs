fn main() {
    // 变量遮蔽
    // 实质：通过let关键字声明一个新的变量，只是名称恰巧相同，但是完全不同的变量，值和值类型都可以不同
    let x = 3;
    let x = x + 2;
    let x = x * 2;
    println!("x: {}", x);

    let x = "Hello, Rust!";
    println!("x: {}", x);

    // x: 10
    // x: Hello, Rust!
}
