use std::collections::HashMap;
// 6-7
#[derive(Debug, Clone)]
struct Foo {
    x: i32,
    y: String,
}
fn main() {
    // 所有权系统：所有权Ownership,借用Borrowing,生命周期Lifetimes
    // 堆栈：代码在运行时可供使用的内容，存储数据方式不同
    // 6-1
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2 = {}", s2);
    // println!("s1 = {}", s1); // error: value borrowed here after move
    println!("-----------------");
    // 6-2
    let s = String::from("hello");
    takes_ownership(s);
    // println!("s = {}", s); // error: value borrowed here after move
    let x = 5;
    make_copy(x);
    println!("x = {}", x);
    println!("-----------------");
    // 6-3
    let key = "Favorite color";
    let value = "Red";

    let mut map = HashMap::new();
    map.insert(key, value);
    println!("{:?}", map[key]);
    println!("key = {}", key);
    println!("value = {}", value);
    println!("-----------------");
    // 6-4
    // let key = String::from("Favorite color");
    // let value = String::from("Red");

    // let mut map = HashMap::new();
    // map.insert(key, value);
    // println!("{}", map[key]);
    // 6-5
    let key = String::from("Favorite color");
    let value = String::from("Red");

    let mut map = HashMap::new();
    map.insert(&key, &value);
    println!("{:?}", map[&key]);
    println!("-----------------");
    // 6-6
    // give_ownership函数返回值所有权给s1
    let s1 = give_ownership();
    let s2 = take_and_give_back(s1);
    // println!("s1 = {}", s1);// error: value borrowed here after move
    println!("s2 = {}", s2);
    println!("-----------------");
    // 6-7/6-8
    let foo = Foo {
        x: 8,
        y: String::from("hello"),
    };
    let other = foo.clone();
    println!("foo:{:?},other:{:?}", foo, other);
    println!("-----------------");
    // 6-9
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    println!("-----------------");
    // 6-10
    // let vec1 = vec![1, 2, 3];
    // let vec2 = vec![4, 5, 6];

    // let answer = sum_vec(vec1, vec2);
    // println!("v1:{:?},v2:{:?},sum:{}", vec1, vec2, answer);
    // println!("-----------------");
    // 6-11
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let (v1, v2, answer) = sum_vec2(vec1, vec2);
    println!("v1:{:?},v2:{:?},sum:{}", v1, v2, answer);
    println!("-----------------");
    // 6-12
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let answer = sum_vec(&vec1, &vec2);
    println!("v1:{:?},v2:{:?},sum:{}", vec1, vec2, answer);
    println!("-----------------");
    // 6-13
    let mut vec = Vec::new();
    push_vec(&mut vec, 1);
    push_vec(&mut vec, 2);
    push_vec(&mut vec, 2);
    push_vec(&mut vec, 5);
    println!("vec:{:?}", vec);
    println!("-----------------");
    // 6-14
    let mut x = 6;
    let y = &mut x;
    println!("y = {}", y);
    *y += 1;
    println!("y = {}", y);
    println!("x = {}", x);
    println!("-----------------");
    // 6-15
    // let mut x = 6;
    // let y = &mut x;
    // *y += 1;
    // let z = &x;
    // println!("y:{},z:{}", *y, *z);
    // 6-16
    // let mut x = 6;
    // let y = &mut x;
    // *y += 1;
    // let z = x;
    // println!("y:{},z:{}", *y, z);
}
// 6-2
fn takes_ownership(str: String) {
    println!("{}", str);
}
fn make_copy(int: i32) {
    println!("{}", int);
}
// 6-6
fn give_ownership() -> String {
    let str = String::from("ownership");
    str
}
fn take_and_give_back(name: String) -> String {
    let hello = String::from("hello");
    hello + " " + &name
}
// 6-10
// fn sum_vec(v1: Vec<i32>, v2: Vec<i32>) -> i32 {
//     let sum1: i32 = v1.iter().sum();
//     let sum2: i32 = v2.iter().sum();

//     sum1 + sum2
// }
// 6-11
fn sum_vec2(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    let sum1: i32 = v1.iter().sum();
    let sum2: i32 = v2.iter().sum();

    (v1, v2, sum1 + sum2)
}
// 6-12
fn sum_vec(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let sum1: i32 = v1.iter().sum();
    let sum2: i32 = v2.iter().sum();

    sum1 + sum2
}
// 6-13
fn push_vec(vec: &mut Vec<i32>, value: i32) {
    vec.push(value);
}
