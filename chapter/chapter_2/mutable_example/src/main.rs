fn main() {
    // 不可变变量
    // let x = 3;
    // x = 5;
    // println!("x: {}", x);
    // cannot assign twice to immutable variable
    // 不能对不可变变量赋值两次

    // 变量的可变性声明
    let mut x = 3;
    x = 5;
    println!("x: {}", x);
    // x: 5
}
