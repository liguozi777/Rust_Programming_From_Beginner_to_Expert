use std::fmt::Display;
struct Foo<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}
impl<'a, 'b> Foo<'a, 'b> {
    fn get_x(&self) -> &i32 {
        self.x
    }
    fn get_y(&self) -> &i32 {
        self.y
    }
    fn max_x(&'a self, f: &'a Foo) -> &'a i32 {
        if self.x > f.x {
            self.x
        } else {
            f.x
        }
    }
}
fn main() {
    // 6-17
    let s = String::from("Hello, Rust!");
    println!("{}", &s[0..5]);
    println!("{}", &s[..5]);
    println!("{}", &s[7..s.len()]);
    println!("{}", &s[7..]);
    println!("{}", &s[0..s.len()]);
    println!("{}", &s[..]);

    let vec = vec![1, 2, 3, 4, 5];
    println!("{:?}", &vec[0..2]);
    println!("{:?}", &vec[..2]);
    println!("{:?}", &vec[2..vec.len()]);
    println!("{:?}", &vec[2..]);
    println!("{:?}", &vec[0..vec.len()]);
    println!("{:?}", &vec[..]);

    // 6-18
    let s = String::from("Hello, Rust!");
    let str = "Hello";
    let vec = vec![1, 2, 3, 4, 5];
    print_str(&s[0..5]);
    print_str(&str);
    print_vec(&vec[2..]);

    // 6-19
    let mut vec = vec![1, 2, 3, 4, 5];
    let vec_slice = &mut vec[3..];
    vec_slice[0] = 7;
    println!("{:?}", vec);

    // 迭代器
    // 6-20
    let vec = vec!["Java", "Rust", "Python"];
    for str in vec.into_iter() {
        match str {
            "Rust" => println!("Niubility"),
            _ => println!("{}", str),
        }
    }
    // println!("{:?}", vec); // error: value borrowed here after move
    // 6-21
    let vec = vec!["Java", "Rust", "Python"];
    for str in vec.iter() {
        match str {
            &"Rust" => println!("Niubility"),
            _ => println!("{}", str),
        }
    }
    println!("{:?}", vec);

    // 6-22
    let mut vec = vec!["Java", "Rust", "Python"];
    for str in vec.iter_mut() {
        match str {
            &mut "Rust" => {
                *str = "Niubility";
                println!("{}", str);
            }
            _ => println!("{}", str),
        }
    }
    println!("{:?}", vec);

    // 6-23
    let r;
    {
        let i = 7;
        r = &i;
    }
    // println!("r:{}", r);

    // 6-24
    let r; // 'a
    {
        let i = 7; // 'b
        r = &i;
    }
    // println!("r:{}", r);

    // 6-25
    let i = 7;
    let r = &i;
    println!("r:{}", r);

    // 6-26
    let str1 = String::from("abcd");
    let str2 = "xyz";

    let result = long_str(str1.as_str(), str2);
    println!("longer string:{}", result);

    // 6-27
    let str = String::from("abcd");
    let result;
    {
        let str2 = String::from("xyz");
        result = long_str(str1.as_str(), str2.as_str());
    }
    // println!("longer string:{}", result)

    // 6-28
    let str1 = String::from("abcd");
    let result;
    {
        let str2 = "xyz";
        result = long_str(str1.as_str(), str2);
    }
    println!("longer string:{}", result);

    // 6-30
    let f1 = Foo { x: &3, y: &5 };
    let f2 = Foo { x: &7, y: &9 };
    println!("x:{},y:{}", f1.get_x(), f1.get_y());
    println!("max_x:{}", f1.max_x(&f2));
}
// 6-18
fn print_str(s: &str) {
    println!("slice:{:?},length:{}", s, s.len());
}
fn print_vec(vec: &[i32]) {
    println!("slice:{:?},length:{}", vec, vec.len());
}
// 6-26
fn long_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 6-29
fn long_str_with_tip<'a, T>(x: &'a str, y: &'a str, tip: T) -> &'a str
where
    T: Display,
{
    println!("Tip:{}", tip);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
