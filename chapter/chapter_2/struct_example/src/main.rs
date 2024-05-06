pub struct Student {
    // pub是一个访问修饰符,表示这个结构体是公共的,可以被其他模块访问
    name: &'static str, // &'static str是一个静态字符串类型,表示这个字符串是静态的,在程序的整个生命周期中都存在,不会被释放
    score: i32,
}

fn main() {
    // struct ListNode {
    //     val: i32,// i32是32位有符号整数类型
    //     next: Option<Box<ListNode>>,// Option是一个枚举类型，Box是一个智能指针类型
    // }
    let score = 59;
    let username = "zhangsan";

    let mut student = Student {
        score,
        name: username,
    };

    student.score = 60;
    println!("name:{}, score:{}", student.name, student.score);

    let student2 = Student {
        name: "lisi",
        ..student
    };
    println!("name:{}, score:{}", student2.name, student2.score);

    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
}
